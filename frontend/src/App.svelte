<script lang="ts">
  import { onMount } from 'svelte';
  import TopNavbar from './components/TopNavbar.svelte';
  import SummaryKpiCards from './components/SummaryKpiCards.svelte';
  import JurisdictionSelector from './components/JurisdictionSelector.svelte';
  import OccupancyEpisodesTable from './components/OccupancyEpisodesTable.svelte';
  import ValuationErasTable from './components/ValuationErasTable.svelte';
  import ReliefEligibilityMatrix from './components/ReliefEligibilityMatrix.svelte';
  import AnnualBillingSegmentsTable from './components/AnnualBillingSegmentsTable.svelte';
  import StatutoryAuditNarrative from './components/StatutoryAuditNarrative.svelte';
  import RawJsonEditor from './components/RawJsonEditor.svelte';
  import ReferenceCodeCatalog from './components/ReferenceCodeCatalog.svelte';
  import MultipliersCatalog from './components/MultipliersCatalog.svelte';
  import SbrrThresholdsCatalog from './components/SbrrThresholdsCatalog.svelte';
  import SectorReliefsCatalog from './components/SectorReliefsCatalog.svelte';
  import TransitionalCapsCatalog from './components/TransitionalCapsCatalog.svelte';
  import { calcStore } from './stores/calculation.svelte';

  onMount(() => {
    calcStore.loadParameters();
    calcStore.loadReferenceCodes();
    calcStore.executeCalculation();
  });
</script>

