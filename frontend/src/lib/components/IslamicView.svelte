<script lang="ts">
  import {
    calculateZakatFitrah,
    calculateZakatMal,
    formatIdr,
    getTodayHadith,
    type Hadith,
    type ZakatFitrahCalculation,
    type ZakatMalCalculation,
  } from "$lib/api";
  import { onMount } from "svelte";

  let totalAssetsInput = $state(150000000); // 150 jt
  let debtsInput = $state(10000000); // 10 jt hutang jatuh tempo
  let goldPriceInput = $state(1400000); // Rp 1.400.000 / gram Antam

  let membersCountInput = $state(4); // 4 orang keluarga
  let ricePriceInput = $state(45000); // Rp 45.000 / jiwa

  let zakatMalRes = $state<ZakatMalCalculation | null>(null);
  let zakatFitrahRes = $state<ZakatFitrahCalculation | null>(null);
  let hadith = $state<Hadith | null>(null);

  onMount(async () => {
    recalcMal();
    recalcFitrah();
    try {
      hadith = await getTodayHadith();
    } catch (e) {
      console.error(e);
    }
  });

  async function recalcMal() {
    try {
      zakatMalRes = await calculateZakatMal(totalAssetsInput, debtsInput, goldPriceInput);
    } catch (e) {
      console.error(e);
    }
  }

  async function recalcFitrah() {
    try {
      zakatFitrahRes = await calculateZakatFitrah(membersCountInput, ricePriceInput);
    } catch (e) {
      console.error(e);
    }
  }
</script>

