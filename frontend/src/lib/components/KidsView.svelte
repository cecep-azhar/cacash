<script lang="ts">
  import {
    formatIdr,
    listKidMissions,
    toggleMission,
    type KidMission,
    type Member,
  } from "$lib/api";
  import { onMount } from "svelte";

  interface Props {
    members: Member[];
    activeMember: Member | null;
  }

  let { members, activeMember }: Props = $props();

  let childMember = $derived(
    members.find((m) => m.role === "child") || members[0]
  );

  let missions = $state<KidMission[]>([]);
  let piggyBankBalance = $state(125000); // Rp 125.000 saldo celengan Fathir

  onMount(async () => {
    if (childMember) {
      await loadMissions(childMember.id);
    }
  });

  async function loadMissions(childId: string) {
    try {
      missions = await listKidMissions(childId);
    } catch (e) {
      console.error(e);
    }
  }

  async function handleToggle(m: KidMission) {
    try {
      await toggleMission(m.id);
      if (childMember) await loadMissions(childMember.id);
    } catch (e: any) {
      alert(`Gagal: ${e}`);
    }
  }
</script>

<div class="flex-1 flex flex-col h-full bg-[#0A0A0C] overflow-hidden">
  <!-- Topbar -->
  <div class="h-14 border-b border-[#272732] px-8 flex items-center justify-between bg-[#121217] shrink-0">
    <div>
      <h2 class="font-bold text-white text-sm">Edukasi & Celengan Finansial Anak</h2>
      <span class="text-[10px] text-[#9CA3AF]">Membangun karakter hemat, jujur, dan gemar berbagi sejak dini</span>
    </div>

    <span class="text-xs text-[#10B981] font-bold px-3 py-1 rounded-full bg-[#10B981]/10 border border-[#10B981]/30">
      Profil Anak: {childMember?.display_name || 'Fathir'}
    </span>
  </div>

  <div class="flex-1 overflow-y-auto p-6 md:p-8 max-w-6xl mx-auto w-full space-y-6">
    <!-- Piggy Bank Card -->
    <div class="bg-gradient-to-br from-[#121217] to-[#18181F] border border-[#272732] rounded-2xl p-6 flex items-center justify-between">
      <div class="flex items-center gap-4">
        <div class="w-16 h-16 rounded-2xl bg-gradient-to-tr from-[#10B981] to-[#34D399] flex items-center justify-center text-3xl shadow-lg shadow-[#10B981]/30">
          🐷
        </div>
        <div>
          <span class="text-[10px] uppercase font-bold text-[#9CA3AF] tracking-wider block">Saldo Celengan Mandiri</span>
          <div class="text-3xl font-black text-white mt-0.5">{formatIdr(piggyBankBalance)}</div>
          <span class="text-xs text-[#34D399] font-medium">+Rp 25.000 dari misi pekan ini</span>
        </div>
      </div>

      <div class="hidden sm:flex flex-col items-end gap-1">
        <span class="text-xs text-[#9CA3AF]">Status Literasi:</span>
        <span class="px-3 py-1 rounded-full bg-[#10B981]/20 text-[#34D399] text-xs font-bold border border-[#10B981]/30">
          🌟 Calon Saudagar Beriman
        </span>
      </div>
    </div>

    <!-- Missions & Quests -->
    <div class="bg-[#121217] border border-[#272732] rounded-2xl p-5 space-y-4">
      <div class="flex items-center justify-between">
        <div>
          <h3 class="font-bold text-white text-xs uppercase tracking-wider">Misi & Tantangan Kebaikan</h3>
          <span class="text-[10px] text-[#6B7280]">Selesaikan misi untuk mendapatkan reward uang saku & bintang</span>
        </div>
        <span class="text-xs text-[#10B981] font-semibold">{missions.filter((m) => m.is_completed).length}/{missions.length} Selesai</span>
      </div>

      <div class="space-y-2 pt-1">
        {#each missions as m}
          <div class="p-3.5 rounded-xl bg-[#18181F] border border-[#272732] flex items-center justify-between hover:border-[#10B981]/40 transition-colors">
            <div class="flex items-center gap-3">
              <button
                onclick={() => handleToggle(m)}
                class="w-6 h-6 rounded-lg border flex items-center justify-center transition-colors {m.is_completed ? 'bg-[#10B981] border-[#10B981] text-white font-bold' : 'border-[#374151] hover:border-[#10B981]'}"
              >
                {m.is_completed ? '✓' : ''}
              </button>
              <div>
                <span class="text-xs font-medium {m.is_completed ? 'text-[#6B7280] line-through' : 'text-white'}">
                  {m.title}
                </span>
                <span class="text-[10px] text-[#6B7280] block">Reward Celengan: {formatIdr(m.reward_amount)}</span>
              </div>
            </div>

            <span class="text-[10px] px-2.5 py-1 rounded-full font-bold {m.is_completed ? 'bg-[#064E3B]/40 text-[#34D399] border border-[#059669]' : 'bg-[#1F2937] text-[#9CA3AF]'}">
              {m.is_completed ? 'Telah Diberikan ✓' : 'Belum Selesai'}
            </span>
          </div>
        {/each}
      </div>
    </div>

    <!-- Badges & Achievements -->
    <div class="bg-[#121217] border border-[#272732] rounded-2xl p-5 space-y-4">
      <h3 class="font-bold text-white text-xs uppercase tracking-wider">Lencana Prestasi (Badges)</h3>
      <div class="grid grid-cols-2 sm:grid-cols-4 gap-3 text-center">
        <div class="p-4 rounded-xl bg-[#18181F] border border-[#10B981]/40 space-y-1">
          <div class="text-3xl">🥇</div>
          <span class="font-bold text-xs text-white block">Juara Sedekah</span>
          <span class="text-[10px] text-[#6B7280]">Rutinitas sedekah subuh 7 hari berturut-turut</span>
        </div>

        <div class="p-4 rounded-xl bg-[#18181F] border border-[#10B981]/40 space-y-1">
          <div class="text-3xl">🎯</div>
          <span class="font-bold text-xs text-white block">Target Pertama</span>
          <span class="text-[10px] text-[#6B7280]">Nabung beli buku cerita dari uang saku</span>
        </div>

        <div class="p-4 rounded-xl bg-[#18181F] border border-[#272732] opacity-60 space-y-1">
          <div class="text-3xl">🛡️</div>
          <span class="font-bold text-xs text-white block">Anti-Mubazir</span>
          <span class="text-[10px] text-[#6B7280]">14 hari tanpa jajan di luar anggaran</span>
        </div>

        <div class="p-4 rounded-xl bg-[#18181F] border border-[#272732] opacity-60 space-y-1">
          <div class="text-3xl">👑</div>
          <span class="font-bold text-xs text-white block">Amanah Emas</span>
          <span class="text-[10px] text-[#6B7280]">Lunas pinjaman pensil warna keluarga</span>
        </div>
      </div>
    </div>
  </div>
</div>
