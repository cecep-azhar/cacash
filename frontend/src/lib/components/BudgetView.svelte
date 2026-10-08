<script lang="ts">
  import { formatIdr, type EnvelopeItem } from "$lib/api";

  interface Props {
    envelopes: EnvelopeItem[];
    onRefresh: () => void;
  }

  let { envelopes, onRefresh }: Props = $props();

  let totalBudget = $derived(
    envelopes.reduce((acc, e) => acc + e.monthly_budget, 0)
  );

  let totalSpent = $derived(
    envelopes.reduce((acc, e) => acc + e.spent_amount, 0)
  );

  let totalPercent = $derived(
    totalBudget > 0 ? (totalSpent / totalBudget) * 100 : 0
  );
</script>

<div class="flex-1 flex flex-col h-full bg-[#0A0A0C] overflow-hidden">
  <!-- Topbar -->
  <div class="h-14 border-b border-[#272732] px-8 flex items-center justify-between bg-[#121217] shrink-0">
    <div>
      <h2 class="font-bold text-white text-sm">Amplop Anggaran Bulanan (Envelope Budget)</h2>
      <span class="text-[10px] text-[#9CA3AF]">Kendalikan pos belanja keluarga secara terencana</span>
    </div>

    <button onclick={onRefresh} class="px-3 py-1.5 rounded-lg border border-[#374151] hover:bg-[#18181F] text-xs text-[#D1D5DB]">
      🔄 Refresh
    </button>
  </div>

  <div class="flex-1 overflow-y-auto p-6 md:p-8 max-w-6xl mx-auto w-full space-y-6">
    <!-- Overall Budget Card -->
    <div class="bg-[#121217] border border-[#272732] rounded-2xl p-6 space-y-3">
      <div class="flex items-center justify-between">
        <div>
          <span class="text-xs text-[#9CA3AF] uppercase tracking-wider block">Total Alokasi Anggaran Bulan Ini</span>
          <div class="text-3xl font-black text-white mt-1">
            {formatIdr(totalSpent)} <span class="text-sm font-normal text-[#6B7280]">/ {formatIdr(totalBudget)}</span>
          </div>
        </div>
        <div class="text-right">
          <span class="text-2xl font-black {totalPercent >= 100 ? 'text-[#EF4444]' : totalPercent >= 80 ? 'text-[#F59E0B]' : 'text-[#10B981]'}">
            {totalPercent.toFixed(1)}%
          </span>
          <span class="text-[10px] text-[#6B7280] block">Terpakai</span>
        </div>
      </div>

      <div class="w-full bg-[#18181F] h-3 rounded-full overflow-hidden border border-[#272732]">
        <div
          class="h-full rounded-full transition-all {totalPercent >= 100 ? 'bg-[#EF4444]' : totalPercent >= 80 ? 'bg-[#F59E0B]' : 'bg-[#10B981]'}"
          style="width: {Math.min(totalPercent, 100)}%"
        ></div>
      </div>
    </div>

    <!-- Envelopes Grid -->
    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
      {#each envelopes as env}
        <div class="bg-[#121217] border border-[#272732] rounded-2xl p-5 space-y-3 flex flex-col justify-between">
          <div>
            <div class="flex items-center justify-between mb-1.5">
              <span class="font-bold text-white text-sm">{env.category_name}</span>
              <span class="text-[10px] px-2 py-0.5 rounded font-bold {env.percentage_used >= 100 ? 'bg-red-500/20 text-red-400 border border-red-500/30' : env.percentage_used >= 80 ? 'bg-yellow-500/20 text-yellow-400 border border-yellow-500/30' : 'bg-emerald-500/20 text-emerald-400 border border-emerald-500/30'}">
                {env.percentage_used.toFixed(0)}%
              </span>
            </div>

            <div class="text-lg font-bold text-[#E5E7EB]">
              {formatIdr(env.spent_amount)}
              <span class="text-xs font-normal text-[#6B7280]">/ {formatIdr(env.monthly_budget)}</span>
            </div>
          </div>

          <div class="space-y-1.5">
            <div class="w-full bg-[#18181F] h-2 rounded-full overflow-hidden border border-[#272732]">
              <div
                class="h-full rounded-full transition-all {env.percentage_used >= 100 ? 'bg-[#EF4444]' : env.percentage_used >= 80 ? 'bg-[#F59E0B]' : 'bg-[#10B981]'}"
                style="width: {Math.min(env.percentage_used, 100)}%"
              ></div>
            </div>

            <div class="flex items-center justify-between text-[11px] text-[#6B7280]">
              <span>Sisa: {formatIdr(Math.max(env.monthly_budget - env.spent_amount, 0))}</span>
              <span>{env.percentage_used >= 100 ? '⚠️ Melebihi Pagu' : env.percentage_used >= 80 ? '⚠️ Hampir Habis' : 'Aman ✓'}</span>
            </div>
          </div>
        </div>
      {/each}
    </div>
  </div>
</div>
