<script lang="ts">
  import { onMount } from 'svelte';
  import { calcStore } from '../stores/calculation.svelte';
  import type { Jurisdiction } from '../types';

  let searchTerm = $state('');
  let filterNation = $state<'ALL' | Jurisdiction>('ALL');

  onMount(() => {
    if (calcStore.multipliers.length === 0) {
      calcStore.loadParameters();
    }
  });

  const filteredMultipliers = $derived(
    calcStore.multipliers.filter((m) => {
      if (filterNation !== 'ALL' && m.jurisdiction !== filterNation) {
        return false;
      }
      if (searchTerm.trim()) {
        const q = searchTerm.toLowerCase();
        return (
          m.financial_year.toLowerCase().includes(q) ||
          m.multiplier_type.toLowerCase().includes(q) ||
          m.parameter_id.toLowerCase().includes(q) ||
          m.multiplier_rate.toString().includes(q)
        );
      }
      return true;
    })
  );
</script>

<div class="bg-white border border-slate-200 rounded-xl shadow-sm overflow-hidden">
  <!-- Accordion Header -->
  <button
    type="button"
    onclick={() => { calcStore.isMultipliersExpanded = !calcStore.isMultipliersExpanded; }}
    class="w-full p-4 flex justify-between items-center text-left bg-slate-50/80 hover:bg-slate-50 transition border-b border-slate-200"
  >
    <div class="flex items-center space-x-2">
      <span class="text-sm font-bold text-slate-900">{calcStore.t.multipliers_title}</span>
      <span class="text-[10px] bg-blue-100 text-blue-800 font-bold px-2 py-0.5 rounded">
        {calcStore.multipliers.length} {calcStore.t.multipliers_seeded_count}
      </span>
    </div>
    <span class="text-xs text-slate-500 font-medium">
      {calcStore.isMultipliersExpanded ? calcStore.t.collapse_btn : calcStore.t.expand_btn}
    </span>
  </button>

  {#if calcStore.isMultipliersExpanded}
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
          type="text"
          placeholder={calcStore.t.multipliers_search_ph}
          bind:value={searchTerm}
          class="w-full sm:w-64 px-3 py-1.5 text-xs border border-slate-300 rounded focus:ring-1 focus:ring-blue-500 font-medium text-slate-800 bg-white"
        />
      </div>

      <!-- Multipliers Table -->
      <div class="overflow-x-auto border border-slate-200 rounded-lg">
        <table class="w-full text-left text-xs">
          <thead class="bg-slate-50 text-slate-700 border-b border-slate-200">
            <tr>
              <th class="p-2.5 font-semibold">{calcStore.t.multipliers_col_fy}</th>
              <th class="p-2.5 font-semibold">{calcStore.t.multipliers_col_nation}</th>
              <th class="p-2.5 font-semibold">{calcStore.t.multipliers_col_type}</th>
              <th class="p-2.5 font-semibold">{calcStore.t.multipliers_col_threshold}</th>
              <th class="p-2.5 font-semibold">{calcStore.t.multipliers_col_rate}</th>
              <th class="p-2.5 font-semibold">{calcStore.t.multipliers_col_period}</th>
            </tr>
          </thead>
          <tbody class="divide-y divide-slate-100">
            {#each filteredMultipliers as item}
              <tr class="hover:bg-slate-50/50">
                <td class="p-2.5 font-mono font-bold text-slate-900">{item.financial_year}</td>
                <td class="p-2.5">
                  <span class="px-2 py-0.5 rounded text-[10px] font-semibold {item.jurisdiction === 'WALES' ? 'bg-emerald-100 text-emerald-800' : item.jurisdiction === 'SCOTLAND' ? 'bg-purple-100 text-purple-800' : 'bg-blue-100 text-blue-800'}">
                    {item.jurisdiction}
                  </span>
                </td>
                <td class="p-2.5 font-medium text-slate-800">{item.multiplier_type}</td>
                <td class="p-2.5 text-slate-600 font-mono text-[11px]">
                  {#if item.rv_threshold_max != null}
                    £{Number(item.rv_threshold_min).toLocaleString()} – £{Number(item.rv_threshold_max).toLocaleString()}
                  {:else if Number(item.rv_threshold_min) > 0}
                    ≥ £{Number(item.rv_threshold_min).toLocaleString()}
                  {:else}
                    {calcStore.t.multipliers_all_rv}
                  {/if}
                </td>
                <td class="p-2.5 font-mono font-bold text-blue-700 text-sm">
                  {Number(item.multiplier_rate).toFixed(4)}
                </td>
                <td class="p-2.5 text-slate-500 font-mono text-[11px]">
                  {item.effective_from} {calcStore.t.to_word} {item.effective_to}
                </td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    </div>
  {/if}
</div>