<div class="flex-1 flex flex-col h-full bg-[#0A0A0C] overflow-hidden">
  <!-- Topbar -->
  <div class="h-14 border-b border-[#272732] px-8 flex items-center justify-between bg-[#121217] shrink-0">
    <div>
      <h2 class="font-bold text-white text-sm flex items-center gap-2">
        <span>🕌</span> Modul Syariah & Kalkulator Zakat
      </h2>
      <span class="text-[10px] text-[#9CA3AF]">Penghitungan Zakat Mal (Nishab 85g Emas), Fitrah, Nafkah & Sedekah</span>
    </div>

    <span class="px-3 py-1 rounded-full text-xs font-bold bg-[#10B981]/20 text-[#34D399] border border-[#10B981]/30">
      Mode Syariah Aktif ✓
    </span>
  </div>

  <div class="flex-1 overflow-y-auto p-6 md:p-8 max-w-6xl mx-auto w-full space-y-6">
    <!-- Daily Hadith Card Highlight -->
    {#if hadith}
      <div class="bg-gradient-to-br from-[#121217] to-[#18181F] border border-[#10B981]/40 rounded-2xl p-6 space-y-3">
        <div class="flex items-center justify-between text-xs text-[#10B981] font-bold">
          <span>📜 Hadits Hari Ini: {hadith.theme}</span>
          <span class="text-[10px] text-[#9CA3AF]">Riwayat {hadith.narrator}</span>
        </div>

        <div class="p-4 bg-[#0A0A0C] border border-[#272732] rounded-xl text-right font-serif text-base text-[#A7F3D0] leading-loose" dir="rtl">
          {hadith.arabic}
        </div>

        <p class="text-xs text-[#E5E7EB] italic leading-relaxed">
          "{hadith.translation_id}"
        </p>
      </div>
    {/if}

    <!-- Zakat Mal Calculator -->
    <div class="bg-[#121217] border border-[#272732] rounded-2xl p-6 space-y-4">
      <div class="flex items-center justify-between border-b border-[#272732] pb-3">
        <div>
          <h3 class="font-bold text-white text-sm">Kalkulator Zakat Mal (Harta)</h3>
          <span class="text-[10px] text-[#6B7280]">Kaidah Fiqih: Nishab 85 gram emas murni & Haul 1 tahun Hijriah (2.5%)</span>
        </div>
        <span class="text-xs text-[#10B981] font-mono font-bold">Kadar: 2.5%</span>
      </div>

      <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
        <div>
          <label for="zakat-total-assets-input" class="block text-xs text-[#9CA3AF] mb-1">Total Harta Kena Zakat (Rp)</label>
          <input
            id="zakat-total-assets-input"
            type="number"
            bind:value={totalAssetsInput}
            oninput={recalcMal}
            class="w-full bg-[#18181F] border border-[#272732] rounded-lg px-3 py-2 text-xs text-white focus:border-[#10B981] focus:outline-none"
          />
          <span class="text-[10px] text-[#6B7280]">Tabungan + Emas + Saham + Piutang lancar</span>
        </div>

        <div>
          <label for="zakat-debts-input" class="block text-xs text-[#9CA3AF] mb-1">Hutang Jatuh Tempo / Mendesak (Rp)</label>
          <input
            id="zakat-debts-input"
            type="number"
            bind:value={debtsInput}
            oninput={recalcMal}
            class="w-full bg-[#18181F] border border-[#272732] rounded-lg px-3 py-2 text-xs text-white focus:border-[#10B981] focus:outline-none"
          />
          <span class="text-[10px] text-[#6B7280]">Pengurang kewajiban jatuh tempo</span>
        </div>

        <div>
          <label for="zakat-gold-price-input" class="block text-xs text-[#9CA3AF] mb-1">Harga 1 Gram Emas Saat Ini (Rp)</label>
          <input
            id="zakat-gold-price-input"
            type="number"
            bind:value={goldPriceInput}
            oninput={recalcMal}
            class="w-full bg-[#18181F] border border-[#272732] rounded-lg px-3 py-2 text-xs text-white focus:border-[#10B981] focus:outline-none"
          />
          <span class="text-[10px] text-[#6B7280]">Nishab 85g: {formatIdr(goldPriceInput * 85)}</span>
        </div>
      </div>

      <!-- Result Mal -->
      {#if zakatMalRes}
        <div class="p-4 rounded-xl border mt-2 flex flex-col md:flex-row items-center justify-between gap-4 {zakatMalRes.is_eligible ? 'bg-[#064E3B]/20 border-[#059669]' : 'bg-[#18181F] border-[#272732]'}">
          <div>
            <div class="flex items-center gap-2">
              <span class="text-xs font-bold {zakatMalRes.is_eligible ? 'text-[#34D399]' : 'text-[#9CA3AF]'}">
                {zakatMalRes.is_eligible ? '✓ Wajib Zakat Mal (Telah Melebihi Nishab)' : 'Belum Mencapai Nishab (Tidak Wajib)'}
              </span>
            </div>
            <span class="text-[11px] text-[#9CA3AF]">
              Harta Bersih: {formatIdr(zakatMalRes.net_wealth)} | Batas Nishab: {formatIdr(zakatMalRes.nishab_threshold)}
            </span>
          </div>

          <div class="text-right">
            <span class="text-[10px] text-[#9CA3AF] uppercase block">Kewajiban Zakat yang Harus Dikeluarkan:</span>
            <div class="text-2xl font-black {zakatMalRes.is_eligible ? 'text-[#10B981]' : 'text-white'}">
              {formatIdr(zakatMalRes.zakat_due)}
            </div>
          </div>
        </div>
      {/if}
    </div>

    <!-- Zakat Fitrah Calculator -->
    <div class="bg-[#121217] border border-[#272732] rounded-2xl p-6 space-y-4">
      <div class="flex items-center justify-between border-b border-[#272732] pb-3">
        <div>
          <h3 class="font-bold text-white text-sm">Kalkulator Zakat Fitrah</h3>
          <span class="text-[10px] text-[#6B7280]">Kewajiban per jiwa keluarga di bulan Ramadhan (2.5 kg beras / jiwa)</span>
        </div>
      </div>

      <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        <div>
          <label for="zakat-fitrah-members-input" class="block text-xs text-[#9CA3AF] mb-1">Jumlah Jiwa Tanggungan Keluarga</label>
          <input
            id="zakat-fitrah-members-input"
            type="number"
            bind:value={membersCountInput}
            oninput={recalcFitrah}
            class="w-full bg-[#18181F] border border-[#272732] rounded-lg px-3 py-2 text-xs text-white focus:border-[#10B981] focus:outline-none"
          />
        </div>

        <div>
          <label for="zakat-fitrah-rice-price-input" class="block text-xs text-[#9CA3AF] mb-1">Standar Zakat Fitrah Uang / Orang (Rp)</label>
          <input
            id="zakat-fitrah-rice-price-input"
            type="number"
            bind:value={ricePriceInput}
            oninput={recalcFitrah}
            class="w-full bg-[#18181F] border border-[#272732] rounded-lg px-3 py-2 text-xs text-white focus:border-[#10B981] focus:outline-none"
          />
        </div>
      </div>

      {#if zakatFitrahRes}
        <div class="p-4 rounded-xl bg-[#18181F] border border-[#272732] flex items-center justify-between">
          <span class="text-xs text-[#D1D5DB]">{zakatFitrahRes.household_members_count} Anggota Keluarga ({formatIdr(zakatFitrahRes.price_per_person)} / jiwa)</span>
          <div class="text-right">
            <span class="text-[10px] text-[#9CA3AF] uppercase block">Total Zakat Fitrah:</span>
            <div class="text-xl font-black text-[#10B981]">{formatIdr(zakatFitrahRes.total_due)}</div>
          </div>
        </div>
      {/if}
    </div>
  </div>
</div>
