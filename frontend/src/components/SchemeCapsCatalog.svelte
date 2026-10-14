<script lang="ts">
  import { onMount } from 'svelte';
  import { calcStore } from '../stores/calculation.svelte';
  import type { Jurisdiction } from '../types';

  let searchTerm = $state('');
  let filterNation = $state<'ALL' | Jurisdiction>('ALL');

  onMount(() => {
    if (calcStore.schemeCaps.length === 0) {
      calcStore.loadParameters();
    }
  });

  const filteredCaps = $derived(
    calcStore.schemeCaps.filter((item) => {
      if (filterNation !== 'ALL') {
        if (item.jurisdiction && item.jurisdiction !== filterNation) {
          return false;
        }
      }
      if (searchTerm.trim()) {
        const q = searchTerm.toLowerCase();
        return (
          item.parameter_id.toLowerCase().includes(q) ||
          item.scheme_code.toLowerCase().includes(q) ||
          (item.financial_year && item.financial_year.toLowerCase().includes(q)) ||
          item.cap_unit.toLowerCase().includes(q) ||
          item.cap_value.toString().includes(q) ||
          item.statutory_authority.toLowerCase().includes(q)
        );
      }
      return true;
    })
  );

  function formatPrimaryCap(item: { cap_value: number; cap_unit: string }): string {
    const val = Number(item.cap_value);
    switch (item.cap_unit) {
      case 'GBP_RV':
      case 'GBP_PER_YEAR':
      case 'FIXED_CASH':
        return `£${val.toLocaleString('en-GB', { minimumFractionDigits: 2, maximumFractionDigits: 2 })}`;
      case 'DAYS':
        return `${val} days`;
      case 'PERCENTAGE':
      case 'PERCENTAGE_INCREASE':
        return `${val.toFixed(1)}%`;
      case 'MULTIPLIER_ADDITION':
        return `+${val.toFixed(3)}`;
      case 'GBP_AND_RATE':
        return `£${val.toLocaleString('en-GB')}`;
      default:
        return val.toString();
    }
  }

  function formatSecondaryCap(item: { secondary_cap_value: number | null; cap_unit: string }): string {
    if (item.secondary_cap_value == null) {
      return calcStore.t.scheme_none;
    }
    const sec = Number(item.secondary_cap_value);
    if (item.cap_unit === 'GBP_AND_RATE') {
      return `+ ${(sec * 100).toFixed(0)}% RV`;
    }
    return `${sec.toFixed(0)}%`;
  }
</script>

