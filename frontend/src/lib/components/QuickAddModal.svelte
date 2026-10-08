<script lang="ts">
  import { createTransaction, type Account, type Member } from "$lib/api";

  interface Props {
    isOpen: boolean;
    accounts: Account[];
    activeMember: Member | null;
    onClose: () => void;
    onSuccess: () => void;
  }

  let { isOpen, accounts, activeMember, onClose, onSuccess }: Props = $props();

  let amountStr = $state("");
  let selectedAccountId = $state("");
  let selectedType = $state("expense");
  let selectedCategory = $state("cat-dapur");
  let note = $state("");
  let saving = $state(false);

  // Set default account when opened
  $effect(() => {
    if (accounts.length > 0 && !selectedAccountId) {
      selectedAccountId = accounts[0].id;
    }
  });

  const categories = [
    { id: "cat-dapur", name: "Dapur", icon: "🛒" },
    { id: "cat-tagihan", name: "Tagihan", icon: "⚡" },
    { id: "cat-transport", name: "Transport", icon: "⛽" },
    { id: "cat-nafkah", name: "Nafkah", icon: "❤️" },
    { id: "cat-sedekah", name: "Sedekah", icon: "🎁" },
    { id: "cat-pendidikan", name: "Sekolah", icon: "📚" },
    { id: "cat-gaji", name: "Gaji/Masuk", icon: "💰" },
  ];

  async function handleSave() {
    const rawAmt = parseInt(amountStr.replace(/\D/g, ""), 10);
    if (!rawAmt || rawAmt <= 0) {
      alert("Masukkan nominal transaksi!");
      return;
    }
    if (!activeMember || !selectedAccountId) return;

    saving = true;
    try {
      await createTransaction({
        account_id: selectedAccountId,
        member_id: activeMember.id,
        amount: rawAmt,
        tx_type: selectedCategory === "cat-gaji" ? "income" : selectedCategory === "cat-sedekah" ? "sedekah" : selectedCategory === "cat-nafkah" ? "nafkah" : "expense",
        category_id: selectedCategory,
        note: note.trim() || undefined,
      });

      amountStr = "";
      note = "";
      onSuccess();
      onClose();
    } catch (e: any) {
      alert(`Gagal mencatat transaksi: ${e}`);
    } finally {
      saving = false;
    }
  }

  function appendDigit(digit: string) {
    amountStr = `${amountStr}${digit}`;
  }

  function backspace() {
    amountStr = amountStr.slice(0, -1);
  }
</script>

{#if isOpen}
  <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/75 backdrop-blur-sm p-4">
    <div class="bg-[#121217] border border-[#272732] rounded-2xl w-full max-w-sm p-5 space-y-4 shadow-2xl">
      <div class="flex items-center justify-between border-b border-[#272732] pb-3">
        <h3 class="font-bold text-white text-sm flex items-center gap-2">
          <span>⚡</span> Catat Transaksi Cepat (&le; 5 Detik)
        </h3>
        <button onclick={onClose} class="text-[#9CA3AF] hover:text-white">✕</button>
      </div>

      <!-- Amount Display -->
      <div class="bg-[#18181F] border border-[#272732] rounded-xl p-3.5 text-center">
        <span class="text-[11px] text-[#9CA3AF] block mb-1">Nominal (Rupiah)</span>
        <div class="text-2xl font-black text-[#10B981] tracking-tight">
          Rp {amountStr ? parseInt(amountStr, 10).toLocaleString("id-ID") : "0"}
        </div>
      </div>

      <!-- Category Quick Chips -->
      <div>
        <label for="quick-add-category-label" id="quick-add-category-label" class="text-[10px] uppercase font-bold text-[#6B7280] block mb-1.5">Pilih Kategori:</label>
        <div class="grid grid-cols-4 gap-1.5 text-xs">
          {#each categories as cat}
            <button
              onclick={() => (selectedCategory = cat.id)}
              class="p-2 rounded-lg border text-center transition-all flex flex-col items-center gap-0.5 {selectedCategory === cat.id ? 'bg-[#10B981]/20 border-[#10B981] text-[#A7F3D0] font-bold' : 'bg-[#18181F] border-[#272732] text-[#9CA3AF] hover:text-white'}"
            >
              <span>{cat.icon}</span>
              <span class="text-[10px] truncate w-full">{cat.name}</span>
            </button>
          {/each}
        </div>
      </div>

      <!-- Account Selector -->
      <div>
        <label for="quick-account-select" class="text-[10px] uppercase font-bold text-[#6B7280] block mb-1">Dari Rekening / Dompet:</label>
        <select
          id="quick-account-select"
          bind:value={selectedAccountId}
          class="w-full bg-[#18181F] border border-[#272732] rounded-lg px-3 py-2 text-xs text-white focus:outline-none"
        >
          {#each accounts as acc}
            <option value={acc.id}>{acc.name} ({acc.currency})</option>
          {/each}
        </select>
      </div>

      <!-- Quick Numeric Keypad -->
      <div class="grid grid-cols-3 gap-1.5 pt-1">
        {#each ['1', '2', '3', '4', '5', '6', '7', '8', '9', '000', '0'] as key}
          <button
            onclick={() => appendDigit(key)}
            class="h-10 rounded-lg bg-[#18181F] hover:bg-[#272732] font-bold text-sm text-white active:scale-95 transition-transform"
          >
            {key}
          </button>
        {/each}
        <button
          onclick={backspace}
          class="h-10 rounded-lg bg-[#7F1D1D]/30 hover:bg-[#7F1D1D]/50 text-sm text-red-400 font-bold active:scale-95 transition-transform"
        >
          ⌫
        </button>
      </div>

      <!-- Save Button -->
      <button
        onclick={handleSave}
        disabled={saving || !amountStr}
        class="w-full py-2.5 rounded-xl bg-[#10B981] hover:bg-[#059669] text-white text-xs font-extrabold shadow-lg shadow-[#10B981]/30 transition-all disabled:opacity-50"
      >
        {saving ? "Menyimpan..." : "Simpan Transaksi ✓"}
      </button>
    </div>
  </div>
{/if}
