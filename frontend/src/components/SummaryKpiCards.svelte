<script lang="ts">
  import { calcStore } from '../stores/calculation.svelte';

  let isExpanded = $state<boolean>(false);
  let selectedCountry = $state<'ENGLAND' | 'WALES' | 'SCOTLAND'>('ENGLAND');

  const countryData = $derived(calcStore.t.statutory_framework);
</script>

<!-- Statutory Framework Section -->
<section class="bg-white dark:bg-slate-900 rounded-xl border border-slate-200 dark:border-slate-800 shadow-sm overflow-hidden transition-all">
  <!-- Expandable Header Bar -->
  <div class="px-5 py-4 bg-white dark:bg-slate-900">
    <button
      type="button"
      onclick={() => (isExpanded = !isExpanded)}
      aria-expanded={isExpanded}
      class="w-full flex items-center justify-between text-left group p-1 -m-1 rounded-lg hover:bg-slate-50 dark:hover:bg-slate-800/60 transition focus:outline-hidden focus-visible:ring-2 focus-visible:ring-blue-500"
    >
      <div class="flex items-center gap-3">
        <span class="inline-flex items-center justify-center w-7 h-7 rounded-lg bg-blue-100 dark:bg-blue-950 text-blue-700 dark:text-blue-300 text-sm font-bold shadow-xs">
          §
        </span>
        <div>
          <div class="flex items-center gap-2">
            <h2 class="text-base font-bold text-slate-900 dark:text-slate-50">
              {calcStore.t.statutory_framework_title}
            </h2>
            <span class="text-xs font-semibold px-2 py-0.5 rounded bg-blue-50 dark:bg-blue-950 text-blue-700 dark:text-blue-300 border border-blue-200 dark:border-blue-800">
              {calcStore.t.statutory_coverage_sub}
            </span>
          </div>
          <p class="text-xs text-slate-500 dark:text-slate-400 mt-0.5">
            {calcStore.t.statutory_framework_sub}
          </p>
        </div>
      </div>
      <div class="flex items-center gap-2">
        <span class="text-xs font-semibold text-blue-600 dark:text-blue-400 group-hover:underline">
          {isExpanded ? calcStore.t.statutory_framework_collapse : calcStore.t.statutory_framework_expand}
        </span>
        <svg
          xmlns="http://www.w3.org/2000/svg"
          class="w-4 h-4 text-slate-400 dark:text-slate-500 transition-transform duration-200 {isExpanded ? 'rotate-180 text-blue-600' : ''}"
          viewBox="0 0 20 20"
          fill="currentColor"
        >
          <path fill-rule="evenodd" d="M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z" clip-rule="evenodd" />
        </svg>
      </div>
    </button>
  </div>

  <!-- Expandable Content Panel -->
  {#if isExpanded}
    <div class="border-t border-slate-200 dark:border-slate-800 p-6 bg-slate-50/40 dark:bg-slate-950/40 space-y-6">
      <!-- Country Selection Buttons (England, Wales, Scotland) with No Icons -->
      <div class="flex flex-wrap gap-2.5 pb-2 border-b border-slate-200 dark:border-slate-800" role="tablist">
        <button
          type="button"
          role="tab"
          aria-selected={selectedCountry === 'ENGLAND'}
          onclick={() => (selectedCountry = 'ENGLAND')}
          class="px-5 py-2 rounded-lg text-xs font-bold transition border {selectedCountry === 'ENGLAND'
            ? 'bg-blue-700 text-white border-blue-700 shadow-sm ring-2 ring-blue-400/40'
            : 'bg-white dark:bg-slate-800 text-slate-700 dark:text-slate-200 hover:bg-slate-100 dark:hover:bg-slate-700 hover:text-slate-950 dark:hover:text-white border-slate-300 dark:border-slate-700'}"
        >
          {calcStore.t.jur_btn_england}
        </button>
        <button
          type="button"
          role="tab"
          aria-selected={selectedCountry === 'WALES'}
          onclick={() => (selectedCountry = 'WALES')}
          class="px-5 py-2 rounded-lg text-xs font-bold transition border {selectedCountry === 'WALES'
            ? 'bg-emerald-700 text-white border-emerald-700 shadow-sm ring-2 ring-emerald-400/40'
            : 'bg-white dark:bg-slate-800 text-slate-700 dark:text-slate-200 hover:bg-slate-100 dark:hover:bg-slate-700 hover:text-slate-950 dark:hover:text-white border-slate-300 dark:border-slate-700'}"
        >
          {calcStore.t.jur_btn_wales}
        </button>
        <button
          type="button"
          role="tab"
          aria-selected={selectedCountry === 'SCOTLAND'}
          onclick={() => (selectedCountry = 'SCOTLAND')}
          class="px-5 py-2 rounded-lg text-xs font-bold transition border {selectedCountry === 'SCOTLAND'
            ? 'bg-purple-700 text-white border-purple-700 shadow-sm ring-2 ring-purple-400/40'
            : 'bg-white dark:bg-slate-800 text-slate-700 dark:text-slate-200 hover:bg-slate-100 dark:hover:bg-slate-700 hover:text-slate-950 dark:hover:text-white border-slate-300 dark:border-slate-700'}"
        >
          {calcStore.t.jur_btn_scotland}
        </button>
      </div>

      <!-- Country Region Details -->
      {#key selectedCountry}
        <div class="space-y-6 animate-fadeIn">
          <!-- Top of Country Region: Appropriate Legislation Links -->
          <div class="bg-white dark:bg-slate-900 rounded-xl p-4 border border-slate-200 dark:border-slate-800 shadow-xs flex flex-col sm:flex-row justify-between items-start sm:items-center gap-3">
            <div>
              <span class="text-[11px] font-bold text-slate-600 dark:text-slate-300 uppercase tracking-wider block">
                {countryData[selectedCountry].name} {calcStore.t.framework_authorities_title}
              </span>
              <span class="text-xs text-slate-800 dark:text-slate-100 font-semibold">
                {calcStore.t.framework_authorities_sub}
              </span>
            </div>
            <div class="flex flex-wrap gap-2 text-xs">
              {#each countryData[selectedCountry].legislation as b}
                <a
                  href={b.url}
                  target="_blank"
                  rel="noopener noreferrer"
                  title={b.title + calcStore.t.opens_legislation_link}
                  class="inline-flex items-center gap-1.5 px-2.5 py-1 rounded text-xs transition border group {b.primary
                    ? 'bg-blue-50 dark:bg-blue-950/60 text-blue-900 dark:text-blue-200 hover:bg-blue-100 dark:hover:bg-blue-900/60 hover:text-blue-950 dark:hover:text-white border-blue-300 dark:border-blue-700 font-bold'
                    : 'bg-slate-100 dark:bg-slate-800 text-slate-800 dark:text-slate-200 hover:bg-slate-200 dark:hover:bg-slate-700 hover:text-slate-950 dark:hover:text-white border-slate-300 dark:border-slate-700 font-semibold'}"
                >
                  <span>{b.key}</span>
                  <svg xmlns="http://www.w3.org/2000/svg" class="w-3 h-3 opacity-50 group-hover:opacity-100 transition-opacity" viewBox="0 0 20 20" fill="currentColor">
                    <path d="M11 3a1 1 0 100 2h2.586l-6.293 6.293a1 1 0 101.414 1.414L15 6.414V9a1 1 0 102 0V4a1 1 0 00-1-1h-5z" />
                    <path d="M5 5a2 2 0 00-2 2v8a2 2 0 002 2h8a2 2 0 002-2v-3a1 1 0 10-2 0v3H5V7h3a1 1 0 000-2H5z" />
                  </svg>
                </a>
              {/each}
            </div>
          </div>

          <!-- Section 1: Pre-Devolution UK Regulations Block -->
          <div class="bg-amber-50/70 dark:bg-amber-950/30 rounded-xl p-5 border border-amber-200 dark:border-amber-800/60 shadow-xs space-y-3">
            <div class="flex items-center justify-between border-b border-amber-200/80 dark:border-amber-800/50 pb-2">
              <div class="flex items-center gap-2">
                <span class="w-2.5 h-2.5 rounded-full bg-amber-500"></span>
                <h3 class="font-bold text-xs sm:text-sm text-amber-950 dark:text-amber-200">
                  {countryData[selectedCountry].preDevolution.heading}
                </h3>
              </div>
              <span class="text-[11px] font-mono px-2 py-0.5 rounded bg-amber-100 dark:bg-amber-900/60 text-amber-800 dark:text-amber-300 border border-amber-300 dark:border-amber-700">
                {countryData[selectedCountry].preDevolution.era}
              </span>
            </div>
            <p class="text-xs text-amber-900/90 dark:text-amber-200/90 leading-relaxed">
              {countryData[selectedCountry].preDevolution.overview}
            </p>
            <div class="grid grid-cols-1 md:grid-cols-2 gap-2 text-xs text-amber-950 dark:text-amber-100">
              {#each countryData[selectedCountry].preDevolution.rules as rule}
                <div class="flex items-start gap-2 bg-white/70 dark:bg-slate-900/60 p-2.5 rounded-lg border border-amber-200/60 dark:border-amber-800/40">
                  <span class="text-amber-600 dark:text-amber-400 font-bold">•</span>
                  <span class="leading-relaxed">{rule}</span>
                </div>
              {/each}
            </div>
          </div>

          <!-- Section 2: Summary Regulations & Current Regime (2023–2027) -->
          <div class="bg-white dark:bg-slate-900 rounded-xl p-5 border border-slate-200 dark:border-slate-800 shadow-xs space-y-4">
            <div class="flex items-center justify-between border-b border-slate-100 dark:border-slate-800 pb-2">
              <div class="flex items-center gap-2">
                <span class="w-2.5 h-2.5 rounded-full bg-blue-600"></span>
                <h3 class="font-bold text-xs sm:text-sm text-slate-900 dark:text-slate-100">
                  {countryData[selectedCountry].currentRegimeSummary.frameworkTitle}
                </h3>
              </div>
              <span class="text-xs font-medium text-slate-500 dark:text-slate-400">
                {countryData[selectedCountry].currentRegimeSummary.frameworkSub}
              </span>
            </div>

            <!-- Multiplier Architecture Grid -->
            <div>
              <h4 class="text-xs font-bold text-slate-800 dark:text-slate-200 mb-2">
                {countryData[selectedCountry].currentRegimeSummary.multiplierTitle}
              </h4>
              <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-2 text-xs font-mono">
                {#each countryData[selectedCountry].currentRegimeSummary.multipliers as m}
                  <div class="p-2.5 rounded-lg bg-slate-50 dark:bg-slate-800/60 border border-slate-200 dark:border-slate-700 flex flex-col justify-between">
                    <span class="text-[11px] font-sans font-semibold text-slate-600 dark:text-slate-400">{m.label}</span>
                    <div class="flex items-baseline justify-between mt-1">
                      <span class="text-sm font-bold text-blue-600 dark:text-blue-400">{m.rate}</span>
                      <span class="text-[10px] text-slate-500 dark:text-slate-400 font-sans">{m.rvThreshold}</span>
                    </div>
                  </div>
                {/each}
              </div>
            </div>

            <!-- Reliefs and Protections -->
            <div class="grid grid-cols-1 sm:grid-cols-2 gap-3 text-xs pt-1">
              {#each countryData[selectedCountry].currentRegimeSummary.reliefHighlights as rh}
                <div class="p-3 rounded-lg bg-slate-50/60 dark:bg-slate-800/40 border border-slate-100 dark:border-slate-800">
                  <span class="font-bold text-slate-900 dark:text-slate-100 block mb-0.5">{rh.title}:</span>
                  <span class="text-slate-600 dark:text-slate-300 leading-relaxed block">{rh.desc}</span>
                </div>
              {/each}
            </div>
          </div>

          <!-- Section 3: Country-Specific 8 Rating Lists Matrix (1990–2027) -->
          <div class="bg-white dark:bg-slate-900 rounded-xl border border-slate-200 dark:border-slate-800 shadow-xs overflow-hidden space-y-0">
            <div class="p-4 bg-slate-50/80 dark:bg-slate-800/60 border-b border-slate-200 dark:border-slate-800 flex items-center justify-between">
              <div>
                <h4 class="font-bold text-xs sm:text-sm text-slate-900 dark:text-slate-100">
                  {countryData[selectedCountry].name} — {calcStore.t.framework_timeline_title}
                </h4>
                <p class="text-[11px] text-slate-500 dark:text-slate-400 mt-0.5">
                  {calcStore.t.framework_timeline_sub}
                </p>
              </div>
              <span class="text-xs font-bold px-2.5 py-1 rounded-md bg-blue-50 dark:bg-blue-950 text-blue-700 dark:text-blue-300 border border-blue-200 dark:border-blue-800">
                {calcStore.t.framework_rating_lists_badge}
              </span>
            </div>

            <div class="overflow-x-auto">
              <table class="w-full text-left border-collapse text-xs">
                <thead>
                  <tr class="bg-slate-100 dark:bg-slate-800/80 text-slate-700 dark:text-slate-300 border-b border-slate-200 dark:border-slate-700 font-semibold">
                    <th class="p-3 whitespace-nowrap">{calcStore.t.col_rating_list}</th>
                    <th class="p-3 whitespace-nowrap">{calcStore.t.col_active_period}</th>
                    <th class="p-3">{countryData[selectedCountry].name} {calcStore.t.col_multipliers}</th>
                    <th class="p-3">{calcStore.t.col_sbrr_scheme}</th>
                    <th class="p-3">{calcStore.t.col_trans_empty}</th>
                    <th class="p-3 whitespace-nowrap">{calcStore.t.col_statute}</th>
                  </tr>
                </thead>
                <tbody class="divide-y divide-slate-100 dark:divide-slate-800 text-slate-600 dark:text-slate-300">
                  {#each countryData[selectedCountry].ratingLists as item}
                    <tr class="hover:bg-slate-50 dark:hover:bg-slate-800/40 transition">
                      <td class="p-3 font-bold text-slate-900 dark:text-slate-100 whitespace-nowrap">
                        {item.list}
                      </td>
                      <td class="p-3 font-mono text-[11px] whitespace-nowrap text-blue-600 dark:text-blue-400 font-medium">
                        {item.years}
                      </td>
                      <td class="p-3 min-w-[200px]">
                        {item.multipliers}
                      </td>
                      <td class="p-3 min-w-[220px]">
                        {item.sbrr}
                      </td>
                      <td class="p-3 min-w-[220px]">
                        {item.transition}
                      </td>
                      <td class="p-3 font-mono text-[10px] text-slate-500 dark:text-slate-400 whitespace-nowrap">
                        {item.statute}
                      </td>
                    </tr>
                  {/each}
                </tbody>
              </table>
            </div>
          </div>
        </div>
      {/key}
    </div>
  {/if}
</section>
