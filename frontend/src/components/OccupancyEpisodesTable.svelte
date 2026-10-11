<script lang="ts">
  import { calcStore } from '../stores/calculation.svelte';
  import type { OccupancyType } from '../types';

  function onTypeChange(index: number, newType: OccupancyType) {
    calcStore.occupancyPeriods[index].occupancy_type = newType;
    if (newType === 'PART_OCCUPIED') {
      if (!calcStore.occupancyPeriods[index].part_occupied_apportionment) {
        calcStore.occupancyPeriods[index].part_occupied_apportionment = {
          occupied_percentage: 50,
        };
      }
    } else {
      calcStore.occupancyPeriods[index].part_occupied_apportionment = undefined;
    }
    calcStore.syncFormToJson();
  }

  function onPctChange(index: number, value: string) {
    const num = parseFloat(value);
    if (!calcStore.occupancyPeriods[index].part_occupied_apportionment) {
      calcStore.occupancyPeriods[index].part_occupied_apportionment = {
        occupied_percentage: isNaN(num) ? 50 : num,
      };
    } else {
      calcStore.occupancyPeriods[index].part_occupied_apportionment!.occupied_percentage = isNaN(num) ? 50 : num;
    }
    calcStore.syncFormToJson();
  }
</script>

<div class="bg-white border border-slate-200 rounded-lg p-4 space-y-3">
  <div class="flex justify-between items-center">
    <h3 class="text-xs font-bold text-slate-900 uppercase tracking-wider">
      {calcStore.t.ratepayer_section}
    </h3>
    <button
      type="button"
      onclick={() => calcStore.addOccupancyPeriod()}
      class="text-xs px-2.5 py-1 rounded bg-blue-50 text-blue-700 font-bold hover:bg-blue-100 transition"
    >
      {calcStore.t.btn_add_occ}
    </button>
  </div>

  <div class="overflow-x-auto">
    <table class="w-full text-left text-xs border border-slate-200 rounded">
      <thead class="bg-slate-50 text-slate-700 border-b border-slate-200">
        <tr>
          <th class="p-2 font-semibold">{calcStore.t.col_occ_start}</th>
          <th class="p-2 font-semibold">{calcStore.t.col_occ_end}</th>
          <th class="p-2 font-semibold">{calcStore.t.col_occ_type}</th>
          <th class="p-2 font-semibold">{calcStore.t.col_occ_apportionment}</th>
          <th class="p-2 font-semibold text-center">{calcStore.t.col_occ_action}</th>
        </tr>
      </thead>
      <tbody class="divide-y divide-slate-100">
        {#each calcStore.occupancyPeriods as period, idx}
          <tr class="hover:bg-slate-50/50">
            <!-- Start Date -->
            <td class="p-2">
              <input
                type="date"
                bind:value={period.start_date}
                oninput={() => calcStore.syncFormToJson()}
                class="w-full px-2 py-1 text-xs border border-slate-300 rounded focus:ring-1 focus:ring-blue-500 font-mono font-medium text-slate-800 bg-white"
              />
            </td>

            <!-- End Date -->
            <td class="p-2">
              <input
                type="date"
                bind:value={period.end_date}
                oninput={() => calcStore.syncFormToJson()}
                placeholder="Open"
                class="w-full px-2 py-1 text-xs border border-slate-300 rounded focus:ring-1 focus:ring-blue-500 font-mono font-medium text-slate-800 bg-white"
              />
            </td>

            <!-- Occupancy Status -->
            <td class="p-2">
              <select
                value={period.occupancy_type}
                onchange={(e) => onTypeChange(idx, e.currentTarget.value as OccupancyType)}
                class="w-full px-2 py-1 text-xs border border-slate-300 rounded focus:ring-1 focus:ring-blue-500 bg-white font-medium text-slate-800"
              >
                <option value="OCCUPIED">{calcStore.t.occ_opt_occupied}</option>
                <option value="UNOCCUPIED">{calcStore.t.occ_opt_unoccupied}</option>
                <option value="PART_OCCUPIED">{calcStore.t.occ_opt_part_occupied}</option>
              </select>
            </td>

            <!-- Section 44A Apportionment -->
            <td class="p-2">
              {#if period.occupancy_type === 'PART_OCCUPIED'}
                <div class="flex items-center space-x-1">
                  <input
                    type="number"
                    min="1"
                    max="99"
                    step="1"
                    value={period.part_occupied_apportionment?.occupied_percentage ?? 50}
                    oninput={(e) => onPctChange(idx, e.currentTarget.value)}
                    class="w-16 px-1.5 py-1 text-xs border border-amber-300 bg-amber-50 rounded font-bold text-amber-900"
                  />
                  <span class="text-amber-800 font-bold">% occ</span>
                </div>
              {:else}
                <span class="text-slate-600 italic">{calcStore.t.occ_na_text}</span>
              {/if}
            </td>

            <!-- Action -->
            <td class="p-2 text-center">
              <button
                type="button"
                onclick={() => calcStore.removeOccupancyPeriod(idx)}
                disabled={calcStore.occupancyPeriods.length <= 1}
                class="text-xs px-2 py-0.5 text-red-600 hover:text-red-800 disabled:opacity-30 disabled:cursor-not-allowed font-medium"
              >
                {calcStore.t.btn_remove}
              </button>
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>

  <!-- Common Party Type Selector -->
  <div class="pt-2 border-t border-slate-100 flex flex-col sm:flex-row sm:items-center justify-between gap-2">
    <label for="partyTypeSelect" class="text-xs font-semibold text-slate-700">
      {calcStore.t.party_type}
    </label>
    <select
      id="partyTypeSelect"
      bind:value={calcStore.partyType}
      onchange={() => calcStore.syncFormToJson()}
      class="text-xs px-2.5 py-1 border border-slate-300 rounded bg-white font-medium text-slate-800 max-w-xs"
    >
      <option value="CORPORATE">CORPORATE (Company / Ltd / PLC)</option>
      <option value="INDIVIDUAL">INDIVIDUAL (Natural Person)</option>
      <option value="CHARITY">CHARITY (Registered Charity)</option>
      <option value="CASC">CASC (Community Amateur Sports Club)</option>
      <option value="LOCAL_AUTHORITY">LOCAL_AUTHORITY (Billing Authority / Public Body)</option>
    </select>
  </div>
</div>
