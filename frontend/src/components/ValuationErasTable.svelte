<script lang="ts">
  import { calcStore } from '../stores/calculation.svelte';

  const scatOptions = $derived([
    { code: '091', label: calcStore.t.scat_091 },
    { code: '003', label: calcStore.t.scat_003 },
    { code: '096', label: calcStore.t.scat_096 },
    { code: '209', label: calcStore.t.scat_209 },
    { code: '211', label: calcStore.t.scat_211 },
    { code: '225', label: calcStore.t.scat_225 },
    { code: '249', label: calcStore.t.scat_249 },
    { code: '084', label: calcStore.t.scat_084 },
    { code: '227', label: calcStore.t.scat_227 },
    { code: '024', label: calcStore.t.scat_024 },
    { code: '027', label: calcStore.t.scat_027 },
    { code: '050', label: calcStore.t.scat_050 },
  ]);
</script>

<div class="space-y-4">
  <!-- Hereditament & Property Details -->
  <div class="bg-white border border-slate-200 rounded-lg p-4 space-y-3">
    <h3 class="text-xs font-bold text-slate-900 uppercase tracking-wider">
      {calcStore.t.property_section}
    </h3>

    <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
      <div>
        <label for="scatCodeSelect" class="block text-xs font-semibold text-slate-700 mb-1">
          {calcStore.t.scat_code}
        </label>
        <select
          id="scatCodeSelect"
          name="scatCode"
          bind:value={calcStore.scatCode}
          onchange={() => calcStore.markCalculationDue()}
          class="w-full text-xs px-2.5 py-1.5 border border-slate-300 rounded bg-white font-medium text-slate-800"
        >
          {#each scatOptions as opt}
            <option value={opt.code}>{opt.label}</option>
          {/each}
        </select>
      </div>

      <div>
        <label for="ratingClassificationInput" class="block text-xs font-semibold text-slate-700 mb-1">
          {calcStore.t.rating_classification}
        </label>
        <input
          id="ratingClassificationInput"
          name="ratingClassification"
          type="text"
          value="STANDARD_COMMERCIAL"
          readonly
          class="w-full text-xs px-2.5 py-1.5 border border-slate-200 bg-slate-50 text-slate-600 rounded font-mono"
        />
      </div>
    </div>
  </div>

  <!-- Valuation Eras Table -->
  <div class="bg-white border border-slate-200 rounded-lg p-4 space-y-3">
    <div class="flex justify-between items-center">
      <h3 class="text-xs font-bold text-slate-900 uppercase tracking-wider">
        {calcStore.t.rv_history}
      </h3>
      <button
        type="button"
        onclick={() => calcStore.addValuationEra()}
        class="text-xs px-2.5 py-1 rounded bg-blue-50 text-blue-700 font-bold hover:bg-blue-100 transition"
      >
        {calcStore.t.btn_add_rv}
      </button>
    </div>

    <div class="overflow-x-auto">
      <table class="w-full text-left text-xs border border-slate-200 rounded">
        <thead class="bg-slate-50 text-slate-700 border-b border-slate-200">
          <tr>
            <th class="p-2 font-semibold">{calcStore.t.col_from}</th>
            <th class="p-2 font-semibold">{calcStore.t.col_to}</th>
            <th class="p-2 font-semibold">{calcStore.t.col_rv}</th>
            <th class="p-2 font-semibold text-center">{calcStore.t.col_action}</th>
          </tr>
        </thead>
        <tbody class="divide-y divide-slate-100">
          {#each calcStore.rateableValueHistory as era, idx}
            <tr class="hover:bg-slate-50/50">
              <!-- Effective From -->
              <td class="p-2">
                <input
                  id="valuation_effective_from_{idx}"
                  name="valuation_effective_from_{idx}"
                  type="date"
                  bind:value={era.effective_from}
                  oninput={() => calcStore.markCalculationDue()}
                  aria-label="{calcStore.t.col_from} #{idx + 1}"
                  class="w-full px-2 py-1 text-xs border border-slate-300 rounded focus:ring-1 focus:ring-blue-500 font-mono font-medium text-slate-800 bg-white"
                />
              </td>

              <!-- Effective To -->
              <td class="p-2">
                <input
                  id="valuation_effective_to_{idx}"
                  name="valuation_effective_to_{idx}"
                  type="date"
                  bind:value={era.effective_to}
                  oninput={() => calcStore.markCalculationDue()}
                  placeholder={calcStore.t.placeholder_open}
                  aria-label="{calcStore.t.col_to} #{idx + 1}"
                  class="w-full px-2 py-1 text-xs border border-slate-300 rounded focus:ring-1 focus:ring-blue-500 font-mono font-medium text-slate-800 bg-white"
                />
              </td>

              <!-- Rateable Value -->
              <td class="p-2">
                <div class="relative rounded-md">
                  <span class="absolute inset-y-0 left-0 pl-2 flex items-center text-slate-500 font-mono text-xs">
                    £
                  </span>
                  <input
                    id="valuation_rateable_value_{idx}"
                    name="valuation_rateable_value_{idx}"
                    type="number"
                    min="0"
                    step="100"
                    bind:value={era.rateable_value}
                    oninput={() => calcStore.markCalculationDue()}
                    aria-label="{calcStore.t.col_rv} #{idx + 1}"
                    class="w-full pl-6 pr-2 py-1 text-xs border border-slate-300 rounded focus:ring-1 focus:ring-blue-500 font-bold text-slate-900"
                  />
                </div>
              </td>

              <!-- Action -->
              <td class="p-2 text-center">
                <button
                  type="button"
                  onclick={() => calcStore.removeValuationEra(idx)}
                  disabled={calcStore.rateableValueHistory.length <= 1}
                  aria-label="{calcStore.t.btn_remove} — {calcStore.t.rv_history} #{idx + 1}"
                  class="inline-flex items-center justify-center min-h-[32px] px-2.5 py-1 text-xs font-bold rounded-md border transition-colors
                    bg-red-50 text-red-900 border-red-800
                    hover:bg-red-800 hover:text-white hover:border-red-900
                    focus:outline-hidden focus-visible:ring-2 focus-visible:ring-red-700 focus-visible:ring-offset-2
                    dark:bg-red-950/80 dark:text-red-100 dark:border-red-400
                    dark:hover:bg-red-800 dark:hover:text-white dark:hover:border-red-300
                    disabled:opacity-40 disabled:cursor-not-allowed disabled:hover:bg-red-50 disabled:hover:text-red-900 dark:disabled:hover:bg-red-950/80 dark:disabled:hover:text-red-100"
                >
                  {calcStore.t.btn_remove}
                </button>
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  </div>
</div>
