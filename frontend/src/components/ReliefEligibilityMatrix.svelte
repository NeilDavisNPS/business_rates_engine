<script lang="ts">
  import { calcStore } from '../stores/calculation.svelte';

  function onFlagChange() {
    calcStore.syncFormToJson();
  }
</script>

<div class="bg-white border border-slate-200 rounded-lg p-4 space-y-4">
  <div class="flex justify-between items-center">
    <h3 class="text-xs font-bold text-slate-900 uppercase tracking-wider">
      {calcStore.t.relief_section}
    </h3>
    <span class="text-[10px] bg-slate-100 text-slate-600 px-2 py-0.5 rounded font-medium">
      {calcStore.t.relief_badge} ({calcStore.jurisdiction})
    </span>
  </div>

  <div class="grid grid-cols-1 md:grid-cols-2 gap-3">
    <!-- Small Business Single Property (All nations) -->
    <label class="flex items-start space-x-2.5 p-2 rounded-lg border border-slate-200 hover:bg-slate-50 cursor-pointer">
      <input
        type="checkbox"
        bind:checked={calcStore.qualifyingReliefFlags.is_small_business_sole_property}
        onchange={onFlagChange}
        class="mt-0.5 rounded text-blue-600 focus:ring-blue-500"
      />
      <div>
        <span class="text-xs font-bold text-slate-900 block">{calcStore.jurisdictionInfo.sbrrTitle}</span>
        <span class="text-[11px] text-slate-500">{calcStore.jurisdictionInfo.sbrrDesc}</span>
      </div>
    </label>

    <!-- Mandatory Charitable Relief (All nations) -->
    <label class="flex items-start space-x-2.5 p-2 rounded-lg border border-slate-200 hover:bg-slate-50 cursor-pointer">
      <input
        type="checkbox"
        bind:checked={calcStore.qualifyingReliefFlags.is_charitable_purposes}
        onchange={onFlagChange}
        class="mt-0.5 rounded text-blue-600 focus:ring-blue-500"
      />
      <div>
        <span class="text-xs font-bold text-slate-900 block">{calcStore.t.flag_charity_title}</span>
        <span class="text-[11px] text-slate-500">{calcStore.jurisdictionInfo.charityDesc}</span>
      </div>
    </label>

    <!-- Retail, Hospitality & Leisure / Food & Drink Hospitality (All nations) -->
    <label class="flex items-start space-x-2.5 p-2 rounded-lg border border-slate-200 hover:bg-slate-50 cursor-pointer">
      <input
        type="checkbox"
        bind:checked={calcStore.qualifyingReliefFlags.is_rhl_qualifying}
        onchange={onFlagChange}
        class="mt-0.5 rounded text-blue-600 focus:ring-blue-500"
      />
      <div>
        <span class="text-xs font-bold text-slate-900 block">{calcStore.jurisdictionInfo.rhlTitle}</span>
        <span class="text-[11px] text-slate-500">{calcStore.jurisdictionInfo.rhlDesc}</span>
      </div>
    </label>

    <!-- Scottish Island Property (Scotland only) -->
    {#if calcStore.jurisdiction === 'SCOTLAND' && calcStore.qualifyingReliefFlags.is_rhl_qualifying}
      <label class="flex items-start space-x-2.5 p-2 rounded-lg border border-purple-200 bg-purple-50/50 hover:bg-purple-50 cursor-pointer">
        <input
          type="checkbox"
          bind:checked={calcStore.qualifyingReliefFlags.is_island_property}
          onchange={onFlagChange}
          class="mt-0.5 rounded text-purple-600 focus:ring-purple-500"
        />
        <div>
          <span class="text-xs font-bold text-purple-900 block">Scottish Island Property (100% RHL Relief)</span>
          <span class="text-[11px] text-purple-700">Property located on a Scottish island under the Islands (Scotland) Act 2018 (100% relief capped at £110k).</span>
        </div>
      </label>
    {/if}

    <!-- Rural Settlement Relief (All nations) -->
    <label class="flex items-start space-x-2.5 p-2 rounded-lg border border-slate-200 hover:bg-slate-50 cursor-pointer">
      <input
        type="checkbox"
        bind:checked={calcStore.qualifyingReliefFlags.is_rural_settlement}
        onchange={onFlagChange}
        class="mt-0.5 rounded text-blue-600 focus:ring-blue-500"
      />
      <div>
        <span class="text-xs font-bold text-slate-900 block">{calcStore.t.flag_rural_title}</span>
        <span class="text-[11px] text-slate-500">{calcStore.t.flag_rural_desc}</span>
      </div>
    </label>

    <!-- Public Lavatory Exemption (England & Wales) -->
    {#if calcStore.jurisdiction !== 'SCOTLAND'}
      <label class="flex items-start space-x-2.5 p-2 rounded-lg border border-slate-200 hover:bg-slate-50 cursor-pointer">
        <input
          type="checkbox"
          bind:checked={calcStore.qualifyingReliefFlags.is_public_lavatory}
          onchange={onFlagChange}
          class="mt-0.5 rounded text-blue-600 focus:ring-blue-500"
        />
        <div>
          <span class="text-xs font-bold text-slate-900 block">{calcStore.t.flag_lavatory_title}</span>
          <span class="text-[11px] text-slate-500">{calcStore.t.flag_lavatory_desc}</span>
        </div>
      </label>
    {/if}

    <!-- Low-Carbon Heat Network Relief (England only) -->
    {#if calcStore.jurisdiction === 'ENGLAND'}
      <label class="flex items-start space-x-2.5 p-2 rounded-lg border border-slate-200 hover:bg-slate-50 cursor-pointer">
        <input
          type="checkbox"
          bind:checked={calcStore.qualifyingReliefFlags.is_heat_network}
          onchange={onFlagChange}
          class="mt-0.5 rounded text-blue-600 focus:ring-blue-500"
        />
        <div>
          <span class="text-xs font-bold text-slate-900 block">{calcStore.t.flag_heat_network_title}</span>
          <span class="text-[11px] text-slate-500">{calcStore.t.flag_heat_network_desc}</span>
        </div>
      </label>
    {/if}

    <!-- Improvement Relief G-factor increment (England only) -->
    {#if calcStore.jurisdiction === 'ENGLAND'}
      <div class="p-2 rounded-lg border border-slate-200 space-y-1">
        <div class="flex justify-between items-center">
          <label for="improvementIncrementInput" class="text-xs font-bold text-slate-900 block">
            {calcStore.t.flag_improvement_title}
          </label>
          <div class="relative w-28">
            <span class="absolute inset-y-0 left-0 pl-2 flex items-center text-slate-500 text-xs">£</span>
            <input
              id="improvementIncrementInput"
              type="number"
              min="0"
              step="100"
              bind:value={calcStore.qualifyingReliefFlags.qualifying_improvement_rv_increment}
              oninput={onFlagChange}
              class="w-full pl-5 pr-2 py-1 text-xs border border-slate-300 rounded font-bold text-slate-800 bg-white"
            />
          </div>
        </div>
        <p class="text-[11px] text-slate-500">{calcStore.t.flag_improvement_desc}</p>
      </div>
    {/if}

    <!-- Welsh Artificial Avoidance Penalty (Wales only) -->
    {#if calcStore.jurisdiction === 'WALES'}
      <label class="flex items-start space-x-2.5 p-2 rounded-lg border border-red-200 bg-red-50/50 hover:bg-red-50 cursor-pointer">
        <input
          type="checkbox"
          bind:checked={calcStore.qualifyingReliefFlags.is_artificial_avoidance_suspected}
          onchange={onFlagChange}
          class="mt-0.5 rounded text-red-600 focus:ring-red-500"
        />
        <div>
          <span class="text-xs font-bold text-red-900 block">{calcStore.t.flag_avoidance_title}</span>
          <span class="text-[11px] text-red-700">{calcStore.t.flag_avoidance_desc}</span>
        </div>
      </label>
    {/if}

    <!-- Scottish Holiday Let Days (Scotland only) -->
    {#if calcStore.jurisdiction === 'SCOTLAND'}
      <div class="p-2 rounded-lg border border-purple-200 bg-purple-50/50 space-y-1">
        <div class="flex justify-between items-center">
          <label for="holidayDaysInput" class="text-xs font-bold text-purple-900 block">
            {calcStore.t.flag_holiday_title}
          </label>
          <input
            id="holidayDaysInput"
            type="number"
            min="0"
            max="365"
            placeholder={calcStore.t.placeholder_holiday_days}
            bind:value={calcStore.qualifyingReliefFlags.holiday_let_actual_days_let}
            oninput={onFlagChange}
            class="w-24 px-2 py-1 text-xs border border-purple-300 rounded font-bold text-purple-900"
          />
        </div>
        <p class="text-[11px] text-purple-700">{calcStore.t.flag_holiday_desc}</p>
      </div>
    {/if}

    <!-- Industrial Hereditament (All nations) -->
    <label class="flex items-start space-x-2.5 p-2 rounded-lg border border-slate-200 hover:bg-slate-50 cursor-pointer">
      <input
        type="checkbox"
        bind:checked={calcStore.qualifyingReliefFlags.is_industrial_hereditament}
        onchange={onFlagChange}
        class="mt-0.5 rounded text-blue-600 focus:ring-blue-500"
      />
      <div>
        <span class="text-xs font-bold text-slate-900 block">{calcStore.t.flag_industrial_title}</span>
        <span class="text-[11px] text-slate-500">{calcStore.t.flag_industrial_desc}</span>
      </div>
    </label>

    <!-- Days Previously Unoccupied (All nations) -->
    <div class="p-2 rounded-lg border border-slate-200 space-y-1">
      <div class="flex justify-between items-center">
        <label for="unoccupiedDaysInput" class="text-xs font-bold text-slate-900 block">
          {calcStore.t.flag_unoccupied_days_title}
        </label>
        <input
          id="unoccupiedDaysInput"
          type="number"
          min="0"
          max="365"
          placeholder="0"
          bind:value={calcStore.qualifyingReliefFlags.days_previously_unoccupied}
          oninput={onFlagChange}
          class="w-24 px-2 py-1 text-xs border border-slate-300 rounded font-bold text-slate-800"
        />
      </div>
      <p class="text-[11px] text-slate-500">{calcStore.t.flag_unoccupied_days_desc}</p>
    </div>
  </div>
</div>
