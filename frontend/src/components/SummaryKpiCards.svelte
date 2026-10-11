<script lang="ts">
  import { calcStore } from '../stores/calculation.svelte';
</script>

<!-- Statutory List Coverage Banner -->
<section class="bg-white p-5 rounded-xl border border-slate-200 shadow-sm flex flex-col md:flex-row justify-between items-start md:items-center gap-4">
  <div>
    <span class="text-xs font-semibold text-slate-600 uppercase tracking-wider">
      {calcStore.t.statutory_coverage}
    </span>
    <h2 class="text-base font-bold text-slate-900 mt-0.5">
      {calcStore.t.statutory_coverage_sub}
    </h2>
  </div>
  <div class="flex flex-wrap gap-2 text-xs">
    <span class="bg-slate-100 text-slate-700 px-2.5 py-1 rounded font-medium border border-slate-200">
      {calcStore.t.badge_ndra}
    </span>
    <span class="bg-slate-100 text-slate-700 px-2.5 py-1 rounded font-medium border border-slate-200">
      {calcStore.t.badge_lgfwa}
    </span>
    <span class="bg-slate-100 text-slate-700 px-2.5 py-1 rounded font-medium border border-slate-200">
      {calcStore.t.badge_ndr}
    </span>
    <span class="bg-slate-100 text-slate-700 px-2.5 py-1 rounded font-medium border border-slate-200">
      {calcStore.t.badge_wsi}
    </span>
  </div>
</section>

