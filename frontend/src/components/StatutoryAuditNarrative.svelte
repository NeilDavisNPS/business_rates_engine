<script lang="ts">
  import { calcStore } from '../stores/calculation.svelte';

  let copied = $state(false);

  function copyNarrative() {
    if (!calcStore.response?.calculation_narrative) return;
    navigator.clipboard.writeText(calcStore.response.calculation_narrative).then(() => {
      copied = true;
      setTimeout(() => {
        copied = false;
      }, 2000);
    });
  }
</script>

{#if !calcStore.response}
  <div class="p-8 text-center bg-slate-50 border border-dashed border-slate-200 rounded-xl text-slate-500 text-xs">
    <p>{calcStore.t.narrative_placeholder}</p>
  </div>
{:else}
  <div class="bg-white border border-slate-200 rounded-xl p-5 shadow-sm space-y-3">
    <div class="flex justify-between items-center">
      <span class="text-xs font-bold text-slate-700 uppercase tracking-wider">
        {calcStore.t.tab_narrative}
      </span>
      <button
        type="button"
        onclick={copyNarrative}
        class="text-xs px-2.5 py-1 rounded bg-slate-100 hover:bg-slate-200 text-slate-700 font-medium transition"
      >
        {copied ? '✓ Copied' : calcStore.t.copy_api_key}
      </button>
    </div>

    <!-- High-Contrast Accessible Text Area -->
    <pre class="w-full p-4 rounded-lg bg-slate-900 text-emerald-400 font-mono text-xs leading-relaxed whitespace-pre-wrap overflow-x-auto border border-slate-800 shadow-inner select-all">{calcStore.response.calculation_narrative}</pre>
  </div>
{/if}

