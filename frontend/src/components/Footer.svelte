<script lang="ts">
  import { calcStore } from '../stores/calculation.svelte';
</script>

<footer class="mt-16 pt-8 pb-12 border-t border-slate-300 dark:border-slate-800 text-slate-800 dark:text-slate-200 transition-colors">
  <div class="max-w-7xl mx-auto px-4 sm:px-6 flex flex-col md:flex-row justify-between items-start md:items-center gap-6">
    <!-- Left: App metadata & navigation links -->
    <div class="space-y-2">
      <div class="flex items-center space-x-2">
        <span class="font-bold text-sm text-slate-900 dark:text-white">{calcStore.t.app_name}</span>
        <span class="text-slate-400 dark:text-slate-600">·</span>
        <span class="text-xs text-slate-700 dark:text-slate-300">{calcStore.t.statutory_coverage_sub}</span>
      </div>
      <nav aria-label="Footer links" class="flex flex-wrap items-center gap-4 text-xs">
        <a
          href="/docs"
          target="_blank"
          rel="noopener noreferrer"
          class="font-semibold text-blue-800 dark:text-blue-300 hover:text-blue-950 dark:hover:text-white underline decoration-slate-400 dark:decoration-slate-600 underline-offset-2 transition"
          title={calcStore.t.scalar_docs_title}
        >
          {calcStore.t.scalar_docs_label}
        </a>
        <span class="text-slate-400 dark:text-slate-600">·</span>
        <a
          href="/licenses"
          target="_blank"
          rel="noopener noreferrer"
          class="font-semibold text-blue-800 dark:text-blue-300 hover:text-blue-950 dark:hover:text-white underline decoration-slate-400 dark:decoration-slate-600 underline-offset-2 transition"
        >
          {calcStore.t.licenses_label}
        </a>
        <span class="text-slate-400 dark:text-slate-600">·</span>
        <a
          href="/api/v1/openapi.json"
          target="_blank"
          rel="noopener noreferrer"
          class="font-semibold text-blue-800 dark:text-blue-300 hover:text-blue-950 dark:hover:text-white underline decoration-slate-400 dark:decoration-slate-600 underline-offset-2 transition"
        >
          {calcStore.t.openapi_spec_label}
        </a>
      </nav>
    </div>

    <!-- Right: Deployment & Build Metadata -->
    <div class="flex flex-col sm:flex-row sm:items-center gap-3 text-xs bg-slate-100 dark:bg-slate-900/90 border border-slate-300 dark:border-slate-800 px-4 py-2.5 rounded-xl shadow-xs">
      <div class="flex items-center space-x-1.5">
        <span class="font-bold text-slate-900 dark:text-slate-100">{calcStore.t.version_label}</span>
        <span class="text-slate-800 dark:text-slate-200 font-semibold">v{calcStore.build.version}</span>
        <span class="inline-flex items-center px-1.5 py-0.5 rounded bg-blue-100 dark:bg-blue-950/80 text-blue-900 dark:text-blue-200 font-bold border border-blue-300 dark:border-blue-700/60 text-[11px]">
          {calcStore.t.build_number_label} #{calcStore.build.buildNumber}
        </span>
      </div>

      <span class="hidden sm:inline text-slate-400 dark:text-slate-600">·</span>

      <div class="flex items-center space-x-1.5">
        <span class="font-bold text-slate-900 dark:text-slate-100">{calcStore.t.commit_label}:</span>
        <code class="font-mono text-xs font-bold px-1.5 py-0.5 rounded bg-slate-200 dark:bg-slate-800 text-slate-900 dark:text-emerald-300 border border-slate-300 dark:border-slate-700">
          {calcStore.build.commitHash}
        </code>
      </div>

      <span class="hidden sm:inline text-slate-400 dark:text-slate-600">·</span>

      <div class="flex items-center space-x-1 text-slate-800 dark:text-slate-200">
        <span class="font-bold text-slate-900 dark:text-slate-100">{calcStore.t.built_timestamp_label}:</span>
        <time datetime={calcStore.build.timestamp} class="font-medium">
          {calcStore.build.formattedTimestamp}
        </time>
      </div>
    </div>
  </div>
</footer>

