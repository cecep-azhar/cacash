<script lang="ts">
  import { windowMinimize, windowMaximize, windowClose, type Member } from "$lib/api";

  interface Props {
    activeMember?: Member | null;
    onSwitchProfile?: () => void;
    onQuickAdd?: () => void;
    onOpenSettings?: () => void;
  }

  let { activeMember, onSwitchProfile, onQuickAdd, onOpenSettings }: Props = $props();
</script>

<div
  data-tauri-drag-region
  class="h-10 bg-[#0A0A0C] border-b border-[#272732] flex items-center justify-between px-3 select-none text-xs text-[#9CA3AF] z-50 shrink-0"
>
  <!-- Logo & Household Brand -->
  <div class="flex items-center gap-2.5">
    <div class="w-5 h-5 rounded-md bg-gradient-to-br from-[#10B981] to-[#059669] flex items-center justify-center shadow-[0_0_10px_rgba(16,185,129,0.5)]">
      <svg class="w-3.5 h-3.5 text-white" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
        <path d="M12 2L2 7l10 5 10-5-10-5z" />
        <path d="M2 17l10 5 10-5" />
        <path d="M2 12l10 5 10-5" />
      </svg>
    </div>
    <span class="font-bold tracking-wider text-white text-sm">CACASH</span>
    <span class="text-[#4B5563]">/</span>
    <span class="text-[#D1D5DB] font-medium text-[11px]">Keluarga Mandiri</span>

    {#if activeMember}
      <button
        onclick={onSwitchProfile}
        class="ml-2 px-2 py-0.5 rounded-full bg-[#18181F] hover:bg-[#272732] border border-[#272732] flex items-center gap-1.5 text-[11px] text-[#A7F3D0] transition-colors"
        title="Ganti Profil Anggota"
      >
        <span class="w-2 h-2 rounded-full bg-[#10B981]"></span>
        <span>{activeMember.display_name}</span>
        <span class="text-[9px] text-[#6B7280]">▼</span>
      </button>
    {/if}
  </div>

  <!-- Center Quick Actions -->
  <div class="hidden md:flex items-center gap-2">
    {#if onQuickAdd}
      <button
        onclick={onQuickAdd}
        class="px-2.5 py-0.5 rounded bg-[#10B981] hover:bg-[#059669] text-white text-[11px] font-bold flex items-center gap-1 shadow-sm transition-all"
        title="Catat Transaksi Cepat (<= 5 detik)"
      >
        <span>+</span> Catat Transaksi
      </button>
    {/if}
  </div>

  <!-- Right Actions & Window Controls -->
  <div class="flex items-center gap-1">
    {#if onOpenSettings}
      <button
        onclick={onOpenSettings}
        title="Pengaturan Mode & AI"
        class="p-1.5 hover:bg-[#1E1E28] rounded text-[#9CA3AF] hover:text-white transition-colors mr-2"
      >
        <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <circle cx="12" cy="12" r="3"></circle>
          <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"></path>
        </svg>
      </button>
    {/if}

    <!-- Minimize -->
    <button
      onclick={windowMinimize}
      class="p-1.5 hover:bg-[#1E1E28] rounded text-[#9CA3AF] hover:text-white transition-colors"
      title="Minimize"
    >
      <svg class="w-3.5 h-3.5" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
        <line x1="5" y1="12" x2="19" y2="12"></line>
      </svg>
    </button>

    <!-- Maximize -->
    <button
      onclick={windowMaximize}
      class="p-1.5 hover:bg-[#1E1E28] rounded text-[#9CA3AF] hover:text-white transition-colors"
      title="Maximize"
    >
      <svg class="w-3.5 h-3.5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <rect x="3" y="3" width="18" height="18" rx="2" ry="2"></rect>
      </svg>
    </button>

    <!-- Close -->
    <button
      onclick={windowClose}
      class="p-1.5 hover:bg-[#EF4444] rounded text-[#9CA3AF] hover:text-white transition-colors"
      title="Close"
    >
      <svg class="w-3.5 h-3.5" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
        <line x1="18" y1="6" x2="6" y2="18"></line>
        <line x1="6" y1="6" x2="18" y2="18"></line>
      </svg>
    </button>
  </div>
</div>
