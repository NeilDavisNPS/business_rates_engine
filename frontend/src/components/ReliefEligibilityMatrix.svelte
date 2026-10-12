<script lang="ts">
  import { calcStore } from '../stores/calculation.svelte';

  function onFlagChange() {
    calcStore.syncFormToJson();
  }
</script>

<div class="bg-white border border-slate-200 rounded-lg p-4 space-y-5">
  <div class="flex justify-between items-center border-b border-slate-100 pb-2">
    <div>
      <h3 class="text-xs font-bold text-slate-900 uppercase tracking-wider">
        {calcStore.t.relief_section}
      </h3>
      <p class="text-[11px] text-slate-500 mt-0.5">
        Statutory mandatory reliefs, discretionary schemes, and revaluation transitional phasing.
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
        Mandatory Statutory Reliefs
      </span>
      <span class="text-[11px] text-slate-400">Awarded by right when qualifying conditions are met</span>
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
            <span class="text-xs font-bold text-blue-950 block">SBRR Second Property 12-Month Grace Period</span>
            <span class="text-[11px] text-blue-800">Retain SBRR on main hereditament for 12 months after taking on an additional property (S.I. 2017/411 art. 3(4) / LGFA 1988 s.43(4B)).</span>
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
            <span class="text-xs font-bold text-emerald-950 block">Welsh Retail Premises (35.0p Multiplier)</span>
            <span class="text-[11px] text-emerald-800">
              Qualifying retail hereditament with RV &lt; £51,000 eligible for the 35.0p Welsh Retail Multiplier under Local Government Finance (Wales) Act 2024.
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
        Discretionary & Revaluation Protections
      </span>
      <span class="text-[11px] text-slate-400">Local authority discretionary schemes and statutory transitional caps</span>
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
          <span class="text-xs font-bold text-amber-950 block">Apply Statutory Transitional Relief (TR)</span>
          <span class="text-[11px] text-amber-800">
            {#if calcStore.jurisdiction === 'WALES'}
              Automatically caps revaluation bill increases exceeding £300 under W.S.I. 2025/1371 (67% relief in 2026/27, 34% in 2027/28). Uncheck to calculate unphased liability.
            {:else if calcStore.jurisdiction === 'SCOTLAND'}
              Automatically caps revaluation bill increases under Scottish Transitional Relief Regulations. Uncheck to calculate unphased liability.
            {:else}
              Automatically caps revaluation bill increases (e.g. England 2026/27: +5% Small, +15% Medium, +30% Large under LGFA 1988 s.57A / NDRA 2023). Uncheck to calculate unphased liability.
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
            <span class="text-xs font-bold text-purple-950 block">Supporting Small Business Relief (SSBR)</span>
            <span class="text-[11px] text-purple-800">
              Discretionary s.47 protection capping net bill increases to £600/yr (2023–2025) and £800/yr (2026/27) for ratepayers losing or seeing reduced SBRR, Rural, or RHL relief.
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
            <span class="text-xs font-bold text-purple-900 block">Scottish Island Property (100% RHL Relief)</span>
            <span class="text-[11px] text-purple-700">Property located on a Scottish island under Islands (Scotland) Act 2018 (100% relief capped at £110k).</span>
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
            <span class="text-xs font-bold text-purple-900 block">Small Business Bonus Transitional Relief (SBBS TR)</span>
            <span class="text-[11px] text-purple-700">Caps bill increases for properties losing or seeing reduced SBBS (£600/£1,200/£1,800 under 2023 list; max 25% bill increase in 2026/27).</span>
          </div>
        </label>
      {/if}
    </div>
  </div>

  <!-- 3. PROPERTY & OCCUPANCY CONDITIONS -->
  <div class="space-y-2 pt-2 border-t border-slate-100">
    <div class="flex items-center space-x-2">
      <span class="text-[11px] font-bold uppercase tracking-wider text-slate-700 bg-slate-100 px-2 py-0.5 rounded">
        Occupancy & Property Conditions
      </span>
      <span class="text-[11px] text-slate-400">Empty exemptions, self-catering qualifications, and compliance checks</span>
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

      <!-- Scottish Holiday Let Days (Scotland only) -->
      {#if calcStore.jurisdiction === 'SCOTLAND'}
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
          <p class="text-[11px] text-purple-700">{calcStore.t.flag_holiday_desc}</p>
        </div>
      {/if}

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