<div class="bg-white border border-slate-200 rounded-xl shadow-sm overflow-hidden">
  <!-- Accordion Header -->
  <button
    type="button"
    onclick={() => { calcStore.isSchemeCapsExpanded = !calcStore.isSchemeCapsExpanded; }}
    class="w-full p-4 flex justify-between items-center text-left bg-slate-50/80 hover:bg-slate-50 transition border-b border-slate-200"
    aria-expanded={calcStore.isSchemeCapsExpanded}
  >
    <div class="flex items-center space-x-2">
      <span class="text-sm font-bold text-slate-900">{calcStore.t.scheme_caps_title}</span>
      <span class="text-[10px] bg-rose-100 text-rose-900 border border-rose-300 font-bold px-2 py-0.5 rounded">
        {calcStore.schemeCaps.length} {calcStore.t.scheme_caps_count}
      </span>
    </div>
    <span class="text-xs text-slate-500 font-medium">
      {calcStore.isSchemeCapsExpanded ? calcStore.t.collapse_btn : calcStore.t.expand_btn}
    </span>
  </button>

  {#if calcStore.isSchemeCapsExpanded}
    <div class="p-5 space-y-4">
      <!-- Filter Bar -->
      <div class="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-3">
        <div class="flex items-center space-x-1.5 text-xs">
          <span class="text-slate-500 font-semibold mr-1">{calcStore.t.filter_codes_label}</span>
          <button
            type="button"
            onclick={() => { filterNation = 'ALL'; }}
            class="px-2.5 py-1 rounded border text-xs font-semibold transition {filterNation === 'ALL'
              ? 'bg-blue-600 text-white border-blue-600 shadow-sm'
              : 'bg-white text-slate-700 border-slate-200 hover:bg-slate-50'}"
          >
            {calcStore.t.dict_btn_all}
          </button>
          <button
            type="button"
            onclick={() => { filterNation = 'ENGLAND'; }}
            class="px-2.5 py-1 rounded border text-xs font-semibold transition {filterNation === 'ENGLAND'
              ? 'bg-blue-600 text-white border-blue-600 shadow-sm'
              : 'bg-white text-slate-700 border-slate-200 hover:bg-slate-50'}"
          >
            {calcStore.t.dict_btn_england}
          </button>
          <button
            type="button"
            onclick={() => { filterNation = 'WALES'; }}
            class="px-2.5 py-1 rounded border text-xs font-semibold transition {filterNation === 'WALES'
              ? 'bg-blue-600 text-white border-blue-600 shadow-sm'
              : 'bg-white text-slate-700 border-slate-200 hover:bg-slate-50'}"
          >
            {calcStore.t.dict_btn_wales}
          </button>
          <button
            type="button"
            onclick={() => { filterNation = 'SCOTLAND'; }}
            class="px-2.5 py-1 rounded border text-xs font-semibold transition {filterNation === 'SCOTLAND'
              ? 'bg-blue-600 text-white border-blue-600 shadow-sm'
              : 'bg-white text-slate-700 border-slate-200 hover:bg-slate-50'}"
          >
            {calcStore.t.dict_btn_scotland}
          </button>
        </div>

        <input
          id="schemeCapsSearchInput"
          name="schemeCapsSearch"
          type="text"
          placeholder={calcStore.t.scheme_caps_search_ph}
          autocomplete="off"
          aria-label={calcStore.t.scheme_caps_search_ph}
          bind:value={searchTerm}
          class="w-full sm:w-64 px-3 py-1.5 text-xs border border-slate-300 rounded focus:ring-1 focus:ring-blue-500 font-medium text-slate-800 bg-white"
        />
      </div>

      <!-- Scheme Caps Table -->
      <div class="overflow-x-auto border border-slate-200 rounded-lg">
        <table class="w-full text-left text-xs">
          <thead class="bg-slate-50 text-slate-700 border-b border-slate-200">
            <tr>
              <th class="p-2.5 font-semibold">{calcStore.t.scheme_col_code}</th>
              <th class="p-2.5 font-semibold">{calcStore.t.scheme_col_fy}</th>
              <th class="p-2.5 font-semibold">{calcStore.t.scheme_col_nation}</th>
              <th class="p-2.5 font-semibold">{calcStore.t.scheme_col_primary_cap}</th>
              <th class="p-2.5 font-semibold">{calcStore.t.scheme_col_secondary_cap}</th>
              <th class="p-2.5 font-semibold">{calcStore.t.scheme_col_unit}</th>
              <th class="p-2.5 font-semibold">{calcStore.t.scheme_col_statute}</th>
              <th class="p-2.5 font-semibold">{calcStore.t.scheme_col_period}</th>
            </tr>
          </thead>
          <tbody class="divide-y divide-slate-100">
            {#each filteredCaps as item}
              <tr class="hover:bg-slate-50/50">
                <td class="p-2.5 font-mono font-bold text-slate-900">{item.scheme_code}</td>
                <td class="p-2.5 font-mono text-slate-700 font-medium">
                  {item.financial_year || calcStore.t.scheme_all_years}
                </td>
                <td class="p-2.5">
                  {#if item.jurisdiction}
                    <span class="px-2 py-0.5 rounded text-[10px] font-semibold {item.jurisdiction === 'WALES' ? 'bg-emerald-100 text-emerald-900 border border-emerald-300' : item.jurisdiction === 'SCOTLAND' ? 'bg-purple-100 text-purple-900 border border-purple-300' : 'bg-blue-100 text-blue-900 border border-blue-300'}">
                      {item.jurisdiction}
                    </span>
                  {:else}
                    <span class="px-2 py-0.5 rounded text-[10px] font-semibold bg-slate-100 text-slate-800 border border-slate-300">
                      GB
                    </span>
                  {/if}
                </td>
                <td class="p-2.5 font-mono font-bold text-rose-800 text-sm">
                  {formatPrimaryCap(item)}
                </td>
                <td class="p-2.5 font-mono font-medium text-slate-800">
                  {formatSecondaryCap(item)}
                </td>
                <td class="p-2.5 font-mono text-[11px] text-slate-500">
                  {item.cap_unit}
                </td>
                <td class="p-2.5 text-slate-600 text-[11px] font-medium">
                  {item.statutory_authority}
                </td>
                <td class="p-2.5 text-slate-500 font-mono text-[11px]">
                  {#if item.effective_from && item.effective_to}
                    {item.effective_from} {calcStore.t.to_word} {item.effective_to}
                  {:else}
                    <span class="text-slate-400">Statutory Scheme Standard</span>
                  {/if}
                </td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    </div>
  {/if}
</div>
