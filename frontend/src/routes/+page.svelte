<script lang="ts">
  import { onMount } from "svelte";
  import {
    listMembers,
    listAccounts,
    listTransactions,
    listEnvelopes,
    listGoals,
    listInvestments,
    listDebts,
    windowMinimize,
    windowMaximize,
    windowClose,
    windowStartDragging,
    type Member,
    type Account,
    type Transaction,
    type EnvelopeItem,
    type Goal,
    type Investment,
    type Debt,
  } from "$lib/api";

  import Logo from "$lib/components/Logo.svelte";
  import AmbientGlow from "$lib/components/AmbientGlow.svelte";
  import AiChatDrawer from "$lib/components/AiChatDrawer.svelte";
  import ProfilePinModal from "$lib/components/ProfilePinModal.svelte";
  import QuickAddModal from "$lib/components/QuickAddModal.svelte";
  import SettingsModal from "$lib/components/SettingsModal.svelte";

  import DashboardHome from "$lib/components/DashboardHome.svelte";
  import TransactionView from "$lib/components/TransactionView.svelte";
  import BudgetView from "$lib/components/BudgetView.svelte";
  import AccountsView from "$lib/components/AccountsView.svelte";
  import GoalsView from "$lib/components/GoalsView.svelte";
  import InvestmentsView from "$lib/components/InvestmentsView.svelte";
  import KidsView from "$lib/components/KidsView.svelte";
  import IslamicView from "$lib/components/IslamicView.svelte";

  let activeTab = $state<
    "dashboard" | "transactions" | "budget" | "accounts" | "goals" | "investments" | "kids" | "islamic"
  >("dashboard");

  let isSidebarCollapsed = $state(false);
  let isAiDrawerOpen = $state(false);

  let members = $state<Member[]>([]);
  let activeMember = $state<Member | null>(null);

  let accounts = $state<Account[]>([]);
  let transactions = $state<Transaction[]>([]);
  let envelopes = $state<EnvelopeItem[]>([]);
  let goals = $state<Goal[]>([]);
  let investments = $state<Investment[]>([]);
  let debts = $state<Debt[]>([]);

  let showPinModal = $state(false);
  let showQuickAddModal = $state(false);
  let showSettingsModal = $state(false);

  onMount(async () => {
    await refreshAll();
  });

  async function refreshAll() {
    try {
      members = await listMembers();
      if (!activeMember && members.length > 0) {
        activeMember = members[0]; // Ayah default
      }

      if (activeMember) {
        const isChild = activeMember.role === "child";
        accounts = await listAccounts(activeMember.id, isChild);
      }

      transactions = await listTransactions(100);
      envelopes = await listEnvelopes();
      goals = await listGoals();
      investments = await listInvestments();
      debts = await listDebts();
    } catch (e) {
      console.error("Gagal inisialisasi data:", e);
    }
  }

  function handleProfileSwitched(m: Member) {
    activeMember = m;
    refreshAll();
  }

  const navItems = [
    {
      id: "dashboard",
      label: "Ringkasan",
      iconPath: "M3 13h8V3H3v10zm0 8h8v-6H3v6zm10 0h8V11h-8v10zm0-18v6h8V3h-8z", // Dashboard grid
    },
    {
      id: "transactions",
      label: "Transaksi",
      iconPath: "M8 7h12m0 0l-4-4m4 4l-4 4m0 6H4m0 0l4 4m-4-4l4-4", // Exchange arrows
    },
    {
      id: "budget",
      label: "Amplop Anggaran",
      iconPath: "M4 4h16c1.1 0 2 .9 2 2v12c0 1.1-.9 2-2 2H4c-1.1 0-2-.9-2-2V6c0-1.1.9-2 2-2z M22 6l-10 7L2 6", // Envelope
    },
    {
      id: "accounts",
      label: "Rekening & Kas",
      iconPath: "M3 21h18M3 10h18M5 6l7-3 7 3M4 10v11M20 10v11M8 14v4M12 14v4M16 14v4", // Bank columns
    },
    {
      id: "goals",
      label: "Target Impian",
      iconPath: "M12 2a10 10 0 100 20 10 10 0 000-20zm0 4a6 6 0 110 12 6 6 0 010-12zm0 4a2 2 0 100 4 2 2 0 000-4z", // Target bullseye
    },
    {
      id: "investments",
      label: "Aset & Hutang",
      iconPath: "M23 6l-9.5 9.5-5-5L1 18 M17 6h6v6", // Trending up chart
    },
    {
      id: "kids",
      label: "Celengan Anak",
      iconPath: "M19 14c1.49-1.46 3-3.21 3-5.5A5.5 5.5 0 0 0 16.5 3c-1.76 0-3 .5-4.5 2-1.5-1.5-2.74-2-4.5-2A5.5 5.5 0 0 0 2 8.5c0 2.3 1.5 4.05 3 5.5l7 7Z", // Heart / Kindness
    },
    {
      id: "islamic",
      label: "Zakat & Syariah",
      iconPath: "M12 3a6 6 0 0 0 9 9 9 9 0 1 1-9-9Z", // Moon / Syariah
    },
  ];

  let currentSection = $derived(
    navItems.find((n) => n.id === activeTab) || navItems[0]
  );
