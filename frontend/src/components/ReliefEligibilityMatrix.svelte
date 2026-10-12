<script lang="ts">
  import { calcStore } from '../stores/calculation.svelte';

  function onFlagChange() {
    calcStore.markCalculationDue();
  }
</script>

<div class="bg-white border border-slate-200 rounded-lg p-4 space-y-5">
  <div class="flex justify-between items-center border-b border-slate-100 pb-2">
    <div>
      <h3 class="text-xs font-bold text-slate-900 uppercase tracking-wider">
        {calcStore.t.relief_section}
      </h3>
      <p class="text-[11px] text-slate-500 mt-0.5">
        {calcStore.t.relief_section_sub}
      </p>
    </div>
    <span class="text-[10px] bg-slate-100 text-slate-600 px-2 py-0.5 rounded font-medium">
      {calcStore.t.relief_badge} ({calcStore.jurisdiction})
    </span>
  </div>

  <!-- 1. MANDATORY STATUTORY RELIEFS & EXEMPTIONS -->
  <div class="space-y-2">
    <div class="flex items-center space-x-2">
      <span class="text-[11px] font-bold uppercase tracking-wider text-blue-700 bg-blue-50 px-2 py-0.5 rounded">
        {calcStore.t.mandatory_reliefs_title}
      </span>
      <span class="text-[11px] text-slate-400">{calcStore.t.mandatory_reliefs_sub}</span>
    </div>

    <div class="grid grid-cols-1 md:grid-cols-2 gap-3">
      <!-- Small Business Single Property (All nations) -->
      <label class="flex items-start space-x-2.5 p-2.5 rounded-lg border border-slate-200 hover:bg-slate-50 cursor-pointer transition-colors">
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

      <!-- Small Business Second Property Grace Period (England only - S.I. 2017/411 art 3(4)) -->
      {#if calcStore.jurisdiction === 'ENGLAND'}
        <label class="flex items-start space-x-2.5 p-2.5 rounded-lg border border-blue-200 bg-blue-50/40 hover:bg-blue-50 cursor-pointer transition-colors">
          <input
            type="checkbox"
            bind:checked={calcStore.qualifyingReliefFlags.is_small_business_second_property_grace}
            onchange={onFlagChange}
            class="mt-0.5 rounded text-blue-600 focus:ring-blue-500"
          />
          <div>
            <span class="text-xs font-bold text-blue-950 block">{calcStore.t.flag_sbrr_grace_title}</span>
            <span class="text-[11px] text-blue-800">{calcStore.t.flag_sbrr_grace_desc}</span>
          </div>
        </label>
      {/if}

      <!-- Mandatory Charitable Relief (All nations) -->
      <label class="flex items-start space-x-2.5 p-2.5 rounded-lg border border-slate-200 hover:bg-slate-50 cursor-pointer transition-colors">
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

      <!-- Rural Settlement Relief (All nations) -->
      <label class="flex items-start space-x-2.5 p-2.5 rounded-lg border border-slate-200 hover:bg-slate-50 cursor-pointer transition-colors">
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

      <!-- Telecommunications Infrastructure Relief (All nations - 2018 Act / SSI 2019/111) -->
      <label class="flex items-start space-x-2.5 p-2.5 rounded-lg border border-slate-200 hover:bg-slate-50 cursor-pointer transition-colors">
        <input
          type="checkbox"
          bind:checked={calcStore.qualifyingReliefFlags.is_telecommunications_infrastructure}
          onchange={onFlagChange}
          class="mt-0.5 rounded text-blue-600 focus:ring-blue-500"
        />
        <div>
          <span class="text-xs font-bold text-slate-900 block">{calcStore.t.flag_telecom_title}</span>
          <span class="text-[11px] text-slate-500">{calcStore.t.flag_telecom_desc}</span>
        </div>
      </label>

      <!-- Public Lavatory Exemption (England & Wales) -->
      {#if calcStore.jurisdiction !== 'SCOTLAND'}
        <label class="flex items-start space-x-2.5 p-2.5 rounded-lg border border-slate-200 hover:bg-slate-50 cursor-pointer transition-colors">
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
        <label class="flex items-start space-x-2.5 p-2.5 rounded-lg border border-slate-200 hover:bg-slate-50 cursor-pointer transition-colors">
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
        <div class="p-2.5 rounded-lg border border-slate-200 space-y-1">
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
      <!-- Welsh Retail Premises (Wales only - Local Government Finance (Wales) Act 2024) -->
      {#if calcStore.jurisdiction === 'WALES'}
        <label class="flex items-start space-x-2.5 p-2.5 rounded-lg border border-emerald-200 bg-emerald-50/40 hover:bg-emerald-50 cursor-pointer transition-colors">
          <input
            type="checkbox"
            bind:checked={calcStore.qualifyingReliefFlags.is_retail_premises}
            onchange={onFlagChange}
            class="mt-0.5 rounded text-emerald-600 focus:ring-emerald-500"
          />
          <div>
            <span class="text-xs font-bold text-emerald-950 block">{calcStore.t.flag_welsh_retail_title}</span>
            <span class="text-[11px] text-emerald-800">
              {calcStore.t.flag_welsh_retail_desc}
            </span>
          </div>
        </label>
      {/if}
    </div>
  </div>

  <!-- 2. DISCRETIONARY RELIEFS & REVALUATION PROTECTIONS -->
  <div class="space-y-2 pt-2 border-t border-slate-100">
    <div class="flex items-center space-x-2">
      <span class="text-[11px] font-bold uppercase tracking-wider text-purple-700 bg-purple-50 px-2 py-0.5 rounded">
        {calcStore.t.discretionary_section_title}
      </span>
      <span class="text-[11px] text-slate-400">{calcStore.t.discretionary_section_sub}</span>
    </div>

    <div class="grid grid-cols-1 md:grid-cols-2 gap-3">
      <!-- Statutory Transitional Relief Phasing Toggle (All nations) -->
      <label class="flex items-start space-x-2.5 p-2.5 rounded-lg border border-amber-200 bg-amber-50/40 hover:bg-amber-50 cursor-pointer transition-colors">
        <input
          type="checkbox"
          bind:checked={calcStore.qualifyingReliefFlags.apply_transitional_relief}
          onchange={onFlagChange}
          class="mt-0.5 rounded text-amber-600 focus:ring-amber-500"
        />
        <div>
          <span class="text-xs font-bold text-amber-950 block">{calcStore.t.flag_tr_toggle_title}</span>
          <span class="text-[11px] text-amber-800">
            {#if calcStore.jurisdiction === 'WALES'}
              {calcStore.t.flag_tr_toggle_desc_wales}
            {:else if calcStore.jurisdiction === 'SCOTLAND'}
              {calcStore.t.flag_tr_toggle_desc_scotland}
            {:else}
              {calcStore.t.flag_tr_toggle_desc_england}
            {/if}
          </span>
        </div>
      </label>

      <!-- Supporting Small Business Relief (SSBR) (England only - LGFA 1988 s.47) -->
      {#if calcStore.jurisdiction === 'ENGLAND'}
        <label class="flex items-start space-x-2.5 p-2.5 rounded-lg border border-purple-200 bg-purple-50/40 hover:bg-purple-50 cursor-pointer transition-colors">
          <input
            type="checkbox"
            bind:checked={calcStore.qualifyingReliefFlags.is_ssbr_qualifying}
            onchange={onFlagChange}
            class="mt-0.5 rounded text-purple-600 focus:ring-purple-500"
          />
          <div>
            <span class="text-xs font-bold text-purple-950 block">{calcStore.t.flag_ssbr_title}</span>
            <span class="text-[11px] text-purple-800">
              {calcStore.t.flag_ssbr_desc}
            </span>
          </div>
        </label>
      {/if}

      <!-- Retail, Hospitality & Leisure / Food & Drink Hospitality (All nations) -->
      <label class="flex items-start space-x-2.5 p-2.5 rounded-lg border border-slate-200 hover:bg-slate-50 cursor-pointer transition-colors">
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
        <label class="flex items-start space-x-2.5 p-2.5 rounded-lg border border-purple-200 bg-purple-50/50 hover:bg-purple-50 cursor-pointer transition-colors">
          <input
            type="checkbox"
            bind:checked={calcStore.qualifyingReliefFlags.is_island_property}
            onchange={onFlagChange}
            class="mt-0.5 rounded text-purple-600 focus:ring-purple-500"
          />
          <div>
            <span class="text-xs font-bold text-purple-900 block">{calcStore.t.flag_island_title}</span>
            <span class="text-[11px] text-purple-700">{calcStore.t.flag_island_desc}</span>
          </div>
        </label>
      {/if}

      <!-- Scottish SBBS Transitional Relief (Scotland only) -->
      {#if calcStore.jurisdiction === 'SCOTLAND'}
        <label class="flex items-start space-x-2.5 p-2.5 rounded-lg border border-purple-200 bg-purple-50/50 hover:bg-purple-50 cursor-pointer transition-colors">
          <input
            type="checkbox"
            bind:checked={calcStore.qualifyingReliefFlags.is_sbbs_tr_qualifying}
            onchange={onFlagChange}
            class="mt-0.5 rounded text-purple-600 focus:ring-purple-500"
          />
          <div>
            <span class="text-xs font-bold text-purple-900 block">{calcStore.t.flag_sbbs_tr_title}</span>
            <span class="text-[11px] text-purple-700">{calcStore.t.flag_sbbs_tr_desc}</span>
          </div>
        </label>
      {/if}
      <!-- Charity Discretionary Top-Up (All nations - LGFA 1988 s.47(2)(a) / LGA(S) 1962 s.4(5)) -->
      <label class="flex items-start space-x-2.5 p-2.5 rounded-lg border border-purple-200 bg-purple-50/30 hover:bg-purple-50 cursor-pointer transition-colors">
        <input
          type="checkbox"
          bind:checked={calcStore.qualifyingReliefFlags.is_charity_discretionary_top_up}
          onchange={onFlagChange}
          class="mt-0.5 rounded text-purple-600 focus:ring-purple-500"
        />
        <div>
          <span class="text-xs font-bold text-purple-950 block">{calcStore.t.flag_charity_topup_title}</span>
          <span class="text-[11px] text-purple-800">
            {calcStore.t.flag_charity_topup_desc}
          </span>
        </div>
      </label>

      <!-- Scottish Renewable Energy Generation Relief (Scotland only - SSI 2010/336) -->
      {#if calcStore.jurisdiction === 'SCOTLAND'}
        <label class="flex items-start space-x-2.5 p-2.5 rounded-lg border border-emerald-200 bg-emerald-50/40 hover:bg-emerald-50 cursor-pointer transition-colors">
          <input
            type="checkbox"
            bind:checked={calcStore.qualifyingReliefFlags.is_renewable_energy_generation}
            onchange={onFlagChange}
            class="mt-0.5 rounded text-emerald-600 focus:ring-emerald-500"
          />
          <div>
            <span class="text-xs font-bold text-emerald-950 block">{calcStore.t.flag_renewable_title}</span>
            <span class="text-[11px] text-emerald-800">
              {calcStore.t.flag_renewable_desc}
            </span>
          </div>
        </label>
      {/if}
    </div>
  </div>

  <!-- 3. PROPERTY & OCCUPANCY CONDITIONS -->
  <div class="space-y-2 pt-2 border-t border-slate-100">
    <div class="flex items-center space-x-2">
      <span class="text-[11px] font-bold uppercase tracking-wider text-slate-700 bg-slate-100 px-2 py-0.5 rounded">
        {calcStore.t.property_conditions_title}
      </span>
      <span class="text-[11px] text-slate-400">{calcStore.t.property_conditions_sub}</span>
    </div>

    <div class="grid grid-cols-1 md:grid-cols-2 gap-3">
      <!-- Industrial Hereditament (All nations) -->
      <label class="flex items-start space-x-2.5 p-2.5 rounded-lg border border-slate-200 hover:bg-slate-50 cursor-pointer transition-colors">
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
      <div class="p-2.5 rounded-lg border border-slate-200 space-y-1">
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

      <!-- Self-Catering Holiday Accommodation Actual Days Let (All nations - 182 days Scotland, 70 days England/Wales) -->
      {#if calcStore.scatCode === '249' || calcStore.jurisdiction === 'SCOTLAND'}
        <div class="p-2.5 rounded-lg border border-purple-200 bg-purple-50/50 space-y-1">
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
          <p class="text-[11px] text-purple-700">
            {calcStore.jurisdiction === 'SCOTLAND' ? calcStore.t.holiday_desc_scotland : calcStore.t.holiday_desc_eng_wales}
          </p>
        </div>
      {/if}

      <!-- Statutory Installment Schedule Plan (SI 1989/1058 Sched 1 / LGA 2003) -->
      <div class="p-2.5 rounded-lg border border-slate-200 space-y-1.5">
        <label for="installmentsPlanSelect" class="text-xs font-bold text-slate-900 block">
          {calcStore.t.installments_plan_label}
        </label>
        <select
          id="installmentsPlanSelect"
          bind:value={calcStore.qualifyingReliefFlags.payment_installments_count}
          onchange={onFlagChange}
          class="w-full max-w-full px-2 py-1 text-xs border border-slate-300 rounded font-bold text-slate-800 bg-white cursor-pointer focus:ring-1 focus:ring-blue-500"
        >
          <option value={10}>{calcStore.t.installments_10_opt}</option>
          <option value={12}>{calcStore.t.installments_12_opt}</option>
        </select>
        <p class="text-[11px] text-slate-500">
          {calcStore.t.installments_help}
        </p>
      </div>

      <!-- Welsh Artificial Avoidance Penalty (Wales only) -->
      {#if calcStore.jurisdiction === 'WALES'}
        <label class="flex items-start space-x-2.5 p-2.5 rounded-lg border border-red-200 bg-red-50/50 hover:bg-red-50 cursor-pointer transition-colors">
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
    </div>
  </div>
</div>
