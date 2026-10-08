<script lang="ts">
  import {
    windowMinimize,
    windowMaximize,
    windowClose,
    windowStartDragging,
    type Member,
  } from "$lib/api";
  import Logo from "./Logo.svelte";

  interface Props {
    activeMember?: Member | null;
    activeTitle?: string;
    onSwitchProfile?: () => void;
    onQuickAdd?: () => void;
    onOpenSettings?: () => void;
  }

  let {
    activeMember,
    activeTitle = "Ringkasan",
    onSwitchProfile,
    onQuickAdd,
    onOpenSettings,
  }: Props = $props();
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
<header
  data-tauri-drag-region
  onmousedown={(e) => {
    if (e.buttons === 1) {
      windowStartDragging();
    }
  }}
  class="h-12 border-b border-[#262626] flex items-center justify-between px-3 text-xs bg-[#0E0E12] shrink-0 select-none z-30"
>
  <!-- Left: Brand Logo, Product Code & Breadcrumbs -->
  <div class="flex items-center gap-2.5 min-w-0" data-tauri-drag-region>
    <div class="flex items-center gap-2 pr-3 border-r border-[#262626]">
      <Logo size={20} />
      <span class="font-bold tracking-wider text-white text-xs">CACASH</span>
      <span class="px-1.5 py-0.5 rounded text-[9px] font-semibold bg-[#10B981]/20 text-[#34D399] border border-[#10B981]/30">
        CADS v1.0
      </span>
    </div>

    <!-- Active Breadcrumb -->
    <div class="flex items-center gap-2 text-xs text-[#9CA3AF] px-1">
      <span class="text-[#6B7280]">/</span>
      <span class="text-white font-medium">{activeTitle}</span>
    </div>
  </div>

  <!-- Center: Quick Actions & Live Encrypted Pulse -->
  <div class="hidden md:flex items-center gap-3" data-tauri-drag-region>
    {#if onQuickAdd}
      <button
        type="button"
        onclick={onQuickAdd}
        class="px-3 py-1 rounded-md bg-[#10B981] hover:bg-[#059669] text-white text-[11px] font-bold flex items-center gap-1.5 shadow-sm shadow-[#10B981]/20 transition-all cursor-pointer"
        title="Catat Transaksi Cepat (<= 5 detik)"
      >
        <svg class="w-3 h-3" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
          <line x1="12" y1="5" x2="12" y2="19"></line>
          <line x1="5" y1="12" x2="19" y2="12"></line>
        </svg>
        <span>Catat Cepat</span>
      </button>
    {/if}

    <!-- Live Encrypted Vault Pulse -->
    <div class="flex items-center gap-1.5 px-2 py-0.5 rounded-full bg-[#18181F] border border-[#272732] text-[10px] text-[#A7F3D0]">
      <span class="w-1.5 h-1.5 rounded-full bg-[#10B981] animate-pulse"></span>
      <span class="font-mono text-[9px]">SQLCipher Vault Active</span>
    </div>

    {#if activeMember}
      <button
        type="button"
        onclick={onSwitchProfile}
        class="px-2.5 py-1 rounded-md bg-[#18181F] hover:bg-[#272732] border border-[#272732] flex items-center gap-1.5 text-[11px] text-[#A7F3D0] transition-colors cursor-pointer"
        title="Ganti Profil Anggota"
      >
        <span class="w-1.5 h-1.5 rounded-full bg-[#10B981]"></span>
        <span>{activeMember.display_name}</span>
        <span class="text-[9px] text-[#6B7280]">({activeMember.role})</span>
      </button>
    {/if}
  </div>

  <!-- Right: Settings, Pro Aurora Halo & Frameless Window Controls -->
  <div class="flex items-center gap-1.5 no-drag">
    <!-- Settings Icon -->
    {#if onOpenSettings}
      <button
        type="button"
        onclick={onOpenSettings}
        class="p-1.5 rounded-md text-[#9CA3AF] hover:text-white hover:bg-[#18181F] transition-colors cursor-pointer"
        title="Pengaturan Mode & AI"
      >
        <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <circle cx="12" cy="12" r="3"></circle>
          <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"></path>
        </svg>
      </button>
    {/if}

    <!-- Custom Window Controls -->
    <div class="flex items-center pl-1 border-l border-[#262626] ml-1">
      <button
        type="button"
        onclick={windowMinimize}
        class="p-1.5 rounded-md hover:bg-[#18181F] text-[#9CA3AF] hover:text-white transition-colors cursor-pointer"
        title="Minimize"
      >
        <svg class="w-3.5 h-3.5" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
          <line x1="5" y1="12" x2="19" y2="12"></line>
        </svg>
      </button>
      <button
        type="button"
        onclick={windowMaximize}
        class="p-1.5 rounded-md hover:bg-[#18181F] text-[#9CA3AF] hover:text-white transition-colors cursor-pointer"
        title="Maximize"
      >
        <svg class="w-3.5 h-3.5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <rect x="3" y="3" width="18" height="18" rx="2" ry="2"></rect>
        </svg>
      </button>
      <button
        type="button"
        onclick={windowClose}
        class="p-1.5 rounded-md hover:bg-rose-600 hover:text-white text-[#9CA3AF] transition-colors cursor-pointer"
        title="Close"
      >
        <svg class="w-3.5 h-3.5" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
          <line x1="18" y1="6" x2="6" y2="18"></line>
          <line x1="6" y1="6" x2="18" y2="18"></line>
        </svg>
      </button>
    </div>
  </div>
</header>
