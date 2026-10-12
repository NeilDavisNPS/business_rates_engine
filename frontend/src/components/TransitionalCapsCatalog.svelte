<script lang="ts">
  import { onMount } from 'svelte';
  import { calcStore } from '../stores/calculation.svelte';
  import type { Jurisdiction } from '../types';

  let searchTerm = $state('');
  let filterNation = $state<'ALL' | Jurisdiction>('ALL');
  let filterTier = $state<'ALL' | 'SMALL' | 'MEDIUM' | 'LARGE'>('ALL');

  onMount(() => {
    if (calcStore.transitionalCaps.length === 0) {
      calcStore.loadParameters();
    }
  });

  const filteredTransitionalCaps = $derived(
    calcStore.transitionalCaps.filter((tc) => {
      if (filterNation !== 'ALL' && tc.jurisdiction !== filterNation) {
        return false;
      }
      if (filterTier !== 'ALL' && tc.property_tier !== filterTier) {
        return false;
      }
      if (searchTerm.trim()) {
        const q = searchTerm.toLowerCase();
        return (
          tc.financial_year.toLowerCase().includes(q) ||
          tc.property_tier.toLowerCase().includes(q) ||
          tc.revaluation_list_year.toString().includes(q) ||
          tc.upward_cap_percentage.toString().includes(q) ||
          tc.parameter_id.toLowerCase().includes(q)
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
    onclick={() => { calcStore.isTransitionalCapsExpanded = !calcStore.isTransitionalCapsExpanded; }}
    class="w-full p-4 flex justify-between items-center text-left bg-slate-50/80 hover:bg-slate-50 transition border-b border-slate-200"
  >
    <div class="flex items-center space-x-2">
      <span class="text-sm font-bold text-slate-900">{calcStore.t.trans_catalog_title}</span>
      <span class="text-[10px] bg-amber-100 text-amber-800 font-bold px-2 py-0.5 rounded">
        {calcStore.transitionalCaps.length} {calcStore.t.trans_seeded_count}
      </span>
    </div>
    <span class="text-xs text-slate-500 font-medium">
      {calcStore.isTransitionalCapsExpanded ? calcStore.t.collapse_btn : calcStore.t.expand_btn}
    </span>
  </button>

  {#if calcStore.isTransitionalCapsExpanded}
    <div class="p-5 space-y-4">
      <!-- Filter Bar -->
      <div class="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-3">
        <div class="flex flex-wrap items-center gap-1.5 text-xs">
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

          <span class="text-slate-300 mx-1">|</span>
          <span class="text-slate-500 font-semibold mr-1">{calcStore.t.trans_tier_label}</span>
          <button
            type="button"
            onclick={() => { filterTier = 'ALL'; }}
            class="px-2 py-1 rounded border text-[11px] font-semibold transition {filterTier === 'ALL'
              ? 'bg-slate-700 text-white border-slate-700 shadow-sm'
              : 'bg-white text-slate-600 border-slate-200 hover:bg-slate-50'}"
          >
            {calcStore.t.trans_tier_all}
          </button>
          <button
            type="button"
            onclick={() => { filterTier = 'SMALL'; }}
            class="px-2 py-1 rounded border text-[11px] font-semibold transition {filterTier === 'SMALL'
              ? 'bg-slate-700 text-white border-slate-700 shadow-sm'
              : 'bg-white text-slate-600 border-slate-200 hover:bg-slate-50'}"
          >
            {calcStore.t.trans_tier_small}
          </button>
          <button
            type="button"
            onclick={() => { filterTier = 'MEDIUM'; }}
            class="px-2 py-1 rounded border text-[11px] font-semibold transition {filterTier === 'MEDIUM'
              ? 'bg-slate-700 text-white border-slate-700 shadow-sm'
              : 'bg-white text-slate-600 border-slate-200 hover:bg-slate-50'}"
          >
            {calcStore.t.trans_tier_medium}
          </button>
          <button
            type="button"
            onclick={() => { filterTier = 'LARGE'; }}
            class="px-2 py-1 rounded border text-[11px] font-semibold transition {filterTier === 'LARGE'
              ? 'bg-slate-700 text-white border-slate-700 shadow-sm'
              : 'bg-white text-slate-600 border-slate-200 hover:bg-slate-50'}"
          >
            {calcStore.t.trans_tier_large}
          </button>
        </div>

        <input
          type="text"
          placeholder={calcStore.t.trans_search_ph}
          bind:value={searchTerm}
          class="w-full sm:w-64 px-3 py-1.5 text-xs border border-slate-300 rounded focus:ring-1 focus:ring-blue-500 font-medium text-slate-800 bg-white"
        />
      </div>

      <!-- Transitional Caps Table -->
      <div class="overflow-x-auto border border-slate-200 rounded-lg">
        <table class="w-full text-left text-xs">
          <thead class="bg-slate-50 text-slate-700 border-b border-slate-200">
            <tr>
              <th class="p-2.5 font-semibold">{calcStore.t.trans_col_reval}</th>
              <th class="p-2.5 font-semibold">{calcStore.t.trans_col_fy}</th>
              <th class="p-2.5 font-semibold">{calcStore.t.trans_col_nation}</th>
              <th class="p-2.5 font-semibold">{calcStore.t.trans_col_tier}</th>
              <th class="p-2.5 font-semibold">{calcStore.t.trans_col_threshold}</th>
              <th class="p-2.5 font-semibold">{calcStore.t.trans_col_cap}</th>
              <th class="p-2.5 font-semibold">{calcStore.t.trans_col_statute}</th>
            </tr>
          </thead>
          <tbody class="divide-y divide-slate-100">
            {#each filteredTransitionalCaps as item}
              <tr class="hover:bg-slate-50/50">
                <td class="p-2.5 font-mono font-bold text-slate-900">{item.revaluation_list_year} List</td>
                <td class="p-2.5 font-mono font-semibold text-slate-800">{item.financial_year}</td>
                <td class="p-2.5">
                  <span class="px-2 py-0.5 rounded text-[10px] font-semibold {item.jurisdiction === 'WALES' ? 'bg-emerald-100 text-emerald-800' : 'bg-blue-100 text-blue-800'}">
                    {item.jurisdiction}
                  </span>
                </td>
                <td class="p-2.5">
                  <span class="px-2 py-0.5 rounded text-[10px] font-semibold {item.property_tier === 'SMALL' ? 'bg-cyan-100 text-cyan-800' : item.property_tier === 'MEDIUM' ? 'bg-amber-100 text-amber-800' : 'bg-rose-100 text-rose-800'}">
                    {item.property_tier}
                  </span>
                </td>
                <td class="p-2.5 text-slate-700 font-mono text-[11px]">
                  {#if item.rv_threshold_max != null}
                    £{Number(item.rv_threshold_min).toLocaleString()} – £{Number(item.rv_threshold_max).toLocaleString()}
                  {:else}
                    &gt; £{Number(item.rv_threshold_min).toLocaleString()}
                  {/if}
                </td>
                <td class="p-2.5 font-mono font-bold text-amber-700 text-sm">
                  +{Number(item.upward_cap_percentage).toFixed(1)}%
                </td>
                <td class="p-2.5 text-slate-500 text-[11px]">
                  {#if item.jurisdiction === 'ENGLAND'}
                    SI 2022/1209 (Chargeable Amounts)
                  {:else}
                    LGF(W)A 2024 Transitional Rules
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

