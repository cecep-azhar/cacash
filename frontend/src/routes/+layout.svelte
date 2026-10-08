<script lang="ts">
  import "../app.css";
  import { onMount, type Snippet } from "svelte";
  import { page } from "$app/state";
  import { cashStore } from "$lib/stores/cashStore.svelte";
  import TitleBar from "$lib/components/TitleBar.svelte";
  import FloatingAiCoach from "$lib/components/FloatingAiCoach.svelte";
  import AmbientGlow from "$lib/components/AmbientGlow.svelte";
  import ProfilePinModal from "$lib/components/ProfilePinModal.svelte";
  import QuickAddModal from "$lib/components/QuickAddModal.svelte";
  import SettingsModal from "$lib/components/SettingsModal.svelte";

  interface Props {
    children?: Snippet;
  }

  let { children }: Props = $props();

  onMount(async () => {
    await cashStore.refreshAll();
  });

  const navItems = [
    {
      href: "/dashboard",
      label: "Ringkasan",
      iconPath: "M3 13h8V3H3v10zm0 8h8v-6H3v6zm10 0h8V11h-8v10zm0-18v6h8V3h-8z", // Dashboard grid
    },
    {
      href: "/transactions",
      label: "Transaksi",
      iconPath: "M8 7h12m0 0l-4-4m4 4l-4 4m0 6H4m0 0l4 4m-4-4l4-4", // Exchange arrows
    },
    {
      href: "/accounts",
      label: "Rekening & Kas",
      iconPath: "M3 21h18M3 10h18M5 6l7-3 7 3M4 10v11M20 10v11M8 14v4M12 14v4M16 14v4", // Bank columns
    },
    {
      href: "/budgets",
      label: "Amplop Anggaran",
      iconPath: "M4 4h16c1.1 0 2 .9 2 2v12c0 1.1-.9 2-2 2H4c-1.1 0-2-.9-2-2V6c0-1.1.9-2 2-2z M22 6l-10 7L2 6", // Envelope
    },
    {
      href: "/goals",
      label: "Target Impian",
      iconPath: "M12 2a10 10 0 100 20 10 10 0 000-20zm0 4a6 6 0 110 12 6 6 0 010-12zm0 4a2 2 0 100 4 2 2 0 000-4z", // Target bullseye
    },
    {
      href: "/islamic",
      label: "Zakat & Syariah",
      iconPath: "M12 3a6 6 0 0 0 9 9 9 9 0 1 1-9-9Z", // Moon / Syariah
    },
    {
      href: "/kids",
      label: "Celengan Anak",
      iconPath: "M19 14c1.49-1.46 3-3.21 3-5.5A5.5 5.5 0 0 0 16.5 3c-1.76 0-3 .5-4.5 2-1.5-1.5-2.74-2-4.5-2A5.5 5.5 0 0 0 2 8.5c0 2.3 1.5 4.05 3 5.5l7 7Z", // Heart / Kindness
    },
    {
      href: "/investments",
      label: "Aset & Hutang",
      iconPath: "M23 6l-9.5 9.5-5-5L1 18 M17 6h6v6", // Trending up chart
    },
    {
      href: "/ai-coach",
      label: "AI Coach",
      iconPath: "M12 2a2 2 0 0 1 2 2v2a2 2 0 0 1-2 2 2 2 0 0 1-2-2V4a2 2 0 0 1 2-2z M4 8h16v12H4z", // AI Bot
    },
    {
      href: "/settings",
      label: "Pengaturan",
      iconPath: "M12 15a3 3 0 1 0 0-6 3 3 0 0 0 0 6z M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-4 0v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1 0-4h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 2.83-2.83l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 4 0v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z", // Cog Settings
    },
  ];

  let currentRoute = $derived(page.url.pathname);
  let activeMenuItem = $derived(
    navItems.find((item) => currentRoute.startsWith(item.href)) || navItems[0]
  );
</script>

