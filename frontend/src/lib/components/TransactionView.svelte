<script lang="ts">
  import {
    deleteTransaction,
    exportTransactions,
    formatIdr,
    type Transaction,
  } from "$lib/api";

  interface Props {
    transactions: Transaction[];
    onRefresh: () => void;
    onQuickAdd: () => void;
  }

  let { transactions, onRefresh, onQuickAdd }: Props = $props();

  let searchQuery = $state("");
  let filterType = $state("all");
  let exportMessage = $state<string | null>(null);

  let filteredTxs = $derived(
    transactions.filter((t) => {
      const matchSearch =
        t.category_name.toLowerCase().includes(searchQuery.toLowerCase()) ||
        t.account_name.toLowerCase().includes(searchQuery.toLowerCase()) ||
        t.note.toLowerCase().includes(searchQuery.toLowerCase()) ||
        t.payee.toLowerCase().includes(searchQuery.toLowerCase());

      const matchType =
        filterType === "all" ||
        (filterType === "income" && t.tx_type === "income") ||
        (filterType === "expense" && ["expense", "nafkah", "sedekah", "zakat"].includes(t.tx_type));

      return matchSearch && matchType;
    })
  );

  async function handleDelete(id: string) {
    if (!confirm("Hapus transaksi ini? Saldo rekening akan otomatis disesuaikan.")) return;
    try {
      await deleteTransaction(id);
      onRefresh();
    } catch (e: any) {
      alert(`Gagal hapus: ${e}`);
    }
  }

  async function handleExportCsv() {
    exportMessage = "Mengekspor...";
    try {
      const path = await exportTransactions();
      exportMessage = `Berhasil diekspor: ${path}`;
      setTimeout(() => (exportMessage = null), 6000);
    } catch (e: any) {
      exportMessage = `Gagal ekspor: ${e}`;
    }
  }
</script>

<div class="flex-1 flex flex-col h-full bg-[#0A0A0C] overflow-hidden">
  <!-- Topbar -->
  <div class="h-14 border-b border-[#272732] px-8 flex items-center justify-between bg-[#121217] shrink-0">
    <div>
      <h2 class="font-bold text-white text-sm">Riwayat Transaksi Keuangan</h2>
      <span class="text-[10px] text-[#9CA3AF]">{filteredTxs.length} Transaksi Ditemukan</span>
    </div>

    <div class="flex items-center gap-2">
      <button
        onclick={handleExportCsv}
        class="px-3 py-1.5 rounded-lg border border-[#374151] hover:bg-[#18181F] text-[#D1D5DB] text-xs font-medium transition-colors"
      >
        📥 Ekspor CSV
      </button>
      <button
        onclick={onQuickAdd}
        class="px-3.5 py-1.5 rounded-lg bg-[#10B981] hover:bg-[#059669] text-white text-xs font-bold shadow-md shadow-[#10B981]/30 transition-all"
      >
        + Catat Cepat
      </button>
    </div>
  </div>

  {#if exportMessage}
    <div class="bg-[#18181F] border-b border-[#10B981]/40 px-8 py-2 text-xs text-[#34D399] flex items-center justify-between">
      <span>{exportMessage}</span>
      <button onclick={() => (exportMessage = null)} class="text-white hover:text-red-400">✕</button>
    </div>
  {/if}

  <!-- Filters & Search -->
  <div class="p-6 md:px-8 pb-3 max-w-6xl mx-auto w-full flex items-center justify-between gap-4">
    <input
      type="text"
      bind:value={searchQuery}
      placeholder="Cari transaksi, rekening, toko, atau catatan..."
      class="flex-1 bg-[#121217] border border-[#272732] rounded-xl px-4 py-2 text-xs text-white focus:border-[#10B981] focus:outline-none"
    />

    <div class="flex items-center gap-1 bg-[#121217] p-0.5 rounded-xl border border-[#272732] text-xs">
      <button
        onclick={() => (filterType = "all")}
        class="px-3 py-1 rounded-lg font-medium transition-colors {filterType === 'all' ? 'bg-[#10B981] text-white' : 'text-[#9CA3AF] hover:text-white'}"
      >
        Semua
      </button>
      <button
        onclick={() => (filterType = "income")}
        class="px-3 py-1 rounded-lg font-medium transition-colors {filterType === 'income' ? 'bg-[#10B981] text-white' : 'text-[#9CA3AF] hover:text-white'}"
      >
        Pemasukan
      </button>
      <button
        onclick={() => (filterType = "expense")}
        class="px-3 py-1 rounded-lg font-medium transition-colors {filterType === 'expense' ? 'bg-[#10B981] text-white' : 'text-[#9CA3AF] hover:text-white'}"
      >
        Pengeluaran
      </button>
    </div>
  </div>

  <!-- Transactions Table -->
  <div class="flex-1 overflow-y-auto px-6 md:px-8 pb-8 max-w-6xl mx-auto w-full">
    <div class="bg-[#121217] border border-[#272732] rounded-2xl overflow-hidden shadow-sm">
      <table class="w-full text-left text-xs">
        <thead class="bg-[#18181F] border-b border-[#272732] text-[#6B7280] text-[11px]">
          <tr>
            <th class="py-3 px-4">Tanggal</th>
            <th class="py-3 px-4">Kategori & Tipe</th>
            <th class="py-3 px-4">Rekening / Dompet</th>
            <th class="py-3 px-4">Anggota</th>
            <th class="py-3 px-4">Catatan / Pihak</th>
            <th class="py-3 px-4 text-right">Nominal</th>
            <th class="py-3 px-4 text-center">Aksi</th>
          </tr>
        </thead>
        <tbody class="divide-y divide-[#272732]">
          {#each filteredTxs as t}
            <tr class="hover:bg-[#18181F]/50 transition-colors">
              <td class="py-3 px-4 text-[#9CA3AF] text-[11px] font-mono">{t.date}</td>
              <td class="py-3 px-4">
                <span class="font-bold text-white block">{t.category_name}</span>
                <span class="text-[10px] text-[#6B7280] uppercase">{t.tx_type}</span>
              </td>
              <td class="py-3 px-4 text-[#D1D5DB]">{t.account_name}</td>
              <td class="py-3 px-4 text-[#9CA3AF]">{t.member_name}</td>
              <td class="py-3 px-4 text-[#9CA3AF] max-w-xs truncate">{t.note || t.payee || "-"}</td>
              <td class="py-3 px-4 text-right font-black {t.tx_type === 'income' ? 'text-[#10B981]' : 'text-white'}">
                {t.tx_type === 'income' ? '+' : '-'}{formatIdr(t.amount)}
              </td>
              <td class="py-3 px-4 text-center">
                <button
                  onclick={() => handleDelete(t.id)}
                  class="text-[#6B7280] hover:text-red-400 p-1 rounded transition-colors"
                  title="Hapus Transaksi"
                >
                  🗑️
                </button>
              </td>
            </tr>
          {/each}

          {#if filteredTxs.length === 0}
            <tr>
              <td colspan="7" class="py-12 text-center text-[#6B7280]">
                Tidak ada transaksi yang cocok dengan pencarian.
              </td>
            </tr>
          {/if}
        </tbody>
      </table>
    </div>
  </div>
</div>
