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
  <div class="bg-white border border-slate-200 rounded-xl p-5 shadow-sm space-y-3">
    <div class="flex justify-between items-center">
      <span class="text-xs font-bold text-slate-700 uppercase tracking-wider">
        {calcStore.t.raw_json_title}
      </span>
      <div class="flex items-center gap-2">
        <button
          type="button"
          onclick={formatJson}
          class="text-xs px-2.5 py-1 rounded bg-slate-100 hover:bg-slate-200 text-slate-700 font-medium transition"
        >
          Format JSON
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

