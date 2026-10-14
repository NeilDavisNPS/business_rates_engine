<script lang="ts">
  import { onMount } from 'svelte';
  import { calcStore } from '../stores/calculation.svelte';

  let searchTerm = $state('');

  onMount(() => {
    calcStore.loadReferenceCodes();
  });

  const filteredCodes = $derived(
    calcStore.referenceCodes.filter((code) => {
      // Nation filter
      if (calcStore.dictFilter !== 'ALL') {
        if (code.jurisdiction && code.jurisdiction !== calcStore.dictFilter) {
          return false;
        }
      }
      // Search term
      if (searchTerm.trim()) {
        const q = searchTerm.toLowerCase();
        return (
          code.category.toLowerCase().includes(q) ||
          code.code.toLowerCase().includes(q) ||
          code.label.toLowerCase().includes(q) ||
          code.description.toLowerCase().includes(q)
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
    onclick={() => { calcStore.isDictExpanded = !calcStore.isDictExpanded; }}
    class="w-full p-4 flex justify-between items-center text-left bg-slate-50/80 hover:bg-slate-50 transition border-b border-slate-200"
  >
    <div class="flex items-center space-x-2">
      <span class="text-sm font-bold text-slate-900">{calcStore.t.ref_dict_title}</span>
      <span class="text-[10px] bg-blue-100 text-blue-800 font-bold px-2 py-0.5 rounded">
        {calcStore.t.ref_dict_badge}
      </span>
    </div>
    <span class="text-xs text-slate-500 font-medium">
      {calcStore.isDictExpanded ? calcStore.t.collapse_btn : '▼ ' + calcStore.t.ref_dict_expand}
    </span>
  </button>

  {#if calcStore.isDictExpanded}
    <div class="p-5 space-y-4">
      <!-- Filter Bar -->
      <div class="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-3">
        <div class="flex items-center space-x-1.5 text-xs">
          <span class="text-slate-500 font-semibold mr-1">{calcStore.t.filter_codes_label}</span>
          <button
            type="button"
            onclick={() => { calcStore.dictFilter = 'ALL'; }}
            class="px-2.5 py-1 rounded border text-xs font-semibold transition {calcStore.dictFilter === 'ALL'
              ? 'bg-blue-600 text-white border-blue-600 shadow-sm'
              : 'bg-white text-slate-700 border-slate-200 hover:bg-slate-50'}"
          >
            {calcStore.t.dict_btn_all}
          </button>
          <button
            type="button"
            onclick={() => { calcStore.dictFilter = 'ENGLAND'; }}
            class="px-2.5 py-1 rounded border text-xs font-semibold transition {calcStore.dictFilter === 'ENGLAND'
              ? 'bg-blue-600 text-white border-blue-600 shadow-sm'
              : 'bg-white text-slate-700 border-slate-200 hover:bg-slate-50'}"
          >
            {calcStore.t.dict_btn_england}
          </button>
          <button
            type="button"
            onclick={() => { calcStore.dictFilter = 'WALES'; }}
            class="px-2.5 py-1 rounded border text-xs font-semibold transition {calcStore.dictFilter === 'WALES'
              ? 'bg-blue-600 text-white border-blue-600 shadow-sm'
              : 'bg-white text-slate-700 border-slate-200 hover:bg-slate-50'}"
          >
            {calcStore.t.dict_btn_wales}
          </button>
          <button
            type="button"
            onclick={() => { calcStore.dictFilter = 'SCOTLAND'; }}
            class="px-2.5 py-1 rounded border text-xs font-semibold transition {calcStore.dictFilter === 'SCOTLAND'
              ? 'bg-blue-600 text-white border-blue-600 shadow-sm'
              : 'bg-white text-slate-700 border-slate-200 hover:bg-slate-50'}"
          >
            {calcStore.t.dict_btn_scotland}
          </button>
        </div>

        <input
          id="referenceCodeSearchInput"
          name="referenceCodeSearch"
          type="text"
          placeholder={calcStore.t.ref_search_ph}
          autocomplete="off"
          aria-label={calcStore.t.ref_search_ph}
          bind:value={searchTerm}
          class="w-full sm:w-64 px-3 py-1.5 text-xs border border-slate-300 rounded focus:ring-1 focus:ring-blue-500"
        />
      </div>

      <!-- Codes Table -->
      <div class="overflow-x-auto border border-slate-200 rounded-lg">
        <table class="w-full text-left text-xs">
          <thead class="bg-slate-50 text-slate-700 border-b border-slate-200">
            <tr>
              <th class="p-2.5 font-semibold">{calcStore.t.dict_col_cat}</th>
              <th class="p-2.5 font-semibold">{calcStore.t.dict_col_code}</th>
              <th class="p-2.5 font-semibold">{calcStore.t.dict_col_nation}</th>
              <th class="p-2.5 font-semibold">{calcStore.t.dict_col_label}</th>
              <th class="p-2.5 font-semibold">{calcStore.t.dict_col_rules}</th>
            </tr>
          </thead>
          <tbody class="divide-y divide-slate-100">
            {#each filteredCodes as code}
              <tr class="hover:bg-slate-50/50">
                <td class="p-2.5 font-mono text-slate-500">{code.category}</td>
                <td class="p-2.5 font-mono font-bold text-slate-900">{code.code}</td>
                <td class="p-2.5">
                  <span class="px-2 py-0.5 rounded text-[10px] font-semibold {code.jurisdiction === 'WALES' ? 'bg-emerald-100 text-emerald-800' : code.jurisdiction === 'SCOTLAND' ? 'bg-purple-100 text-purple-800' : code.jurisdiction === 'ENGLAND' ? 'bg-blue-100 text-blue-800' : 'bg-slate-100 text-slate-700'}">
                    {code.jurisdiction || 'GB'}
                  </span>
                </td>
                <td class="p-2.5 font-medium text-slate-800">{code.label}</td>
                <td class="p-2.5 text-slate-600 leading-relaxed text-[11px]">{code.description}</td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    </div>
  {/if}
</div>