<div class="h-screen w-screen flex flex-col bg-[#0A0A0C] text-[#EDEDED] overflow-hidden select-none font-sans">
  <!-- Frameless CADS v1.0 TitleBar -->
  <TitleBar
    activeMember={cashStore.activeMember}
    activeTitle={activeMenuItem.label}
    onSwitchProfile={() => (cashStore.showPinModal = true)}
    onQuickAdd={() => (cashStore.showQuickAddModal = true)}
    onOpenSettings={() => (cashStore.showSettingsModal = true)}
  />

  <!-- Main Shell Body: Collapsible Sidebar + Raised Content Panel -->
  <div class="flex flex-1 min-h-0 overflow-hidden">
    <!-- CADS Collapsible Sidebar: w-60 <-> w-16 -->
    <aside
      class="bg-[#0E0E12] border-r border-[#262626] flex flex-col justify-between py-2 transition-all duration-200 select-none shrink-0 {cashStore.isSidebarCollapsed ? 'w-16' : 'w-60'}"
    >
      <div class="min-h-0 flex flex-col">
        <!-- Sidebar Header with Collapse Button -->
        <div class="px-3 pb-2 flex items-center justify-between">
          {#if !cashStore.isSidebarCollapsed}
            <span class="text-[10px] uppercase font-bold text-[#6B7280] tracking-wider px-1">
              Menu Utama
            </span>
          {/if}
          <button
            type="button"
            onclick={() => (cashStore.isSidebarCollapsed = !cashStore.isSidebarCollapsed)}
            class="p-1.5 rounded-md text-[#9CA3AF] hover:text-white hover:bg-[#18181F] transition-colors ml-auto cursor-pointer"
            title={cashStore.isSidebarCollapsed ? "Perlebar Sidebar (w-60)" : "Persempit Sidebar (w-16)"}
          >
            <svg class="w-3.5 h-3.5 transform transition-transform {cashStore.isSidebarCollapsed ? 'rotate-180' : ''}" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path stroke-linecap="round" stroke-linejoin="round" d="M11 19l-7-7 7-7m8 14l-7-7 7-7" />
            </svg>
          </button>
        </div>

        <!-- Navigation List with CADS 3px vertical active-indicator bar -->
        <nav class="flex-1 min-h-0 overflow-y-auto scrollbar-none px-2 space-y-1 text-xs">
          {#each navItems as item}
            {@const isActive = currentRoute === item.href || (item.href !== "/dashboard" && currentRoute.startsWith(item.href))}
            <a
              href={item.href}
              title={cashStore.isSidebarCollapsed ? item.label : ""}
              class="w-full relative px-2.5 py-2 rounded-lg flex items-center gap-3 transition-colors overflow-hidden text-left cursor-pointer {isActive ? 'bg-[#18181F] text-white font-semibold' : 'text-[#9CA3AF] hover:bg-[#18181F]/50 hover:text-white'}"
            >
              {#if isActive}
                <!-- CADS v1.0 Emerald 3px vertical active-indicator bar -->
                <span class="absolute left-0 top-1.5 bottom-1.5 w-[3px] rounded-r bg-[#10B981]"></span>
              {/if}

              <div class="w-5 h-5 flex items-center justify-center shrink-0 {isActive ? 'text-[#10B981]' : 'text-[#9CA3AF]'}">
                <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8">
                  <path stroke-linecap="round" stroke-linejoin="round" d={item.iconPath} />
                </svg>
              </div>

              {#if !cashStore.isSidebarCollapsed}
                <span class="truncate whitespace-nowrap">{item.label}</span>
              {/if}
            </a>
          {/each}
        </nav>
      </div>

      <!-- Bottom Profile Avatar Card -->
      <div class="p-2 border-t border-[#262626]">
        {#if cashStore.activeMember}
          <button
            type="button"
            onclick={() => (cashStore.showPinModal = true)}
            class="w-full p-1.5 rounded-lg hover:bg-[#18181F] flex items-center gap-2.5 transition-colors text-left cursor-pointer"
            title="Klik untuk ganti profil keluarga"
          >
            <div class="w-7 h-7 rounded-full bg-[#10B981]/20 border border-[#10B981]/40 flex items-center justify-center text-xs font-bold text-[#34D399] shrink-0">
              {cashStore.activeMember.display_name.charAt(0)}
            </div>
            {#if !cashStore.isSidebarCollapsed}
              <div class="min-w-0 flex-1">
                <span class="block text-xs font-bold text-white truncate">{cashStore.activeMember.display_name}</span>
                <span class="block text-[9px] text-[#6B7280] uppercase tracking-wider">{cashStore.activeMember.role}</span>
              </div>
              <span class="text-[10px] text-[#6B7280]">🔒</span>
            {/if}
          </button>
        {/if}
      </div>
    </aside>

    <!-- CADS Raised Content Container: Canvas #0e0e0e with Elevated Main #161616 (rounded-tl-xl border-t border-l border-neutral-800) -->
    <div class="flex-1 min-w-0 flex relative overflow-visible pl-2 pt-2 pr-0 pb-0 md:p-0 bg-[#0e0e0e]">
      <AmbientGlow accent="#10b981" />

      <main
        class="flex-1 min-w-0 flex overflow-hidden relative z-10 bg-[#161616] border-t border-l border-neutral-800 rounded-none rounded-tl-xl md:rounded-none md:rounded-tl-xl transition-colors duration-150"
      >
        {#if children}
          {@render children()}
        {/if}
      </main>
    </div>
  </div>

  <!-- CADS Floating Smart Card AI Assistant (Financial Coach) -->
  <FloatingAiCoach />

  <!-- Global Modals -->
  <ProfilePinModal
    isOpen={cashStore.showPinModal}
    members={cashStore.members}
    onSelectMember={(m) => cashStore.handleProfileSwitched(m)}
    onClose={() => (cashStore.showPinModal = false)}
  />

  <QuickAddModal
    isOpen={cashStore.showQuickAddModal}
    accounts={cashStore.accounts}
    activeMember={cashStore.activeMember}
    onClose={() => (cashStore.showQuickAddModal = false)}
    onSuccess={() => cashStore.refreshAll()}
  />

  <SettingsModal
    isOpen={cashStore.showSettingsModal}
    onClose={() => (cashStore.showSettingsModal = false)}
  />
</div>
