<script lang="ts">
  import { calcStore } from '../stores/calculation.svelte';
  import type { SupportedLanguage } from '../types';

  const languages: { code: SupportedLanguage; label: string }[] = [
    { code: 'en', label: 'English' },
    { code: 'cy', label: 'Cymraeg' },
    { code: 'sco', label: 'Scots' },
    { code: 'gd', label: 'Gàidhlig' },
  ];
</script>

<header class="bg-slate-900 text-white shadow-lg sticky top-0 z-50">
  <div class="max-w-7xl mx-auto px-6 py-4 flex flex-col md:flex-row justify-between items-center gap-4">
    <div class="flex items-center space-x-3">
      <div class="w-9 h-9 rounded-lg bg-blue-600 flex items-center justify-center font-bold text-white shadow-inner">
        BR
      </div>
      <div>
        <h1 class="text-lg font-bold tracking-tight">{calcStore.t.app_name}</h1>
        <p class="text-xs text-slate-300">{calcStore.t.app_subtitle}</p>
      </div>
    </div>

    <div class="flex flex-wrap items-center gap-3">
      <!-- 4-Language Switcher -->
      <nav aria-label="Language selection" class="inline-flex rounded-lg border border-slate-700 bg-slate-800 p-0.5 text-xs font-semibold text-slate-300 shadow-inner">
        {#each languages as lang}
          <button
            type="button"
            lang={lang.code}
            onclick={() => calcStore.setLanguage(lang.code)}
            aria-pressed={calcStore.language === lang.code}
            aria-label={calcStore.t.switch_lang_aria + " " + lang.label}
            class="px-3 py-1.5 rounded-md transition focus:outline-hidden focus-visible:ring-2 focus-visible:ring-blue-400 focus-visible:ring-offset-2 focus-visible:ring-offset-slate-900 {calcStore.language === lang.code
              ? 'bg-white text-slate-900 font-bold shadow-sm'
              : 'text-slate-300 hover:text-white hover:bg-slate-700/50'}"
          >
            {lang.label}
          </button>
        {/each}
      </nav>

      <!-- Scalar API Reference Link -->
      <a
        href="/docs"
        target="_blank"
        rel="noopener noreferrer"
        class="inline-flex items-center px-3 py-1 rounded-full text-xs font-semibold bg-slate-800 hover:bg-slate-700 text-blue-300 hover:text-white border border-slate-700 transition"
        title={calcStore.t.scalar_docs_title}
      >
        {calcStore.t.scalar_docs_label}
      </a>

      <!-- API Status Pill -->
      <span class="inline-flex items-center px-3 py-1 rounded-full text-xs font-semibold bg-emerald-500/20 text-emerald-300 border border-emerald-500/30">
        <span class="w-2 h-2 rounded-full bg-emerald-400 mr-2 animate-pulse"></span>
        <span>{calcStore.t.api_status}</span>
      </span>
    </div>
  </div>
</header>

