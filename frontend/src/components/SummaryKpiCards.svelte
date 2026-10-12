<script lang="ts">
  import { calcStore } from '../stores/calculation.svelte';

  const badges = [
    {
      key: 'badge_lgfa',
      url: 'https://www.legislation.gov.uk/ukpga/1988/41/contents',
      title: 'Local Government Finance Act 1988 (c. 41)',
      primary: true,
    },
    {
      key: 'badge_ndra',
      url: 'https://www.legislation.gov.uk/ukpga/2023/53/contents/enacted',
      title: 'Non-Domestic Rating Act 2023 (c. 53)',
      primary: false,
    },
    {
      key: 'badge_lgfwa',
      url: 'https://www.legislation.gov.uk/asc/2024/6/contents/enacted',
      title: 'Local Government Finance (Wales) Act 2024 (asc 6)',
      primary: false,
    },
    {
      key: 'badge_ndr',
      url: 'https://www.legislation.gov.uk/asp/2020/4/contents',
      title: 'Non-Domestic Rates (Scotland) Act 2020 (asp 4) & Local Government (Scotland) Act 1975',
      primary: false,
    },
    {
      key: 'badge_installments',
      url: 'https://www.legislation.gov.uk/uksi/1989/1058/contents/made',
      title: 'The Non-Domestic Rating (Collection and Enforcement) (Local Lists) Regulations 1989 (S.I. 1989/1058)',
      primary: false,
    },
    {
      key: 'badge_wsi',
      url: 'https://www.legislation.gov.uk/wsi/2025/1371/contents/made',
      title: 'The Non-Domestic Rating (Chargeable Amounts) (Wales) Regulations 2025 (W.S.I. 2025/1371) & W.S.I. 2026/43',
      primary: false,
    },
  ] as const;
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
    {#each badges as b}
      <a
        href={b.url}
        target="_blank"
        rel="noopener noreferrer"
        title={b.title + " (opens on legislation.gov.uk)"}
        class="inline-flex items-center gap-1.5 px-2.5 py-1 rounded text-xs transition border group {b.primary
          ? 'bg-blue-50 text-blue-800 hover:bg-blue-100 hover:text-blue-900 border-blue-200 font-bold'
          : 'bg-slate-100 text-slate-700 hover:bg-slate-200 hover:text-slate-900 border-slate-200 font-medium'}"
      >
        <span>{calcStore.t[b.key]}</span>
        <svg xmlns="http://www.w3.org/2000/svg" class="w-3 h-3 opacity-50 group-hover:opacity-100 transition-opacity" viewBox="0 0 20 20" fill="currentColor">
          <path d="M11 3a1 1 0 100 2h2.586l-6.293 6.293a1 1 0 101.414 1.414L15 6.414V9a1 1 0 102 0V4a1 1 0 00-1-1h-5z" />
          <path d="M5 5a2 2 0 00-2 2v8a2 2 0 002 2h8a2 2 0 002-2v-3a1 1 0 10-2 0v3H5V7h3a1 1 0 000-2H5z" />
        </svg>
      </a>
    {/each}
  </div>
</section>

