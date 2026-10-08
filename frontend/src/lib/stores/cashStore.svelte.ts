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

class CashState {
  members = $state<Member[]>([]);
  activeMember = $state<Member | null>(null);
  accounts = $state<Account[]>([]);
  transactions = $state<Transaction[]>([]);
  envelopes = $state<EnvelopeItem[]>([]);
  goals = $state<Goal[]>([]);
  investments = $state<Investment[]>([]);
  debts = $state<Debt[]>([]);
  loading = $state(false);

  showPinModal = $state(false);
  showQuickAddModal = $state(false);
  showSettingsModal = $state(false);
  isSidebarCollapsed = $state(false);

  async refreshAll() {
    this.loading = true;
    try {
      this.members = await listMembers();
      if (!this.activeMember && this.members.length > 0) {
        this.activeMember = this.members[0];
      }

      if (this.activeMember) {
        const isChild = this.activeMember.role === "child";
        this.accounts = await listAccounts(this.activeMember.id, isChild);
      }

      this.transactions = await listTransactions(100);
      this.envelopes = await listEnvelopes();
      this.goals = await listGoals();
      this.investments = await listInvestments();
      this.debts = await listDebts();
    } catch (e) {
      console.error("Gagal sinkronisasi data kas:", e);
    } finally {
      this.loading = false;
    }
  }

  handleProfileSwitched(m: Member) {
    this.activeMember = m;
    this.refreshAll();
  }
}

export const cashStore = new CashState();
