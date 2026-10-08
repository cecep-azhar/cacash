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
    type Member,
    type Account,
    type Transaction,
    type EnvelopeItem,
    type Goal,
    type Investment,
    type Debt,
  } from "$lib/api";

  import TitleBar from "$lib/components/TitleBar.svelte";
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
  import AiCoachView from "$lib/components/AiCoachView.svelte";

  let activeTab = $state<
    "dashboard" | "transactions" | "budget" | "accounts" | "goals" | "investments" | "kids" | "islamic" | "ai"
  >("dashboard");

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
</script>

<div class="h-screen w-screen flex flex-col bg-[#0A0A0C] text-[#EDEDED] overflow-hidden select-none">
  <!-- TitleBar with quick add & profile -->
  <TitleBar
    {activeMember}
    onSwitchProfile={() => (showPinModal = true)}
    onQuickAdd={() => (showQuickAddModal = true)}
    onOpenSettings={() => (showSettingsModal = true)}
  />

  <!-- Main Body: Sidebar + Viewport -->
  <div class="flex-1 flex overflow-hidden">
    <!-- Sidebar -->
    <aside class="w-56 bg-[#0E0E12] border-r border-[#272732] flex flex-col justify-between p-3 select-none shrink-0">
      <div class="space-y-1">
        <div class="px-3 py-2 text-[10px] uppercase font-bold text-[#6B7280] tracking-wider">
          Menu Keuangan
        </div>

        <button
          onclick={() => (activeTab = "dashboard")}
          class="w-full flex items-center gap-2.5 px-3 py-2 rounded-xl text-xs font-semibold transition-all {activeTab === 'dashboard' ? 'bg-[#10B981] text-white shadow-md shadow-[#10B981]/20' : 'text-[#9CA3AF] hover:text-white hover:bg-[#18181F]'}"
        >
          <span>📊</span>
          <span>Ringkasan</span>
        </button>

        <button
          onclick={() => (activeTab = "transactions")}
          class="w-full flex items-center gap-2.5 px-3 py-2 rounded-xl text-xs font-semibold transition-all {activeTab === 'transactions' ? 'bg-[#10B981] text-white shadow-md shadow-[#10B981]/20' : 'text-[#9CA3AF] hover:text-white hover:bg-[#18181F]'}"
        >
          <span>💳</span>
          <span>Transaksi</span>
        </button>

        <button
          onclick={() => (activeTab = "budget")}
          class="w-full flex items-center gap-2.5 px-3 py-2 rounded-xl text-xs font-semibold transition-all {activeTab === 'budget' ? 'bg-[#10B981] text-white shadow-md shadow-[#10B981]/20' : 'text-[#9CA3AF] hover:text-white hover:bg-[#18181F]'}"
        >
          <span>✉️</span>
          <span>Amplop Anggaran</span>
        </button>

        <button
          onclick={() => (activeTab = "accounts")}
          class="w-full flex items-center gap-2.5 px-3 py-2 rounded-xl text-xs font-semibold transition-all {activeTab === 'accounts' ? 'bg-[#10B981] text-white shadow-md shadow-[#10B981]/20' : 'text-[#9CA3AF] hover:text-white hover:bg-[#18181F]'}"
        >
          <span>🏦</span>
          <span>Rekening & Kas</span>
        </button>

        <button
          onclick={() => (activeTab = "goals")}
          class="w-full flex items-center gap-2.5 px-3 py-2 rounded-xl text-xs font-semibold transition-all {activeTab === 'goals' ? 'bg-[#10B981] text-white shadow-md shadow-[#10B981]/20' : 'text-[#9CA3AF] hover:text-white hover:bg-[#18181F]'}"
        >
          <span>🎯</span>
          <span>Target & Impian</span>
        </button>

        <button
          onclick={() => (activeTab = "investments")}
          class="w-full flex items-center gap-2.5 px-3 py-2 rounded-xl text-xs font-semibold transition-all {activeTab === 'investments' ? 'bg-[#10B981] text-white shadow-md shadow-[#10B981]/20' : 'text-[#9CA3AF] hover:text-white hover:bg-[#18181F]'}"
        >
          <span>📈</span>
          <span>Aset & Hutang</span>
        </button>

        <div class="pt-2 px-3 py-1 text-[10px] uppercase font-bold text-[#6B7280] tracking-wider">
          Modul Khusus
        </div>

        <button
          onclick={() => (activeTab = "kids")}
          class="w-full flex items-center gap-2.5 px-3 py-2 rounded-xl text-xs font-semibold transition-all {activeTab === 'kids' ? 'bg-[#10B981] text-white shadow-md shadow-[#10B981]/20' : 'text-[#9CA3AF] hover:text-white hover:bg-[#18181F]'}"
        >
          <span>🐷</span>
          <span>Celengan Anak</span>
        </button>

        <button
          onclick={() => (activeTab = "islamic")}
          class="w-full flex items-center gap-2.5 px-3 py-2 rounded-xl text-xs font-semibold transition-all {activeTab === 'islamic' ? 'bg-[#10B981] text-white shadow-md shadow-[#10B981]/20' : 'text-[#9CA3AF] hover:text-white hover:bg-[#18181F]'}"
        >
          <span>🕌</span>
          <span>Zakat & Syariah</span>
        </button>

        <button
          onclick={() => (activeTab = "ai")}
          class="w-full flex items-center gap-2.5 px-3 py-2 rounded-xl text-xs font-semibold transition-all {activeTab === 'ai' ? 'bg-[#10B981] text-white shadow-md shadow-[#10B981]/20' : 'text-[#9CA3AF] hover:text-white hover:bg-[#18181F]'}"
        >
          <span>🤖</span>
          <span>AI Coach</span>
        </button>
      </div>

      <!-- Footer Version -->
      <div class="p-3 border-t border-[#272732] text-[10px] text-[#6B7280] flex items-center justify-between">
        <span>CACash v0.1.0</span>
        <span class="text-[#10B981]">Sovereign ✓</span>
      </div>
    </aside>

    <!-- Main Content Area -->
    <main class="flex-1 flex flex-col h-full overflow-hidden bg-[#0A0A0C]">
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
      {:else if activeTab === "ai"}
        <AiCoachView />
      {/if}
    </main>
  </div>
</div>

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
