<script lang="ts">
  import {
    calculateHealthScore,
    getTodayHadith,
    formatIdr,
    type Account,
    type EnvelopeItem,
    type FinancialHealthScore,
    type Hadith,
    type Member,
    type Transaction,
  } from "$lib/api";
  import { onMount } from "svelte";

  interface Props {
    activeMember: Member | null;
    accounts: Account[];
    transactions: Transaction[];
    envelopes: EnvelopeItem[];
    onQuickAdd: () => void;
  }

  let { activeMember, accounts, transactions, envelopes, onQuickAdd }: Props = $props();

  let healthScore = $state<FinancialHealthScore | null>(null);
  let todayHadith = $state<Hadith | null>(null);

  // Compute stats
  let totalBalance = $derived(
    accounts.reduce((acc, a) => acc + a.current_balance, 0)
  );

  let thisMonthIncome = $derived(
    transactions
      .filter((t) => t.tx_type === "income")
      .reduce((acc, t) => acc + t.amount, 0)
  );

  let thisMonthExpense = $derived(
    transactions
      .filter((t) => ["expense", "nafkah", "sedekah", "zakat"].includes(t.tx_type))
      .reduce((acc, t) => acc + t.amount, 0)
  );

  onMount(async () => {
    try {
      todayHadith = await getTodayHadith();
      healthScore = await calculateHealthScore(5.5, 12.0, 22.0, false);
    } catch (e) {
      console.error(e);
    }
  });
</script>

