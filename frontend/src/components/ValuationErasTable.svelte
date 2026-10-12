<script lang="ts">
  import { calcStore } from '../stores/calculation.svelte';

  const scatOptions = [
    { code: '091', label: '091 - General Offices / Commercial Premises' },
    { code: '003', label: '003 - Airport / Major Aviation Infrastructure' },
    { code: '096', label: '096 - Industrial Hereditament / Warehouse / Factory' },
    { code: '209', label: '209 - Public House / Bar / Licensed Premise' },
    { code: '211', label: '211 - Petrol Filling Station & Forecourt' },
    { code: '225', label: '225 - Retail Shop / High Street Unit' },
    { code: '249', label: '249 - Self-Catering Holiday Accommodation' },
  ];
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
                  type="date"
                  bind:value={era.effective_from}
                  oninput={() => calcStore.markCalculationDue()}
                  class="w-full px-2 py-1 text-xs border border-slate-300 rounded focus:ring-1 focus:ring-blue-500 font-mono font-medium text-slate-800 bg-white"
                />
              </td>

              <!-- Effective To -->
              <td class="p-2">
                <input
                  type="date"
                  bind:value={era.effective_to}
                  oninput={() => calcStore.markCalculationDue()}
                  placeholder="Open"
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
                    type="number"
                    min="0"
                    step="100"
                    bind:value={era.rateable_value}
                    oninput={() => calcStore.markCalculationDue()}
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
                  class="text-xs px-2 py-0.5 text-red-700 dark:text-red-400 hover:text-red-950 dark:hover:text-red-100 disabled:opacity-30 disabled:cursor-not-allowed font-semibold"
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
