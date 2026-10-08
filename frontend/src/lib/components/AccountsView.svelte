<script lang="ts">
  import { createAccount, formatIdr, type Account, type Member } from "$lib/api";

  interface Props {
    accounts: Account[];
    activeMember: Member | null;
    onRefresh: () => void;
  }

  let { accounts, activeMember, onRefresh }: Props = $props();

  let showNewAccountModal = $state(false);
  let newName = $state("");
  let newType = $state("bank");
  let newVisibility = $state("shared");
  let newOpening = $state("");
  let saving = $state(false);

  async function handleCreateAccount() {
    if (!newName.trim() || !activeMember) return;
    saving = true;
    try {
      const openingNum = parseInt(newOpening.replace(/\D/g, ""), 10) || 0;
      await createAccount({
        name: newName.trim(),
        acc_type: newType,
        visibility: newVisibility,
        opening_balance: openingNum,
        owner_member_id: activeMember.id,
      });

      showNewAccountModal = false;
      newName = "";
      newOpening = "";
      onRefresh();
    } catch (e: any) {
      alert(`Gagal membuat rekening: ${e}`);
    } finally {
      saving = false;
    }
  }
</script>

<div class="flex-1 flex flex-col h-full bg-[#0A0A0C] overflow-hidden">
  <!-- Topbar -->
  <div class="h-14 border-b border-[#272732] px-8 flex items-center justify-between bg-[#121217] shrink-0">
    <div>
      <h2 class="font-bold text-white text-sm">Rekening, Dompet & Kas Keluarga</h2>
      <span class="text-[10px] text-[#9CA3AF]">Pengelolaan saldo terkomputasi dan kepemilikan bersama</span>
    </div>

    <button
      onclick={() => (showNewAccountModal = true)}
      class="px-3.5 py-1.5 rounded-lg bg-[#10B981] hover:bg-[#059669] text-white text-xs font-bold shadow-md shadow-[#10B981]/30 transition-all"
    >
      + Tambah Akun
    </button>
  </div>

  <!-- Accounts Grid -->
  <div class="flex-1 overflow-y-auto p-6 md:p-8 max-w-6xl mx-auto w-full">
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
      {#each accounts as acc}
        <div class="bg-[#121217] border border-[#272732] rounded-2xl p-5 space-y-4 flex flex-col justify-between hover:border-[#10B981]/40 transition-colors">
          <div>
            <div class="flex items-center justify-between mb-2">
              <span class="text-[10px] uppercase font-bold text-[#10B981] px-2 py-0.5 rounded bg-[#10B981]/20 border border-[#10B981]/30">
                {acc.acc_type}
              </span>
              <span class="text-[10px] text-[#9CA3AF] px-2 py-0.5 rounded bg-[#18181F] border border-[#272732]">
                {acc.visibility === 'shared' ? '🤝 Bersama' : '🔒 Pribadi'}
              </span>
            </div>

            <h3 class="font-bold text-white text-base truncate">{acc.name}</h3>
            <span class="text-[11px] text-[#6B7280]">Saldo Awal: {formatIdr(acc.opening_balance)}</span>
          </div>

          <div class="pt-3 border-t border-[#272732]">
            <span class="text-[10px] text-[#9CA3AF] block uppercase">Saldo Terhitung Saat Ini:</span>
            <div class="text-2xl font-black text-white mt-0.5">
              {formatIdr(acc.current_balance)}
            </div>
          </div>
        </div>
      {/each}
    </div>
  </div>
</div>

<!-- Modal Add Account -->
{#if showNewAccountModal}
  <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/75 backdrop-blur-sm p-4">
    <div class="bg-[#121217] border border-[#272732] rounded-2xl w-full max-w-md p-6 space-y-4 shadow-2xl">
      <div class="flex items-center justify-between border-b border-[#272732] pb-3">
        <h3 class="font-bold text-white text-sm">Tambah Rekening / Dompet Baru</h3>
        <button onclick={() => (showNewAccountModal = false)} class="text-[#9CA3AF] hover:text-white">✕</button>
      </div>

      <div class="space-y-3.5 text-xs">
        <div>
          <label for="new-acc-name-input" class="block text-[#D1D5DB] font-medium mb-1">Nama Rekening / Dompet</label>
          <input
            id="new-acc-name-input"
            type="text"
            bind:value={newName}
            placeholder="Contoh: Tabungan Haji BSI, GoPay Ayah..."
            class="w-full bg-[#18181F] border border-[#272732] rounded-lg px-3 py-2 text-white focus:border-[#10B981] focus:outline-none"
          />
        </div>

        <div>
          <label for="new-acc-type-select" class="block text-[#D1D5DB] font-medium mb-1">Tipe Rekening</label>
          <select
            id="new-acc-type-select"
            bind:value={newType}
            class="w-full bg-[#18181F] border border-[#272732] rounded-lg px-3 py-2 text-white focus:border-[#10B981] focus:outline-none"
          >
            <option value="bank">Rekening Bank</option>
            <option value="ewallet">E-Wallet (GoPay, OVO, ShopeePay)</option>
            <option value="cash">Uang Tunai / Cash</option>
            <option value="credit_card">Kartu Kredit</option>
          </select>
        </div>

        <div>
          <label for="new-acc-vis-select" class="block text-[#D1D5DB] font-medium mb-1">Tingkat Visibilitas Keluarga</label>
          <select
            id="new-acc-vis-select"
            bind:value={newVisibility}
            class="w-full bg-[#18181F] border border-[#272732] rounded-lg px-3 py-2 text-white focus:border-[#10B981] focus:outline-none"
          >
            <option value="shared">Bersama (Terlihat seluruh dewasa)</option>
            <option value="private_summary">Ringkasan Saja (Rincian tersembunyi)</option>
            <option value="private">Pribadi (Hanya pemilik akun)</option>
          </select>
        </div>

        <div>
          <label for="new-acc-opening-input" class="block text-[#D1D5DB] font-medium mb-1">Saldo Awal</label>
          <input
            id="new-acc-opening-input"
            type="number"
            bind:value={newOpening}
            placeholder="0"
            class="w-full bg-[#18181F] border border-[#272732] rounded-lg px-3 py-2 text-white focus:border-[#10B981] focus:outline-none"
          />
        </div>
      </div>

      <div class="pt-3 border-t border-[#272732] flex items-center justify-end gap-2">
        <button onclick={() => (showNewAccountModal = false)} class="px-3 py-1.5 rounded text-xs text-[#9CA3AF] hover:bg-[#18181F]">Batal</button>
        <button onclick={handleCreateAccount} disabled={saving} class="px-4 py-1.5 rounded-lg bg-[#10B981] hover:bg-[#059669] text-white text-xs font-bold shadow-md shadow-[#10B981]/30">
          {saving ? "Menyimpan..." : "Simpan Akun"}
        </button>
      </div>
    </div>
  </div>
{/if}