<div class="flex-1 overflow-y-auto p-6 md:p-8 max-w-6xl mx-auto w-full space-y-6">
  <!-- Priority Suggestion Banner -->
  {#if healthScore}
    <div class="bg-[#121217] border border-[#10B981]/40 rounded-xl p-4 flex items-center justify-between">
      <div class="flex items-center gap-3">
        <div class="w-9 h-9 rounded-lg bg-[#10B981]/15 border border-[#10B981]/30 flex items-center justify-center text-[#34D399] shrink-0">
          <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M12 2v4M12 18v4M4.93 4.93l2.83 2.83M16.24 16.24l2.83 2.83M2 12h4M18 12h4M4.93 19.07l2.83-2.83M16.24 7.76l2.83-2.83"/>
          </svg>
        </div>
        <div>
          <span class="text-[10px] uppercase font-bold text-[#34D399] tracking-wider block">Rekomendasi Kesehatan Finansial</span>
          <p class="text-xs text-[#E5E7EB] font-medium leading-relaxed">{healthScore.priority_suggestion}</p>
        </div>
      </div>
      <div class="hidden sm:flex flex-col items-end shrink-0 pl-4 border-l border-[#272732]">
        <span class="text-2xl font-black text-[#10B981]">{healthScore.total_score}<span class="text-xs text-[#6B7280]">/100</span></span>
        <span class="text-[9px] text-[#9CA3AF] uppercase">Skor Kesehatan</span>
      </div>
    </div>
  {/if}

  <!-- Top KPI Cards -->
  <div class="grid grid-cols-1 sm:grid-cols-3 gap-4">
    <!-- Total Harta Cair -->
    <div class="bg-[#18181F] border border-[#272732] rounded-xl p-5 space-y-1">
      <div class="flex items-center justify-between text-xs text-[#9CA3AF]">
        <span>Total Likuiditas Keluarga</span>
        <svg class="w-4 h-4 text-[#10B981]" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M3 21h18M3 10h18M5 6l7-3 7 3M4 10v11M20 10v11M8 14v4M12 14v4M16 14v4"/>
        </svg>
      </div>
      <div class="text-2xl font-black text-white tracking-tight">{formatIdr(totalBalance)}</div>
      <div class="text-[10px] text-[#6B7280]">{accounts.length} rekening & kas aktif</div>
    </div>

    <!-- Pemasukan Bulan Ini -->
    <div class="bg-[#18181F] border border-[#272732] rounded-xl p-5 space-y-1">
      <div class="flex items-center justify-between text-xs text-[#9CA3AF]">
        <span>Pemasukan Bulan Ini</span>
        <svg class="w-4 h-4 text-[#34D399]" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <polyline points="23 6 13.5 15.5 8.5 10.5 1 18"></polyline>
          <polyline points="17 6 23 6 23 12"></polyline>
        </svg>
      </div>
      <div class="text-2xl font-black text-[#10B981] tracking-tight">{formatIdr(thisMonthIncome)}</div>
      <div class="text-[10px] text-[#6B7280]">Gaji pokok & arus kas masuk</div>
    </div>

    <!-- Pengeluaran Bulan Ini -->
    <div class="bg-[#18181F] border border-[#272732] rounded-xl p-5 space-y-1">
      <div class="flex items-center justify-between text-xs text-[#9CA3AF]">
        <span>Pengeluaran & Belanja</span>
        <svg class="w-4 h-4 text-[#F87171]" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <polyline points="23 18 13.5 8.5 8.5 13.5 1 6"></polyline>
          <polyline points="17 18 23 18 23 12"></polyline>
        </svg>
      </div>
      <div class="text-2xl font-black text-[#F87171] tracking-tight">{formatIdr(thisMonthExpense)}</div>
      <div class="text-[10px] text-[#6B7280]">Kebutuhan pokok, nafkah & zakat</div>
    </div>
  </div>

  <!-- Middle Row: Envelopes Summary & Daily Hadith -->
  <div class="grid grid-cols-1 lg:grid-cols-12 gap-6 items-start">
    <!-- Left: Budget Envelopes Progress (7 cols) -->
    <div class="lg:col-span-7 bg-[#18181F] border border-[#272732] rounded-xl p-5 space-y-4">
      <div class="flex items-center justify-between">
        <div>
          <h3 class="font-bold text-white text-xs uppercase tracking-wider">Amplop Anggaran Bulanan</h3>
          <span class="text-[10px] text-[#6B7280]">Realisasi pengeluaran vs pagu belanja</span>
        </div>
        <span class="text-xs text-[#10B981] font-semibold">{envelopes.length} Kategori</span>
      </div>

      <div class="space-y-3 pt-1">
        {#each envelopes as env}
          <div class="space-y-1 text-xs">
            <div class="flex items-center justify-between">
              <span class="font-medium text-[#E5E7EB]">{env.category_name}</span>
              <span class="text-[#9CA3AF] text-[11px]">
                {formatIdr(env.spent_amount)} / <span class="text-white font-bold">{formatIdr(env.monthly_budget)}</span>
              </span>
            </div>
            <!-- Progress Bar -->
            <div class="w-full bg-[#121217] h-2 rounded-full overflow-hidden border border-[#272732]">
              <div
                class="h-full rounded-full transition-all {env.percentage_used >= 100 ? 'bg-[#EF4444]' : env.percentage_used >= 80 ? 'bg-[#F59E0B]' : 'bg-[#10B981]'}"
                style="width: {Math.min(env.percentage_used, 100)}%"
              ></div>
            </div>
          </div>
        {/each}
      </div>
    </div>

    <!-- Right: Daily Hadith Card (5 cols) -->
    <div class="lg:col-span-5 bg-[#18181F] border border-[#272732] rounded-xl p-5 flex flex-col justify-between space-y-4 relative overflow-hidden">
      <div>
        <div class="flex items-center justify-between text-xs text-[#10B981] mb-2 font-bold">
          <div class="flex items-center gap-1.5">
            <svg class="w-3.5 h-3.5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H20v20H6.5a2.5 2.5 0 0 1-2.5-2.5Z"></path>
            </svg>
            <span>Hadits Harian: Etika Harta</span>
          </div>
          <span class="text-[10px] px-2 py-0.5 rounded bg-[#10B981]/20">{todayHadith?.theme}</span>
        </div>

        {#if todayHadith}
          <!-- Arabic RTL text -->
          <div class="p-3 bg-[#0A0A0C] border border-[#272732] rounded-lg my-2 text-right font-serif text-sm text-[#A7F3D0] leading-loose" dir="rtl">
            {todayHadith.arabic}
          </div>

          <!-- Translation -->
          <p class="text-xs text-[#D1D5DB] leading-relaxed italic mt-3">
            "{todayHadith.translation_id}"
          </p>
          <span class="text-[10px] text-[#6B7280] block mt-2">— {todayHadith.narrator}</span>
        {/if}
      </div>

      <div class="pt-3 border-t border-[#272732] text-[10px] text-[#6B7280] flex items-center justify-between">
        <span>Koleksi 365 Hadits Pilihan</span>
        <span class="text-[#10B981]">Offline Database ✓</span>
      </div>
    </div>
  </div>

  <!-- Recent Transactions Table Preview -->
  <div class="bg-[#18181F] border border-[#272732] rounded-xl p-5 space-y-4">
    <div class="flex items-center justify-between">
      <h3 class="font-bold text-white text-xs uppercase tracking-wider">Transaksi Terkini</h3>
      <button onclick={onQuickAdd} class="px-3 py-1 bg-[#10B981] hover:bg-[#059669] text-white rounded-md text-xs font-bold transition-all shadow-sm">
        + Catat Cepat
      </button>
    </div>

    <div class="overflow-x-auto">
      <table class="w-full text-left text-xs">
        <thead>
          <tr class="border-b border-[#272732] text-[#6B7280] text-[11px]">
            <th class="pb-2">Tanggal</th>
            <th class="pb-2">Kategori</th>
            <th class="pb-2">Akun</th>
            <th class="pb-2">Catatan</th>
            <th class="pb-2 text-right">Nominal</th>
          </tr>
        </thead>
        <tbody class="divide-y divide-[#272732]">
          {#each transactions.slice(0, 6) as t}
            <tr class="hover:bg-[#121217] transition-colors">
              <td class="py-2.5 text-[#9CA3AF] text-[11px] font-mono">{t.date}</td>
              <td class="py-2.5 font-medium text-white">{t.category_name}</td>
              <td class="py-2.5 text-[#9CA3AF]">{t.account_name}</td>
              <td class="py-2.5 text-[#9CA3AF] truncate max-w-xs">{t.note || t.payee || "-"}</td>
              <td class="py-2.5 text-right font-bold {t.tx_type === 'income' ? 'text-[#10B981]' : 'text-white'}">
                {t.tx_type === 'income' ? '+' : '-'}{formatIdr(t.amount)}
              </td>
            </tr>
          {/each}
          {#if transactions.length === 0}
            <tr>
              <td colspan="5" class="py-6 text-center text-[#6B7280]">Belum ada transaksi yang tercatat.</td>
            </tr>
          {/if}
        </tbody>
      </table>
    </div>
  </div>
</div>