<div class="min-h-screen bg-white dark:bg-slate-950 text-slate-800 dark:text-slate-100 antialiased flex flex-col">
  <TopNavbar />

  <main class="max-w-7xl mx-auto w-full px-6 py-8 space-y-8 flex-1">
    <!-- Auth & Identity Cards -->
    <SummaryKpiCards />

    <!-- Test Sandbox Interface -->
    <section class="bg-white dark:bg-slate-900 rounded-xl border border-slate-200 dark:border-slate-800 shadow-sm overflow-hidden">
      <!-- Section Header with Mode Switcher -->
      <div class="border-b border-slate-200 dark:border-slate-800 p-6 flex flex-col md:flex-row justify-between items-start md:items-center gap-4 bg-slate-50/50 dark:bg-slate-900/50">
        <div>
          <h2 class="text-xl font-bold text-slate-900 dark:text-slate-50">
            {calcStore.t.sandbox_heading}
          </h2>
          <p class="text-xs text-slate-500 dark:text-slate-400 mt-0.5">
            {calcStore.t.sandbox_sub}
          </p>
        </div>

        <!-- Form vs Raw JSON Mode Toggle -->
        <div class="inline-flex items-center gap-2">
          <span class="text-xs font-semibold text-slate-600 dark:text-slate-400">
            {calcStore.t.payload_editor}
          </span>
          <div class="inline-flex rounded-lg border border-slate-200 dark:border-slate-700 bg-slate-100 dark:bg-slate-800 p-0.5 text-xs font-semibold">
            <button
              type="button"
              onclick={() => calcStore.setPayloadMode('form')}
              class="px-3 py-1 rounded-md transition {calcStore.payloadMode === 'form'
                ? 'bg-white dark:bg-slate-700 text-blue-700 dark:text-blue-200 shadow-xs font-bold'
                : 'text-slate-700 dark:text-slate-200 hover:text-slate-950 dark:hover:text-white hover:bg-slate-200/60 dark:hover:bg-slate-700/60'}"
            >
              {calcStore.t.btn_form}
            </button>
            <button
              type="button"
              onclick={() => calcStore.setPayloadMode('json')}
              class="px-3 py-1 rounded-md transition {calcStore.payloadMode === 'json'
                ? 'bg-white dark:bg-slate-700 text-blue-700 dark:text-blue-200 shadow-xs font-bold'
                : 'text-slate-700 dark:text-slate-200 hover:text-slate-950 dark:hover:text-white hover:bg-slate-200/60 dark:hover:bg-slate-700/60'}"
            >
              {calcStore.t.btn_json}
            </button>
          </div>
        </div>
      </div>

      <!-- Workbench 2-Column Grid -->
      <div class="p-6 grid grid-cols-1 xl:grid-cols-12 gap-6">
        <!-- Left Column: Payload Setup (6 cols on XL) -->
        <div class="xl:col-span-6 flex flex-col space-y-4">
          <!-- Primary Jurisdiction Selector -->
          <JurisdictionSelector />

          {#if calcStore.payloadMode === 'form'}
            <OccupancyEpisodesTable />
            <ValuationErasTable />
            <ReliefEligibilityMatrix />
          {:else}
            <RawJsonEditor />
          {/if}

          <!-- Execute Calculation Bar -->
          <div class="pt-2 flex flex-col sm:flex-row items-stretch sm:items-center justify-between gap-3">
            <button
              type="button"
              onclick={() => calcStore.executeCalculation()}
              disabled={calcStore.calcStatus === 'calculating'}
              class="px-5 py-2.5 rounded-lg text-white font-bold text-xs shadow transition flex items-center justify-center space-x-2 {calcStore.isCalculationDue
                ? 'bg-amber-600 hover:bg-amber-700 ring-2 ring-amber-400 ring-offset-1 shadow-md'
                : 'bg-blue-600 hover:bg-blue-700 disabled:bg-blue-400'}"
            >
              {#if calcStore.calcStatus === 'calculating'}
                <span class="w-3.5 h-3.5 border-2 border-white border-t-transparent rounded-full animate-spin"></span>
                <span>{calcStore.t.calculating}</span>
              {:else if calcStore.isCalculationDue}
                <span>{calcStore.t.btn_calculate_due}</span>
              {:else}
                <span>{calcStore.t.btn_calculate}</span>
              {/if}
            </button>

            <!-- Status Indicator -->
            <div class="text-xs">
              {#if calcStore.calcStatus === 'calculating'}
                <span class="text-blue-600 font-medium">● {calcStore.t.calculating}</span>
              {:else if calcStore.isCalculationDue}
                <span class="text-amber-600 font-bold">● {calcStore.t.calc_due_status}</span>
              {:else if calcStore.calcStatus === 'ready'}
                <span class="text-slate-500 font-medium">● {calcStore.t.ready}</span>
              {:else}
                {#if calcStore.calcStatus === 'success'}
                  <span class="text-emerald-600 font-bold">✓ {calcStore.t.calc_success}</span>
                {/if}
                {#if calcStore.calcStatus === 'error'}
                  <span class="text-red-600 font-bold">✕ {calcStore.errorMessage || calcStore.t.calc_error}</span>
                {/if}
              {/if}
            </div>
          </div>
        </div>

        <!-- Right Column: Results & Year-by-Year Breakdown (6 cols on XL) -->
        <div class="xl:col-span-6 flex flex-col space-y-4">
          <!-- Results Header & Tabs -->
          <div class="bg-white dark:bg-slate-900 border border-slate-200 dark:border-slate-800 rounded-xl overflow-hidden shadow-sm flex-1 flex flex-col">
            <div class="border-b border-slate-200 dark:border-slate-800 bg-slate-50 dark:bg-slate-800/50 px-4 pt-3 flex flex-wrap justify-between items-center gap-2">
              <h3 class="text-xs font-bold text-slate-800 dark:text-slate-100 uppercase tracking-wider">
                {calcStore.t.results_title}
              </h3>

              <div class="flex space-x-1 text-xs">
                <button
                  type="button"
                  onclick={() => { calcStore.activeResultsTab = 'breakdown'; }}
                  class="px-3.5 py-1.5 font-bold rounded-t-lg transition {calcStore.activeResultsTab === 'breakdown'
                    ? 'bg-white dark:bg-slate-900 text-blue-700 dark:text-blue-300 border-t border-x border-slate-200 dark:border-slate-700 shadow-xs'
                    : 'text-slate-700 dark:text-slate-200 hover:text-slate-950 dark:hover:text-white hover:bg-slate-200/60 dark:hover:bg-slate-700/60'}"
                >
                  {calcStore.t.annual_breakdown_title}
                </button>
                <button
                  type="button"
                  onclick={() => { calcStore.activeResultsTab = 'narrative'; }}
                  class="px-3.5 py-1.5 font-bold rounded-t-lg transition {calcStore.activeResultsTab === 'narrative'
                    ? 'bg-white dark:bg-slate-900 text-blue-700 dark:text-blue-300 border-t border-x border-slate-200 dark:border-slate-700 shadow-xs'
                    : 'text-slate-700 dark:text-slate-200 hover:text-slate-950 dark:hover:text-white hover:bg-slate-200/60 dark:hover:bg-slate-700/60'}"
                >
                  {calcStore.t.tab_narrative}
                </button>
                <button
                  type="button"
                  onclick={() => { calcStore.activeResultsTab = 'json'; }}
                  class="px-3.5 py-1.5 font-bold rounded-t-lg transition {calcStore.activeResultsTab === 'json'
                    ? 'bg-white dark:bg-slate-900 text-blue-700 dark:text-blue-300 border-t border-x border-slate-200 dark:border-slate-700 shadow-xs'
                    : 'text-slate-700 dark:text-slate-200 hover:text-slate-950 dark:hover:text-white hover:bg-slate-200/60 dark:hover:bg-slate-700/60'}"
                >
                  {calcStore.t.tab_json}
                </button>
              </div>
            </div>

            <!-- Tab Content Area -->
            <div class="p-5 flex-1">
              {#if calcStore.activeResultsTab === 'breakdown'}
                <AnnualBillingSegmentsTable />
              {:else if calcStore.activeResultsTab === 'narrative'}
                <StatutoryAuditNarrative />
              {:else if calcStore.activeResultsTab === 'json'}
                {#if !calcStore.response}
                  <div class="p-8 text-center bg-slate-50 border border-dashed border-slate-200 rounded-xl text-slate-500 text-xs">
                    <p>{calcStore.t.json_placeholder}</p>
                  </div>
                {:else}
                  <pre class="w-full p-4 rounded-lg bg-slate-900 text-emerald-400 font-mono text-xs leading-relaxed whitespace-pre-wrap overflow-x-auto border border-slate-800 shadow-inner select-all">{calcStore.rawJsonResponse}</pre>
                {/if}
              {/if}
            </div>
          </div>
        </div>
      </div>
    </section>

    <!-- Seeded Statutory Parameters & Reference Data Catalogs -->
    <section class="space-y-4">
      <div class="border-b border-slate-200 pb-2">
        <h3 class="text-base font-bold text-slate-900">{calcStore.t.parameters_section_title}</h3>
        <p class="text-xs text-slate-500 mt-0.5">{calcStore.t.parameters_section_sub}</p>
      </div>

      <div class="space-y-4">
        <MultipliersCatalog />
        <SbrrThresholdsCatalog />
        <SectorReliefsCatalog />
        <TransitionalCapsCatalog />
        <ReferenceCodeCatalog />
      </div>
    </section>
  </main>
</div>

