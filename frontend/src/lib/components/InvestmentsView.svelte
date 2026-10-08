<script lang="ts">
  import { formatIdr, type Debt, type Investment } from "$lib/api";

  interface Props {
    investments: Investment[];
    debts: Debt[];
    onRefresh: () => void;
  }

  let { investments, debts, onRefresh }: Props = $props();

  let totalInvestmentValue = $derived(
    investments.reduce((acc, i) => acc + i.current_value, 0)
  );

  let totalDebtsRemaining = $derived(
    debts.reduce((acc, d) => acc + d.remaining_amount, 0)
  );
</script>

<div class="flex-1 flex flex-col h-full bg-[#0A0A0C] overflow-hidden">
  <!-- Topbar -->
  <div class="h-14 border-b border-[#272732] px-8 flex items-center justify-between bg-[#121217] shrink-0">
    <div>
      <h2 class="font-bold text-white text-sm">Aset Investasi & Hutang Piutang</h2>
      <span class="text-[10px] text-[#9CA3AF]">Pantau kepemilikan emas fisik, instrumen investasi, dan pelunasan hutang</span>
    </div>

    <button onclick={onRefresh} class="px-3 py-1.5 rounded-lg border border-[#374151] hover:bg-[#18181F] text-xs text-[#D1D5DB]">
      🔄 Refresh
    </button>
  </div>

  <div class="flex-1 overflow-y-auto p-6 md:p-8 max-w-6xl mx-auto w-full space-y-6">
    <!-- Top Summary -->
    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
      <div class="bg-[#121217] border border-[#272732] rounded-2xl p-5 space-y-1">
        <span class="text-xs text-[#9CA3AF] uppercase">Total Nilai Investasi & Aset</span>
        <div class="text-3xl font-black text-[#10B981]">{formatIdr(totalInvestmentValue)}</div>
        <span class="text-[10px] text-[#6B7280]">Emas, saham, properti & reksa dana</span>
      </div>

      <div class="bg-[#121217] border border-[#272732] rounded-2xl p-5 space-y-1">
        <span class="text-xs text-[#9CA3AF] uppercase">Sisa Kewajiban Hutang</span>
        <div class="text-3xl font-black text-[#F87171]">{formatIdr(totalDebtsRemaining)}</div>
        <span class="text-[10px] text-[#6B7280]">KPR, kendaraan & pinjaman produktif</span>
      </div>
    </div>

    <!-- Investments List -->
    <div class="bg-[#121217] border border-[#272732] rounded-2xl p-5 space-y-4">
      <h3 class="font-bold text-white text-xs uppercase tracking-wider">Portofolio Aset & Emas</h3>
      <div class="overflow-x-auto">
        <table class="w-full text-left text-xs">
          <thead class="border-b border-[#272732] text-[#6B7280] text-[11px]">
            <tr>
              <th class="pb-2">Aset / Instrumen</th>
              <th class="pb-2">Jenis</th>
              <th class="pb-2">Unit / Gram</th>
              <th class="pb-2">Modal (Cost Basis)</th>
              <th class="pb-2 text-right">Nilai Sekarang</th>
              <th class="pb-2 text-right">Gain / Loss</th>
            </tr>
          </thead>
          <tbody class="divide-y divide-[#272732]">
            {#each investments as inv}
              <tr class="hover:bg-[#18181F]/50 transition-colors">
                <td class="py-3 font-bold text-white">{inv.title}</td>
                <td class="py-3 text-[11px] text-[#9CA3AF] uppercase">{inv.inv_type}</td>
                <td class="py-3 font-mono text-white">{inv.units}</td>
                <td class="py-3 text-[#9CA3AF]">{formatIdr(inv.cost_basis)}</td>
                <td class="py-3 text-right font-black text-white">{formatIdr(inv.current_value)}</td>
                <td class="py-3 text-right font-bold {inv.gain_loss >= 0 ? 'text-[#10B981]' : 'text-[#EF4444]'}">
                  {inv.gain_loss >= 0 ? '+' : ''}{formatIdr(inv.gain_loss)}
                </td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    </div>

    <!-- Debts List (With Riba Flag) -->
    <div class="bg-[#121217] border border-[#272732] rounded-2xl p-5 space-y-4">
      <div class="flex items-center justify-between">
        <h3 class="font-bold text-white text-xs uppercase tracking-wider">Daftar Hutang & Beban Kewajiban</h3>
        <span class="text-[10px] text-[#F59E0B] bg-[#F59E0B]/10 px-2 py-0.5 rounded border border-[#F59E0B]/20">Prioritas Dilunasi Cepat</span>
      </div>

      <div class="overflow-x-auto">
        <table class="w-full text-left text-xs">
          <thead class="border-b border-[#272732] text-[#6B7280] text-[11px]">
            <tr>
              <th class="pb-2">Hutang / Pinjaman</th>
              <th class="pb-2">Plafon Pokok</th>
              <th class="pb-2">Sisa Hutang</th>
              <th class="pb-2">Suku Bunga</th>
              <th class="pb-2">Status Syariah</th>
              <th class="pb-2">Jatuh Tempo</th>
            </tr>
          </thead>
          <tbody class="divide-y divide-[#272732]">
            {#each debts as d}
              <tr class="hover:bg-[#18181F]/50 transition-colors">
                <td class="py-3 font-bold text-white">{d.title}</td>
                <td class="py-3 text-[#9CA3AF]">{formatIdr(d.principal_amount)}</td>
                <td class="py-3 font-black text-red-400">{formatIdr(d.remaining_amount)}</td>
                <td class="py-3 text-[#9CA3AF]">{d.interest_rate}</td>
                <td class="py-3">
                  {#if d.is_riba}
                    <span class="px-2 py-0.5 rounded text-[10px] bg-red-500/20 text-red-400 border border-red-500/30 font-bold">
                      ⚠️ Mengandung Riba
                    </span>
                  {:else}
                    <span class="px-2 py-0.5 rounded text-[10px] bg-emerald-500/20 text-emerald-400 border border-emerald-500/30 font-bold">
                      ✓ Bebas Riba
                    </span>
                  {/if}
                </td>
                <td class="py-3 text-[11px] text-[#9CA3AF] font-mono">{d.due_date}</td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    </div>
  </div>
</div>
