<script lang="ts">
  import { createGoal, formatIdr, type Goal } from "$lib/api";

  interface Props {
    goals: Goal[];
    onRefresh: () => void;
  }

  let { goals, onRefresh }: Props = $props();

  let showNewGoalModal = $state(false);
  let newTitle = $state("");
  let newTarget = $state("");
  let newDeadline = $state("2027-12-31");
  let newIsIbadah = $state(false);
  let saving = $state(false);

  async function handleCreateGoal() {
    if (!newTitle.trim()) return;
    const targetAmt = parseInt(newTarget.replace(/\D/g, ""), 10) || 0;
    if (targetAmt <= 0) return;

    saving = true;
    try {
      await createGoal({
        title: newTitle.trim(),
        target_amount: targetAmt,
        deadline_date: newDeadline,
        is_ibadah: newIsIbadah,
      });

      showNewGoalModal = false;
      newTitle = "";
      newTarget = "";
      onRefresh();
    } catch (e: any) {
      alert(`Gagal membuat impian: ${e}`);
    } finally {
      saving = false;
    }
  }
</script>

<div class="flex-1 flex flex-col h-full bg-[#0A0A0C] overflow-hidden">
  <!-- Topbar -->
  <div class="h-14 border-b border-[#272732] px-8 flex items-center justify-between bg-[#121217] shrink-0">
    <div>
      <h2 class="font-bold text-white text-sm">Target Tabungan & Rencana Ibadah</h2>
      <span class="text-[10px] text-[#9CA3AF]">Tracking dana darurat, qurban, pendidikan & haji keluarga</span>
    </div>

    <button
      onclick={() => (showNewGoalModal = true)}
      class="px-3.5 py-1.5 rounded-lg bg-[#10B981] hover:bg-[#059669] text-white text-xs font-bold shadow-md shadow-[#10B981]/30 transition-all"
    >
      + Tambah Target
    </button>
  </div>

  <!-- Goals Grid -->
  <div class="flex-1 overflow-y-auto p-6 md:p-8 max-w-6xl mx-auto w-full">
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
      {#each goals as g}
        <div class="bg-[#121217] border border-[#272732] rounded-2xl p-5 space-y-4 flex flex-col justify-between hover:border-[#10B981]/40 transition-colors">
          <div>
            <div class="flex items-center justify-between mb-2">
              <span class="text-[10px] uppercase font-bold px-2 py-0.5 rounded {g.is_ibadah ? 'bg-[#10B981]/20 text-[#34D399] border border-[#10B981]/30' : 'bg-[#18181F] text-[#9CA3AF] border border-[#272732]'}">
                {g.is_ibadah ? '🕌 Ibadah' : '🎯 Finansial'}
              </span>
              <span class="text-[10px] text-[#6B7280]">Target: {g.deadline_date}</span>
            </div>

            <h3 class="font-bold text-white text-base">{g.title}</h3>
            <div class="text-sm font-semibold text-[#10B981] mt-1">
              {formatIdr(g.current_amount)} <span class="text-xs text-[#6B7280]">/ {formatIdr(g.target_amount)}</span>
            </div>
          </div>

          <div class="space-y-1.5">
            <div class="w-full bg-[#18181F] h-2 rounded-full overflow-hidden border border-[#272732]">
              <div
                class="h-full bg-[#10B981] rounded-full transition-all"
                style="width: {Math.min(g.percentage, 100)}%"
              ></div>
            </div>
            <div class="flex items-center justify-between text-[11px] text-[#9CA3AF]">
              <span>Tercapai: {g.percentage.toFixed(1)}%</span>
              <span>Sisa: {formatIdr(Math.max(g.target_amount - g.current_amount, 0))}</span>
            </div>
          </div>
        </div>
      {/each}
    </div>
  </div>
</div>

<!-- Modal Create Goal -->
{#if showNewGoalModal}
  <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/75 backdrop-blur-sm p-4">
    <div class="bg-[#121217] border border-[#272732] rounded-2xl w-full max-w-md p-6 space-y-4 shadow-2xl">
      <div class="flex items-center justify-between border-b border-[#272732] pb-3">
        <h3 class="font-bold text-white text-sm">Target Tabungan Baru</h3>
        <button onclick={() => (showNewGoalModal = false)} class="text-[#9CA3AF] hover:text-white">✕</button>
      </div>

      <div class="space-y-3 text-xs">
        <div>
          <label for="new-goal-title-input" class="block text-[#D1D5DB] font-medium mb-1">Nama Target</label>
          <input
            id="new-goal-title-input"
            type="text"
            bind:value={newTitle}
            placeholder="Contoh: Qurban Kambing 2027, Dana Umrah..."
            class="w-full bg-[#18181F] border border-[#272732] rounded-lg px-3 py-2 text-white focus:border-[#10B981] focus:outline-none"
          />
        </div>

        <div>
          <label for="new-goal-target-input" class="block text-[#D1D5DB] font-medium mb-1">Target Nominal (Rp)</label>
          <input
            id="new-goal-target-input"
            type="number"
            bind:value={newTarget}
            placeholder="3500000"
            class="w-full bg-[#18181F] border border-[#272732] rounded-lg px-3 py-2 text-white focus:border-[#10B981] focus:outline-none"
          />
        </div>

        <div>
          <label for="new-goal-deadline-input" class="block text-[#D1D5DB] font-medium mb-1">Batas Waktu</label>
          <input
            id="new-goal-deadline-input"
            type="date"
            bind:value={newDeadline}
            class="w-full bg-[#18181F] border border-[#272732] rounded-lg px-3 py-2 text-white focus:border-[#10B981] focus:outline-none"
          />
        </div>

        <div class="flex items-center gap-2 pt-1">
          <input type="checkbox" id="is_ibadah" bind:checked={newIsIbadah} class="accent-[#10B981]" />
          <label for="is_ibadah" class="text-xs text-[#D1D5DB]">Tandai sebagai Target Ibadah (Qurban, Umrah, Haji, Aqiqah)</label>
        </div>
      </div>

      <div class="pt-3 border-t border-[#272732] flex items-center justify-end gap-2">
        <button onclick={() => (showNewGoalModal = false)} class="px-3 py-1.5 rounded text-xs text-[#9CA3AF] hover:bg-[#18181F]">Batal</button>
        <button onclick={handleCreateGoal} disabled={saving} class="px-4 py-1.5 rounded-lg bg-[#10B981] hover:bg-[#059669] text-white text-xs font-bold shadow-md shadow-[#10B981]/30">
          {saving ? "Menyimpan..." : "Simpan Target"}
        </button>
      </div>
    </div>
  </div>
{/if}