</script>

<div class="h-screen w-screen flex flex-col bg-[#0A0A0C] text-[#EDEDED] overflow-hidden select-none font-sans">
  <!-- CATerm-Style Header Top Bar -->
  <header
    data-tauri-drag-region
    class="h-11 border-b border-[#262626] flex items-center justify-between px-3 text-xs bg-[#0E0E12] shrink-0 select-none z-30"
  >
    <!-- Left: Brand Logo & Breadcrumb -->
    <div class="flex items-center gap-2.5 min-w-0" data-tauri-drag-region>
      <div class="flex items-center gap-2 pr-2 border-r border-[#262626]">
        <Logo size={20} />
        <span class="font-bold tracking-wider text-white text-xs">CACASH</span>
        <span class="px-1.5 py-0.2 rounded text-[9px] font-medium bg-[#10B981]/20 text-[#34D399] border border-[#10B981]/30">
          Family
        </span>
      </div>

      <!-- Current Breadcrumb -->
      <div class="flex items-center gap-1.5 text-xs text-[#9CA3AF] px-1">
        <svg class="w-3.5 h-3.5 text-[#10B981]" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path stroke-linecap="round" stroke-linejoin="round" d={currentSection.iconPath} />
        </svg>
        <span class="text-white font-medium">{currentSection.label}</span>
      </div>
    </div>

    <!-- Center: Quick Actions & Profile -->
    <div class="hidden md:flex items-center gap-2" data-tauri-drag-region>
      <button
        onclick={() => (showQuickAddModal = true)}
        class="px-2.5 py-1 rounded-md bg-[#10B981] hover:bg-[#059669] text-white text-[11px] font-bold flex items-center gap-1.5 shadow-sm shadow-[#10B981]/20 transition-all"
        title="Catat Transaksi Cepat (<= 5 detik)"
      >
        <svg class="w-3 h-3" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
          <line x1="12" y1="5" x2="12" y2="19"></line>
          <line x1="5" y1="12" x2="19" y2="12"></line>
        </svg>
        Catat Cepat
      </button>

      {#if activeMember}
        <button
          onclick={() => (showPinModal = true)}
          class="px-2.5 py-1 rounded-md bg-[#18181F] hover:bg-[#272732] border border-[#272732] flex items-center gap-1.5 text-[11px] text-[#A7F3D0] transition-colors"
          title="Ganti Profil Anggota"
        >
          <span class="w-1.5 h-1.5 rounded-full bg-[#10B981]"></span>
          <span>{activeMember.display_name}</span>
          <span class="text-[9px] text-[#6B7280]">({activeMember.role})</span>
        </button>
      {/if}
    </div>

    <!-- Right: AI Drawer, Settings, Window Controls -->
    <div class="flex items-center gap-1 no-drag">
      <!-- AI Coach Toggle Button -->
      <button
        onclick={() => (isAiDrawerOpen = !isAiDrawerOpen)}
        class="px-2 py-1 rounded-md text-xs transition-colors flex items-center gap-1.5 mr-1 {isAiDrawerOpen ? 'bg-[#10B981] text-white' : 'text-[#9CA3AF] hover:text-white hover:bg-[#18181F]'}"
        title="Buka AI Financial Coach"
      >
        <svg class="w-3.5 h-3.5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M12 2a2 2 0 0 1 2 2v2a2 2 0 0 1-2 2 2 2 0 0 1-2-2V4a2 2 0 0 1 2-2z"></path>
          <rect x="4" y="8" width="16" height="12" rx="2"></rect>
        </svg>
        <span class="hidden sm:inline text-[11px] font-medium">AI Coach</span>
        <span class="w-1.5 h-1.5 rounded-full bg-[#10B981] animate-pulse"></span>
      </button>

      <!-- Settings Icon -->
      <button
        onclick={() => (showSettingsModal = true)}
        class="p-1.5 rounded-md text-[#9CA3AF] hover:text-white hover:bg-[#18181F] transition-colors"
        title="Pengaturan Mode & AI"
      >
        <svg class="w-3.5 h-3.5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <circle cx="12" cy="12" r="3"></circle>
          <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"></path>
        </svg>
      </button>

      <!-- Custom Window Controls -->
      <div class="hidden sm:flex items-center pl-1 border-l border-[#262626] ml-1">
        <button
          onclick={windowMinimize}
          class="p-1.5 rounded-md hover:bg-[#18181F] text-[#9CA3AF] hover:text-white transition-colors"
          title="Minimize"
        >
          <svg class="w-3 h-3" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
            <line x1="5" y1="12" x2="19" y2="12"></line>
          </svg>
        </button>
        <button
          onclick={windowMaximize}
          class="p-1.5 rounded-md hover:bg-[#18181F] text-[#9CA3AF] hover:text-white transition-colors"
          title="Maximize"
        >
          <svg class="w-3 h-3" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <rect x="3" y="3" width="18" height="18" rx="2" ry="2"></rect>
          </svg>
        </button>
        <button
          onclick={windowClose}
          class="p-1.5 rounded-md hover:bg-rose-600 hover:text-white text-[#9CA3AF] transition-colors"
          title="Close"
        >
          <svg class="w-3 h-3" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
            <line x1="18" y1="6" x2="6" y2="18"></line>
            <line x1="6" y1="6" x2="18" y2="18"></line>
          </svg>
        </button>
      </div>
    </div>
  </header>

  <!-- Main Shell Body: Collapsible Sidebar + Canvas Viewport -->
  <div class="flex flex-1 min-h-0 overflow-hidden">
    <!-- CATerm-Style Collapsible Sidebar -->
    <aside
      class="bg-[#0E0E12] border-r border-[#262626] flex flex-col justify-between py-2 transition-all duration-200 select-none shrink-0 {isSidebarCollapsed ? 'w-14' : 'w-56'}"
    >
      <div class="min-h-0 flex flex-col">
        <!-- Sidebar Header with Collapse Button -->
        <div class="px-2.5 pb-2 flex items-center justify-between">
          {#if !isSidebarCollapsed}
            <span class="text-[10px] uppercase font-bold text-[#6B7280] tracking-wider px-1">
              Menu Utama
            </span>
          {/if}
          <button
            onclick={() => (isSidebarCollapsed = !isSidebarCollapsed)}
            class="p-1.5 rounded-md text-[#9CA3AF] hover:text-white hover:bg-[#18181F] transition-colors ml-auto"
            title={isSidebarCollapsed ? "Perlebar Sidebar" : "Persempit Sidebar"}
          >
            <svg class="w-3.5 h-3.5 transform transition-transform {isSidebarCollapsed ? 'rotate-180' : ''}" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path stroke-linecap="round" stroke-linejoin="round" d="M11 19l-7-7 7-7m8 14l-7-7 7-7" />
            </svg>
          </button>
        </div>

        <!-- Navigation List with Active Marker -->
        <nav class="flex-1 min-h-0 overflow-y-auto scrollbar-none px-2 space-y-1 text-xs">
          {#each navItems as item}
            <button
              onclick={() => (activeTab = item.id as any)}
              title={isSidebarCollapsed ? item.label : ""}
              class="w-full relative px-2.5 py-2 rounded-lg flex items-center gap-3 transition-colors overflow-hidden text-left {activeTab === item.id ? 'bg-[#18181F] text-white font-semibold' : 'text-[#9CA3AF] hover:bg-[#18181F]/50 hover:text-white'}"
            >
              {#if activeTab === item.id}
                <!-- CATerm Left Edge Active Bar -->
                <span class="absolute -left-2 top-1.5 bottom-1.5 w-[3px] rounded-r bg-[#10B981]"></span>
              {/if}

              <div class="w-5 h-5 flex items-center justify-center shrink-0 {activeTab === item.id ? 'text-[#10B981]' : 'text-[#9CA3AF]'}">
                <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8">
                  <path stroke-linecap="round" stroke-linejoin="round" d={item.iconPath} />
                </svg>
              </div>

              {#if !isSidebarCollapsed}
                <span class="truncate whitespace-nowrap">{item.label}</span>
              {/if}
            </button>
          {/each}
        </nav>
      </div>

      <!-- Bottom Profile Card -->
      <div class="p-2 border-t border-[#262626]">
        {#if activeMember}
          <button
            onclick={() => (showPinModal = true)}
            class="w-full p-1.5 rounded-lg hover:bg-[#18181F] flex items-center gap-2.5 transition-colors text-left"
            title="Klik untuk ganti profil keluarga"
          >
            <div class="w-7 h-7 rounded-full bg-[#10B981]/20 border border-[#10B981]/40 flex items-center justify-center text-xs font-bold text-[#34D399] shrink-0">
              {activeMember.display_name.charAt(0)}
            </div>
            {#if !isSidebarCollapsed}
              <div class="min-w-0 flex-1">
                <span class="block text-xs font-bold text-white truncate">{activeMember.display_name}</span>
                <span class="block text-[9px] text-[#6B7280] uppercase tracking-wider">{activeMember.role}</span>
              </div>
              <span class="text-[10px] text-[#6B7280]">🔒</span>
            {/if}
          </button>
        {/if}
      </div>
    </aside>

    <!-- Canvas Viewport with CATerm Ambient Halo Underglow -->
    <div class="flex-1 min-w-0 flex relative overflow-visible pl-2 pt-2 pr-0 pb-0 md:p-0 bg-[#0A0A0C]">
      <AmbientGlow accent="#10B981" />

      <!-- Inner Content Card with Matrix Grid and Rounded-TL Corner -->
      <main
        class="flex-1 min-w-0 flex overflow-hidden relative z-10 bg-[#121217] border-t border-l border-[#262626] rounded-none rounded-tl-2xl md:rounded-none md:rounded-tl-xl transition-colors duration-150 bg-[linear-gradient(to_right,#ffffff08_1px,transparent_1px),linear-gradient(to_bottom,#ffffff08_1px,transparent_1px)] bg-[size:56px_56px]"
      >
        {#if activeTab === "dashboard"}
          <DashboardHome
            {activeMember}
            {accounts}
            {transactions}
            {envelopes}
            onQuickAdd={() => (showQuickAddModal = true)}
          />
        {:else if activeTab === "transactions"}
          <TransactionView
            {transactions}
            onRefresh={refreshAll}
            onQuickAdd={() => (showQuickAddModal = true)}
          />
        {:else if activeTab === "budget"}
          <BudgetView {envelopes} onRefresh={refreshAll} />
        {:else if activeTab === "accounts"}
          <AccountsView {accounts} {activeMember} onRefresh={refreshAll} />
        {:else if activeTab === "goals"}
          <GoalsView {goals} onRefresh={refreshAll} />
        {:else if activeTab === "investments"}
          <InvestmentsView {investments} {debts} onRefresh={refreshAll} />
        {:else if activeTab === "kids"}
          <KidsView {members} {activeMember} />
        {:else if activeTab === "islamic"}
          <IslamicView />
        {/if}
      </main>
    </div>
  </div>

  <!-- AI Financial Coach Side Drawer (Accessible from all tabs) -->
  <AiChatDrawer
    isOpen={isAiDrawerOpen}
    onClose={() => (isAiDrawerOpen = false)}
  />

  <!-- Modals -->
  <ProfilePinModal
    isOpen={showPinModal}
    {members}
    onSelectMember={handleProfileSwitched}
    onClose={() => (showPinModal = false)}
  />

  <QuickAddModal
    isOpen={showQuickAddModal}
    {accounts}
    {activeMember}
    onClose={() => (showQuickAddModal = false)}
    onSuccess={refreshAll}
  />

  <SettingsModal
    isOpen={showSettingsModal}
    onClose={() => (showSettingsModal = false)}
  />
</div>
