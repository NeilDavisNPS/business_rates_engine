<script lang="ts">
  import { onMount } from 'svelte';
  import { calcStore } from '../stores/calculation.svelte';
  import type { Jurisdiction } from '../types';

  let searchTerm = $state('');
  let filterNation = $state<'ALL' | Jurisdiction>('ALL');

  onMount(() => {
    if (calcStore.emptyPropertyThresholds.length === 0) {
      calcStore.loadParameters();
    }
  });

  const filteredThresholds = $derived(
    calcStore.emptyPropertyThresholds.filter((item) => {
      if (filterNation !== 'ALL' && item.jurisdiction !== filterNation) {
        return false;
      }
      if (searchTerm.trim()) {
        const q = searchTerm.toLowerCase();
        return (
          item.parameter_id.toLowerCase().includes(q) ||
          item.threshold_rv.toString().includes(q) ||
          item.statutory_authority.toLowerCase().includes(q) ||
          item.effective_from.toLowerCase().includes(q) ||
          item.effective_to.toLowerCase().includes(q)
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
    onclick={() => { calcStore.isEmptyPropertyExpanded = !calcStore.isEmptyPropertyExpanded; }}
    class="w-full p-4 flex justify-between items-center text-left bg-slate-50/80 hover:bg-slate-50 transition border-b border-slate-200"
    aria-expanded={calcStore.isEmptyPropertyExpanded}
  >
    <div class="flex items-center space-x-2">
      <span class="text-sm font-bold text-slate-900">{calcStore.t.ept_catalog_title}</span>
      <span class="text-[10px] bg-sky-100 text-sky-900 border border-sky-300 font-bold px-2 py-0.5 rounded">
        {calcStore.emptyPropertyThresholds.length} {calcStore.t.ept_seeded_count}
      </span>
    </div>
    <span class="text-xs text-slate-500 font-medium">
      {calcStore.isEmptyPropertyExpanded ? calcStore.t.collapse_btn : calcStore.t.expand_btn}
    </span>
  </button>

  {#if calcStore.isEmptyPropertyExpanded}
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
          placeholder={calcStore.t.ept_search_ph}
          bind:value={searchTerm}
          class="w-full sm:w-64 px-3 py-1.5 text-xs border border-slate-300 rounded focus:ring-1 focus:ring-blue-500 font-medium text-slate-800 bg-white"
        />
      </div>

      <!-- Empty Property Thresholds Table -->
      <div class="overflow-x-auto border border-slate-200 rounded-lg">
        <table class="w-full text-left text-xs">
          <thead class="bg-slate-50 text-slate-700 border-b border-slate-200">
            <tr>
              <th class="p-2.5 font-semibold">{calcStore.t.ept_col_nation}</th>
              <th class="p-2.5 font-semibold">{calcStore.t.ept_col_threshold}</th>
              <th class="p-2.5 font-semibold">{calcStore.t.ept_col_statute}</th>
              <th class="p-2.5 font-semibold">{calcStore.t.ept_col_period}</th>
            </tr>
          </thead>
          <tbody class="divide-y divide-slate-100">
            {#each filteredThresholds as item}
              <tr class="hover:bg-slate-50/50">
                <td class="p-2.5">
                  <span class="px-2 py-0.5 rounded text-[10px] font-semibold {item.jurisdiction === 'WALES' ? 'bg-emerald-100 text-emerald-900 border border-emerald-300' : item.jurisdiction === 'SCOTLAND' ? 'bg-purple-100 text-purple-900 border border-purple-300' : 'bg-blue-100 text-blue-900 border border-blue-300'}">
                    {item.jurisdiction}
                  </span>
                </td>
                <td class="p-2.5 font-mono font-bold text-amber-700 text-sm">
                  £{Number(item.threshold_rv).toLocaleString('en-GB', { minimumFractionDigits: 2, maximumFractionDigits: 2 })}
                </td>
                <td class="p-2.5 text-slate-600 text-[11px] font-medium">
                  {item.statutory_authority}
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
