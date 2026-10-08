<script lang="ts">
  import { onMount } from "svelte";
  import {
    getAiSettings,
    saveAiSettings,
    exportTransactions,
    type AiSettings,
  } from "$lib/api";
  import { cashStore } from "$lib/stores/cashStore.svelte";

  let ai = $state<AiSettings>({
    base_url: "http://localhost:20128/v1",
    api_key: "",
    model: "gpt-4o",
  });
  let saving = $state(false);
  let statusMsg = $state("");
  let exportPath = $state("");

  onMount(async () => {
    try {
      ai = await getAiSettings();
    } catch (e) {
      console.warn("Gagal membaca AI settings:", e);
    }
  });

  async function handleSave() {
    saving = true;
    statusMsg = "";
    try {
      await saveAiSettings(ai);
      statusMsg = "Pengaturan AI berhasil disimpan ke vault.";
    } catch (e) {
      statusMsg = `Gagal menyimpan: ${e}`;
    } finally {
      saving = false;
    }
  }

  async function handleExport() {
    try {
      const res = await exportTransactions();
      exportPath = res;
    } catch (e) {
      alert(`Gagal mengekspor CSV: ${e}`);
    }
  }
</script>

<div class="flex-1 overflow-y-auto p-6 md:p-8 space-y-6 max-w-4xl mx-auto w-full">
  <!-- Header -->
  <div>
    <h2 class="text-lg font-bold text-white flex items-center gap-2">
      <span>⚙️</span> Pengaturan & Keamanan Kas
    </h2>
    <p class="text-xs text-[#9CA3AF] mt-1">
      Konfigurasi local-first AI, zero-knowledge SQLCipher vault, dan ekspor data buku kas.
    </p>
  </div>

  <!-- Profil Anggota Aktif -->
  <div class="bg-[#18181F] border border-[#272732] rounded-xl p-5 space-y-4">
    <div class="flex items-center justify-between border-b border-[#272732] pb-3">
      <div>
        <h3 class="text-sm font-semibold text-white">Profil & Anggota Keluarga</h3>
        <p class="text-[11px] text-[#9CA3AF]">Kelola multi-profile & kontrol akses PIN anak</p>
      </div>
      <button
        type="button"
        onclick={() => (cashStore.showPinModal = true)}
        class="px-3 py-1.5 rounded-lg bg-[#272732] hover:bg-[#323240] text-white text-xs font-medium transition-colors cursor-pointer"
      >
        Ganti / Kunci Profil
      </button>
    </div>

    {#if cashStore.activeMember}
      <div class="flex items-center gap-3">
        <div class="w-10 h-10 rounded-full bg-[#10B981]/20 border border-[#10B981]/40 flex items-center justify-center text-sm font-bold text-[#34D399]">
          {cashStore.activeMember.display_name.charAt(0)}
        </div>
        <div>
          <span class="text-sm font-bold text-white block">{cashStore.activeMember.display_name}</span>
          <span class="text-xs text-[#10B981] capitalize block">Peran: {cashStore.activeMember.role}</span>
        </div>
      </div>
    {/if}
  </div>

  <!-- AI Financial Coach Settings -->
  <div class="bg-[#18181F] border border-[#272732] rounded-xl p-5 space-y-4">
    <div class="border-b border-[#272732] pb-3">
      <h3 class="text-sm font-semibold text-white flex items-center gap-2">
        <span>🤖</span> Integrasi AI Financial Coach
      </h3>
      <p class="text-[11px] text-[#9CA3AF]">
        Koneksi LLM via local proxy (9Router port 20128 atau Ollama / OpenAI-compatible endpoint).
      </p>
    </div>

    <div class="space-y-3 text-xs">
      <div>
        <label for="ai-base-url" class="block text-[#9CA3AF] mb-1 font-medium">Base URL</label>
        <input
          id="ai-base-url"
          type="text"
          bind:value={ai.base_url}
          placeholder="http://localhost:20128/v1"
          class="w-full bg-[#121217] border border-[#272732] focus:border-[#10B981] rounded-lg px-3 py-2 text-white outline-none"
        />
      </div>

      <div>
        <label for="ai-api-key" class="block text-[#9CA3AF] mb-1 font-medium">API Key (Opsional jika local)</label>
        <input
          id="ai-api-key"
          type="password"
          bind:value={ai.api_key}
          placeholder="sk-..."
          class="w-full bg-[#121217] border border-[#272732] focus:border-[#10B981] rounded-lg px-3 py-2 text-white outline-none"
        />
      </div>

      <div>
        <label for="ai-model" class="block text-[#9CA3AF] mb-1 font-medium">Model Identifier</label>
        <input
          id="ai-model"
          type="text"
          bind:value={ai.model}
          placeholder="gpt-4o"
          class="w-full bg-[#121217] border border-[#272732] focus:border-[#10B981] rounded-lg px-3 py-2 text-white outline-none"
        />
      </div>

      <div class="flex items-center justify-between pt-2">
        {#if statusMsg}
          <span class="text-xs {statusMsg.startsWith('Gagal') ? 'text-rose-400' : 'text-[#34D399]'}">
            {statusMsg}
          </span>
        {:else}
          <span></span>
        {/if}
        <button
          type="button"
          onclick={handleSave}
          disabled={saving}
          class="px-4 py-2 rounded-lg bg-[#10B981] hover:bg-[#059669] text-white text-xs font-bold transition-all disabled:opacity-50 cursor-pointer"
        >
          {saving ? "Menyimpan..." : "Simpan Pengaturan AI"}
        </button>
      </div>
    </div>
  </div>

  <!-- Zero-Knowledge Vault & Ekspor -->
  <div class="bg-[#18181F] border border-[#272732] rounded-xl p-5 space-y-4">
    <div class="border-b border-[#272732] pb-3">
      <h3 class="text-sm font-semibold text-white flex items-center gap-2">
        <span>🔒</span> Zero-Knowledge Vault & Ekspor CSV
      </h3>
      <p class="text-[11px] text-[#9CA3AF]">
        Seluruh data dienkripsi menggunakan SQLCipher di disk lokal. Tidak ada server eksternal penampung data keuangan Anda.
      </p>
    </div>

    <div class="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-3 pt-1">
      <div>
        <span class="text-xs text-white font-medium block">Backup Transaksi (CSV)</span>
        <span class="text-[11px] text-[#6B7280] block">Ekspor semua mutasi ke format spreadsheet lokal</span>
      </div>
      <button
        type="button"
        onclick={handleExport}
        class="px-3 py-2 rounded-lg bg-[#272732] hover:bg-[#323240] text-white text-xs font-semibold flex items-center gap-1.5 transition-colors cursor-pointer"
      >
        <svg class="w-3.5 h-3.5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path>
          <polyline points="7 10 12 15 17 10"></polyline>
          <line x1="12" y1="15" x2="12" y2="3"></line>
        </svg>
        Ekspor Buku Kas (.CSV)
      </button>
    </div>

    {#if exportPath}
      <div class="p-3 bg-[#121217] border border-[#10B981]/30 rounded-lg text-xs text-[#34D399]">
        File CSV berhasil disimpan ke: <span class="font-mono text-white">{exportPath}</span>
      </div>
    {/if}
  </div>
</div>
