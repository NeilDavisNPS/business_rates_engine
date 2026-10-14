<script lang="ts">
  import { onMount } from 'svelte';
  import { calcStore } from '../stores/calculation.svelte';
  import type { Jurisdiction } from '../types';

  let searchTerm = $state('');
  let filterNation = $state<'ALL' | Jurisdiction>('ALL');

  onMount(() => {
    if (calcStore.sbrrThresholds.length === 0) {
      calcStore.loadParameters();
    }
  });

  const filteredThresholds = $derived(
    calcStore.sbrrThresholds.filter((t) => {
      if (filterNation !== 'ALL' && t.jurisdiction !== filterNation) {
        return false;
      }
      if (searchTerm.trim()) {
        const q = searchTerm.toLowerCase();
        return (
          t.financial_year.toLowerCase().includes(q) ||
          t.parameter_id.toLowerCase().includes(q) ||
          t.relief_percentage.toString().includes(q)
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
    onclick={() => { calcStore.isSbrrExpanded = !calcStore.isSbrrExpanded; }}
    class="w-full p-4 flex justify-between items-center text-left bg-slate-50/80 hover:bg-slate-50 transition border-b border-slate-200"
  >
    <div class="flex items-center space-x-2">
      <span class="text-sm font-bold text-slate-900">{calcStore.t.sbrr_catalog_title}</span>
      <span class="text-[10px] bg-emerald-100 text-emerald-800 font-bold px-2 py-0.5 rounded">
        {calcStore.sbrrThresholds.length} {calcStore.t.sbrr_seeded_count}
      </span>
    </div>
    <span class="text-xs text-slate-500 font-medium">
      {calcStore.isSbrrExpanded ? calcStore.t.collapse_btn : calcStore.t.expand_btn}
    </span>
  </button>

  {#if calcStore.isSbrrExpanded}
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
          placeholder={calcStore.t.sbrr_search_ph}
          bind:value={searchTerm}
          class="w-full sm:w-64 px-3 py-1.5 text-xs border border-slate-300 rounded focus:ring-1 focus:ring-blue-500 font-medium text-slate-800 bg-white"
        />
      </div>

      <!-- Thresholds Table -->
      <div class="overflow-x-auto border border-slate-200 rounded-lg">
        <table class="w-full text-left text-xs">
          <thead class="bg-slate-50 text-slate-700 border-b border-slate-200">
            <tr>
              <th class="p-2.5 font-semibold">{calcStore.t.sbrr_col_fy}</th>
              <th class="p-2.5 font-semibold">{calcStore.t.sbrr_col_nation}</th>
              <th class="p-2.5 font-semibold">{calcStore.t.sbrr_col_range}</th>
              <th class="p-2.5 font-semibold">{calcStore.t.sbrr_col_max_relief}</th>
              <th class="p-2.5 font-semibold">{calcStore.t.sbrr_col_mechanics}</th>
              <th class="p-2.5 font-semibold">{calcStore.t.sbrr_col_statute}</th>
              <th class="p-2.5 font-semibold">{calcStore.t.sbrr_col_period}</th>
            </tr>
          </thead>
          <tbody class="divide-y divide-slate-100">
            {#each filteredThresholds as item}
              <tr class="hover:bg-slate-50/50">
                <td class="p-2.5 font-mono font-bold text-slate-900">{item.financial_year}</td>
                <td class="p-2.5">
                  <span class="px-2 py-0.5 rounded text-[10px] font-semibold {item.jurisdiction === 'WALES' ? 'bg-emerald-100 text-emerald-900 border border-emerald-300' : item.jurisdiction === 'SCOTLAND' ? 'bg-purple-100 text-purple-900 border border-purple-300' : 'bg-blue-100 text-blue-900 border border-blue-300'}">
                    {item.jurisdiction}
                  </span>
                </td>
                <td class="p-2.5 font-mono text-slate-800 font-medium">
                  £{Number(item.min_rv).toLocaleString()} – £{Number(item.max_rv).toLocaleString()}
                </td>
                <td class="p-2.5 font-mono font-bold text-emerald-700 text-sm">
                  {Number(item.relief_percentage).toFixed(0)}%
                </td>
                <td class="p-2.5">
                  {#if item.is_tapered}
                    <span class="inline-flex items-center px-2 py-0.5 rounded text-[11px] font-semibold bg-amber-100 text-amber-900 border border-amber-300">
                      {calcStore.t.sbrr_linear_taper}
                    </span>
                  {:else}
                    <span class="inline-flex items-center px-2 py-0.5 rounded text-[11px] font-semibold bg-emerald-100 text-emerald-900 border border-emerald-300">
                      {calcStore.t.sbrr_full_exemption}
                    </span>
                  {/if}
                </td>
                <td class="p-2.5 text-slate-600 text-[11px]">
                  {#if item.jurisdiction === 'ENGLAND'}
                    {#if item.financial_year >= '2023/24'}
                      LGFA 1988 s.43(4B) / NDRA 2023
                    {:else if item.financial_year >= '2017/18'}
                      S.I. 2017/411 / S.I. 2017/102
                    {:else if item.financial_year >= '2010/11'}
                      S.I. 2010/1959 / S.I. 2012/148
                    {:else}
                      S.I. 2004/3315
                    {/if}
                  {:else if item.jurisdiction === 'WALES'}
                    {#if item.financial_year >= '2024/25'}
                      LGF(W)A 2024 / W.S.I. 2017/1229
                    {:else}
                      W.S.I. 2017/1229 / W.S.I. 2008/2499
                    {/if}
                  {:else}
                    {#if item.financial_year >= '2026/27'}
                      S.S.I. 2026/39 &amp; 2026/68
                    {:else if item.financial_year >= '2023/24'}
                      S.S.I. 2023/34 / S.S.I. 2023/30
                    {:else if item.financial_year >= '2017/18'}
                      S.S.I. 2017/36
                    {:else if item.financial_year >= '2010/11'}
                      S.S.I. 2010/37
                    {:else}
                      S.S.I. 2008/106
                    {/if}
                  {/if}
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

