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
  <div class="p-8 text-center bg-slate-50 dark:bg-slate-800/50 border border-dashed border-slate-200 dark:border-slate-700 rounded-xl text-slate-600 dark:text-slate-300 text-xs">
    <p>{calcStore.t.narrative_placeholder}</p>
  </div>
{:else}
  <div class="bg-white dark:bg-slate-900 border border-slate-200 dark:border-slate-800 rounded-xl p-5 shadow-sm space-y-3">
    <div class="flex justify-between items-center">
      <span class="text-xs font-bold text-slate-800 dark:text-slate-100 uppercase tracking-wider">
        {calcStore.t.tab_narrative}
      </span>
      <button
        type="button"
        onclick={copyNarrative}
        class="text-xs px-2.5 py-1 rounded bg-slate-100 dark:bg-slate-800 hover:bg-slate-200 dark:hover:bg-slate-700 text-slate-800 dark:text-slate-200 hover:text-slate-950 dark:hover:text-white font-semibold transition border border-slate-300 dark:border-slate-700 cursor-pointer"
      >
        {copied ? calcStore.t.copied_btn : calcStore.t.copy_api_key}
      </button>
    </div>

    <!-- High-Contrast Accessible Text Area -->
    <pre class="w-full p-4 rounded-lg bg-slate-950 text-emerald-300 font-mono text-xs leading-relaxed whitespace-pre-wrap overflow-x-auto border border-slate-800 shadow-inner select-all">{calcStore.response.calculation_narrative}</pre>
  </div>
{/if}

