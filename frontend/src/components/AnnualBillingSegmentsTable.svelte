<script lang="ts">
  import { calcStore } from '../stores/calculation.svelte';

  const fmt = (v: number | undefined | null) =>
    '£' + Number(v || 0).toLocaleString('en-GB', { minimumFractionDigits: 2, maximumFractionDigits: 2 });
</script>

{#if !calcStore.response}
  <div class="p-8 text-center bg-slate-50 border border-dashed border-slate-200 rounded-xl text-slate-500 text-xs">
    <p>{calcStore.t.annual_placeholder}</p>
  </div>
{:else}
  <div class="space-y-6">
    <!-- Period Summary Cards -->
    <div class="bg-white p-4 rounded-xl border border-slate-200 shadow-sm space-y-3">
      <span class="text-xs font-bold text-slate-700 uppercase tracking-wider block">
        {calcStore.t.period_summary_label}
      </span>
      <div class="grid grid-cols-2 md:grid-cols-4 gap-3">
        <div class="bg-slate-50 p-3 rounded-lg border border-slate-100">
          <span class="text-[11px] font-semibold text-slate-500 block">{calcStore.t.sum_gross_label}</span>
          <span class="text-sm font-bold text-slate-900 font-mono mt-0.5 block">
            {fmt(calcStore.response.summary_financials.total_gross_liability)}
          </span>
        </div>
        <div class="bg-slate-50 p-3 rounded-lg border border-slate-100">
          <span class="text-[11px] font-semibold text-slate-500 block">{calcStore.t.sum_trans_label}</span>
          <span class="text-sm font-bold text-slate-900 font-mono mt-0.5 block">
            -{fmt(calcStore.response.summary_financials.total_transitional_adjustments)}
          </span>
        </div>
        <div class="bg-emerald-50 p-3 rounded-lg border border-emerald-100">
          <span class="text-[11px] font-semibold text-emerald-700 block">{calcStore.t.sum_reliefs_label}</span>
          <span class="text-sm font-bold text-emerald-800 font-mono mt-0.5 block">
            -{fmt(calcStore.response.summary_financials.total_reliefs_and_discounts)}
          </span>
        </div>
        <div class="bg-blue-50 p-3 rounded-lg border border-blue-200">
          <span class="text-[11px] font-semibold text-blue-700 block">{calcStore.t.sum_net_label}</span>
          <span class="text-base font-black text-blue-900 font-mono mt-0.5 block">
            {fmt(calcStore.response.summary_financials.total_net_payable)}
          </span>
        </div>
      </div>
    </div>

    <!-- Year-by-Year Segments List -->
    <div class="space-y-4">
      <h3 class="text-xs font-bold text-slate-900 uppercase tracking-wider">
        {calcStore.t.annual_breakdown_title}
      </h3>

      {#each calcStore.response.annual_billing_segments as seg, idx}
        <div class="bg-white border border-slate-200 rounded-xl p-5 shadow-sm space-y-4">
          <!-- Segment Header -->
          <div class="flex flex-wrap justify-between items-start gap-2 border-b border-slate-100 pb-3">
            <div>
              <div class="flex items-center gap-2">
                <span class="bg-blue-600 text-white font-bold text-xs px-2.5 py-0.5 rounded">
                  {calcStore.t.fy_badge} {seg.financial_year}
                </span>
                <span class="text-xs font-semibold text-slate-600">
                  {seg.segment_period.start_date} {calcStore.t.to_word} {seg.segment_period.end_date}
                </span>
                <span class="text-[11px] text-slate-400">
                  ({seg.segment_period.days} {calcStore.t.days_label})
                </span>
              </div>
              <div class="mt-2 flex flex-wrap items-center gap-2 text-xs text-slate-600">
                <span class="font-medium">{calcStore.t.rv_label}: <strong class="text-slate-900">{fmt(seg.rateable_value)}</strong></span>
                <span>•</span>
                <span class="font-medium">{calcStore.t.multiplier_label}: <strong class="text-slate-900">{seg.multiplier_used.rate}</strong> ({seg.multiplier_used.type})</span>
                {#if seg.occupancy_type}
                  <span>•</span>
                  <span class="bg-slate-100 text-slate-700 px-2 py-0.5 rounded font-mono text-[10px]">
                    {seg.occupancy_type}
                  </span>
                {/if}
              </div>
            </div>

            <!-- Net Segment Pill -->
            <div class="text-right">
              <span class="text-[10px] uppercase font-bold text-slate-500 block">{calcStore.t.year_net}</span>
              <span class="text-lg font-black text-slate-900 font-mono">
                {fmt(seg.net_segment_payable)}
              </span>
            </div>
          </div>

          <!-- Numbers Breakdown Grid -->
          <div class="grid grid-cols-2 sm:grid-cols-4 gap-2 text-xs">
            <div class="bg-slate-50 p-2 rounded">
              <span class="text-[10px] text-slate-500 block">{calcStore.t.year_gross}</span>
              <span class="font-bold text-slate-800 font-mono">{fmt(seg.gross_liability)}</span>
            </div>
            <div class="bg-slate-50 p-2 rounded">
              <span class="text-[10px] text-slate-500 block">{calcStore.t.trans_phasing}</span>
              <span class="font-bold text-slate-800 font-mono">
                {seg.transitional_adjustment.amount > 0 ? `-${fmt(seg.transitional_adjustment.amount)}` : '£0.00'}
              </span>
            </div>
            <div class="bg-slate-50 p-2 rounded">
              <span class="text-[10px] text-slate-500 block">{calcStore.t.year_reliefs}</span>
              <span class="font-bold text-emerald-700 font-mono">
                -{fmt(seg.itemized_reliefs.reduce((acc, r) => acc + r.amount_deducted, 0))}
              </span>
            </div>
            <div class="bg-blue-50/50 p-2 rounded border border-blue-100">
              <span class="text-[10px] text-blue-700 block">{calcStore.t.year_net}</span>
              <span class="font-black text-blue-950 font-mono">{fmt(seg.net_segment_payable)}</span>
            </div>
          </div>

          <!-- Itemized Reliefs & Penalties -->
          <div class="space-y-2 pt-2 border-t border-slate-100">
            <span class="text-xs font-bold text-slate-700 block">
              {calcStore.t.itemized_label}
            </span>

            {#if seg.itemized_reliefs.length === 0}
              <p class="text-xs text-slate-400 italic">
                {calcStore.t.no_reliefs}
              </p>
            {:else}
              <div class="space-y-1.5">
                {#each seg.itemized_reliefs as r}
                  <div class="flex flex-wrap justify-between items-center text-xs p-2 bg-emerald-50/60 border border-emerald-100 rounded">
                    <div>
                      <span class="font-bold text-emerald-950">[{r.relief_code}] {r.relief_name}</span>
                      <span class="text-emerald-700 font-medium ml-1">({r.discount_percentage}% {calcStore.t.discount_word})</span>
                      <span class="text-[10px] text-slate-500 block mt-0.5">{r.statutory_authority}</span>
                    </div>
                    <span class="font-bold font-mono text-emerald-900">
                      -{fmt(r.amount_deducted)}
                    </span>
                  </div>
                {/each}
              </div>
            {/if}

            {#if seg.statutory_penalties && seg.statutory_penalties.length > 0}
              <div class="space-y-1.5 mt-2">
                <span class="text-xs font-bold text-red-700 block">{calcStore.t.statutory_penalty}</span>
                {#each seg.statutory_penalties as p}
                  <div class="flex flex-wrap justify-between items-center text-xs p-2 bg-red-50/80 border border-red-200 rounded">
                    <div>
                      <span class="font-bold text-red-900">[{p.penalty_type}]</span>
                      <span class="text-[10px] text-red-700 block mt-0.5">{p.statutory_authority} — {p.calculation_formula}</span>
                    </div>
                    <span class="font-bold font-mono text-red-900">
                      +{fmt(p.amount)}
                    </span>
                  </div>
                {/each}
              </div>
            {/if}

            <!-- Statutory Payment Installment Schedule -->
            {#if seg.statutory_installments && seg.statutory_installments.length > 0}
              <div class="space-y-1.5 pt-2 border-t border-slate-100">
                <div class="flex items-center justify-between">
                  <span class="text-xs font-bold text-slate-700 block">
                    Statutory Payment Schedule ({seg.statutory_installments.length} Monthly Installments)
                  </span>
                  <span class="text-[10px] text-slate-500 font-medium">
                    SI 1989/1058 Sched 1 / LGA 2003
                  </span>
                </div>
                <div class="grid grid-cols-2 sm:grid-cols-5 md:grid-cols-6 gap-1.5">
                  {#each seg.statutory_installments as inst}
                    <div class="bg-slate-50 border border-slate-200/80 p-2 rounded text-center">
                      <span class="text-[10px] text-slate-500 font-semibold block">#{inst.installment_number} ({inst.due_date})</span>
                      <span class="font-bold font-mono text-xs text-slate-900 block mt-0.5">{fmt(inst.amount)}</span>
                    </div>
                  {/each}
                </div>
              </div>
            {:else if seg.net_segment_payable === 0}
              <div class="pt-2 border-t border-slate-100 flex flex-wrap items-center justify-between gap-1 text-xs text-slate-600 bg-emerald-50/50 p-2.5 rounded border border-emerald-100">
                <span class="font-bold text-emerald-950">Statutory Payment Schedule ({calcStore.qualifyingReliefFlags.payment_installments_count || 10} Monthly Installments selected):</span>
                <span class="font-semibold text-emerald-700">Net payable is £0.00 (100% relief applied — no payment installments required under SI 1989/1058 reg. 4)</span>
              </div>
            {/if}
          </div>
        </div>
      {/each}
    </div>
  </div>
{/if}

