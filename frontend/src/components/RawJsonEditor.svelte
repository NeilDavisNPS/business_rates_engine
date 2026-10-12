<script lang="ts">
  import { calcStore } from '../stores/calculation.svelte';

  function onJsonChange(val: string) {
    calcStore.rawJsonRequest = val;
    calcStore.syncJsonToForm();
  }

  function formatJson() {
    try {
      const obj = JSON.parse(calcStore.rawJsonRequest);
      calcStore.rawJsonRequest = JSON.stringify(obj, null, 2);
    } catch (_) {}
  }
</script>

<div class="space-y-4">
  <div class="bg-white dark:bg-slate-900 border border-slate-200 dark:border-slate-800 rounded-xl p-5 shadow-sm space-y-3">
    <div class="flex justify-between items-center">
      <span class="text-xs font-bold text-slate-800 dark:text-slate-100 uppercase tracking-wider">
        {calcStore.t.raw_json_title}
      </span>
      <div class="flex items-center gap-2">
        <button
          type="button"
          onclick={formatJson}
          class="text-xs px-2.5 py-1 rounded bg-slate-100 dark:bg-slate-800 hover:bg-slate-200 dark:hover:bg-slate-700 text-slate-800 dark:text-slate-200 hover:text-slate-950 dark:hover:text-white font-semibold transition border border-slate-300 dark:border-slate-700 cursor-pointer"
        >
          {calcStore.t.btn_format_json}
        </button>
      </div>
    </div>

    <textarea
      bind:value={calcStore.rawJsonRequest}
      oninput={(e) => onJsonChange(e.currentTarget.value)}
      rows="18"
      spellcheck="false"
      class="w-full p-4 rounded-lg bg-slate-900 text-emerald-400 font-mono text-xs leading-relaxed border border-slate-800 shadow-inner focus:outline-none focus:ring-1 focus:ring-blue-500"
    ></textarea>
  </div>
</div>

