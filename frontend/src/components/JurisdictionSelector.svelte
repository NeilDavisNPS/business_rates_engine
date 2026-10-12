<script lang="ts">
  import { calcStore } from '../stores/calculation.svelte';
  import type { Jurisdiction } from '../types';

  const jurisdictions: { id: Jurisdiction; labelKey: 'jur_btn_england' | 'jur_btn_wales' | 'jur_btn_scotland' }[] = [
    { id: 'ENGLAND', labelKey: 'jur_btn_england' },
    { id: 'WALES', labelKey: 'jur_btn_wales' },
    { id: 'SCOTLAND', labelKey: 'jur_btn_scotland' },
  ];
</script>

<div class="bg-white border-2 border-blue-100 rounded-lg p-3.5 shadow-xs space-y-3">
  <div class="flex justify-between items-center">
    <span class="block text-xs font-bold text-slate-800 uppercase tracking-wider">
      {calcStore.t.rating_jurisdiction}
    </span>
    <span class="text-[10px] text-slate-500 font-medium">
      {calcStore.t.jurisdiction_help}
    </span>
  </div>

  <div class="grid grid-cols-3 gap-2" role="group" aria-label={calcStore.t.rating_jurisdiction}>
    {#each jurisdictions as jur}
      <button
        type="button"
        onclick={() => calcStore.setJurisdiction(jur.id)}
        aria-pressed={calcStore.jurisdiction === jur.id}
        class="py-2 px-3 text-xs font-bold rounded-lg border flex items-center justify-center transition focus:outline-hidden focus-visible:ring-2 focus-visible:ring-blue-500 focus-visible:ring-offset-2 {calcStore.jurisdiction === jur.id
          ? 'bg-blue-600 text-white border-blue-600 shadow-sm'
          : 'bg-white text-slate-700 border-slate-300 hover:bg-slate-50'}"
      >
        <span>{calcStore.t[jur.labelKey]}</span>
      </button>
    {/each}
  </div>
</div>
