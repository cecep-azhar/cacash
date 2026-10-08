import { invoke } from "@tauri-apps/api/core";

export interface Member {
  id: string;
  display_name: string;
  role: string; // 'owner', 'partner', 'child', 'member'
  birth_year: number;
  avatar: string;
  created_at: number;
  updated_at: number;
}

export interface Account {
  id: string;
  name: string;
  acc_type: string;
  currency: string;
  visibility: string;
  opening_balance: number;
  current_balance: number;
  is_archived: boolean;
  created_at: number;
}

export interface Transaction {
  id: string;
  account_id: string;
  account_name: string;
  member_id: string;
  member_name: string;
  date: string;
  amount: number;
  tx_type: string; // 'income', 'expense', 'transfer', 'nafkah', 'sedekah', 'zakat'
  category_id: string;
  category_name: string;
  payee: string;
  note: string;
  created_at: number;
}

export interface EnvelopeItem {
  id: string;
  category_id: string;
  category_name: string;
  monthly_budget: number;
  spent_amount: number;
  percentage_used: number;
}

export interface Goal {
  id: string;
  title: string;
  target_amount: number;
  current_amount: number;
  deadline_date: string;
  is_ibadah: boolean;
  percentage: number;
}

export interface Investment {
  id: string;
  title: string;
  inv_type: string;
  units: string;
  cost_basis: number;
  current_value: number;
  gain_loss: number;
}

export interface Debt {
  id: string;
  title: string;
  principal_amount: number;
  remaining_amount: number;
  interest_rate: string;
  is_riba: boolean;
  due_date: string;
}

export interface KidMission {
  id: string;
  member_id: string;
  title: string;
  reward_amount: number;
  is_approved: boolean;
  is_completed: boolean;
}

export interface Hadith {
  id: string;
  number: number;
  narrator: string;
  arabic: string;
  translation_id: string;
  theme: string;
}

export interface ZakatMalCalculation {
  total_zakatable_assets: number;
  deductible_short_term_debts: number;
  net_wealth: number;
  gold_price_per_gram: number;
  nishab_threshold: number;
  is_eligible: boolean;
  zakat_due: number;
}

export interface ZakatFitrahCalculation {
  household_members_count: number;
  price_per_person: number;
  total_due: number;
}

export interface FinancialHealthScore {
  total_score: number;
  emergency_fund_score: number;
  debt_ratio_score: number;
  savings_rate_score: number;
  budget_adherence_score: number;
  priority_suggestion: string;
}

export interface AiSettings {
  base_url: string;
  api_key: string;
  model: string;
}

export interface AskAiResponse {
  text: string;
  model: string;
}

// Window controls
export const windowMinimize = () => invoke("window_minimize");
export const windowMaximize = () => invoke("window_maximize");
export const windowClose = () => invoke("window_close");
export const windowStartDragging = () => invoke("window_start_dragging");

// Profile & Members
export const listMembers = () => invoke<Member[]>("cmd_list_members");
export const createMember = (input: {
  display_name: string;
  role: string;
  birth_year: number;
  pin: string;
  avatar?: string;
}) => invoke<Member>("cmd_create_member", { input });
export const verifyPin = (memberId: string, pin: string) =>
  invoke<boolean>("cmd_verify_pin", { memberId, pin });

// Accounts
export const listAccounts = (requesterMemberId: string, isChild: boolean) =>
  invoke<Account[]>("cmd_list_accounts", { requesterMemberId, isChild });
export const createAccount = (input: {
  name: string;
  acc_type: string;
  currency?: string;
  visibility?: string;
  opening_balance?: number;
  owner_member_id: string;
}) => invoke<Account>("cmd_create_account", { input });

// Transactions
export const listTransactions = (limit = 100) =>
  invoke<Transaction[]>("cmd_list_transactions", { limit });
export const createTransaction = (input: {
  account_id: string;
  member_id: string;
  date?: string;
  amount: number;
  tx_type: string;
  category_id: string;
  payee?: string;
  note?: string;
}) => invoke<Transaction>("cmd_create_transaction", { input });
export const deleteTransaction = (id: string) =>
  invoke<void>("cmd_delete_transaction", { id });

// Envelopes Budget
export const listEnvelopes = () => invoke<EnvelopeItem[]>("cmd_list_envelopes");

// Goals
export const listGoals = () => invoke<Goal[]>("cmd_list_goals");
export const createGoal = (input: {
  title: string;
  target_amount: number;
  current_amount?: number;
  deadline_date: string;
  is_ibadah?: boolean;
}) => invoke<Goal>("cmd_create_goal", { input });

// Investments & Debts
export const listInvestments = () => invoke<Investment[]>("cmd_list_investments");
export const listDebts = () => invoke<Debt[]>("cmd_list_debts");

// Kids
export const listKidMissions = (memberId: string) =>
  invoke<KidMission[]>("cmd_list_kid_missions", { memberId });
export const toggleMission = (id: string) =>
  invoke<boolean>("cmd_toggle_mission", { id });

// Islamic Finance: Zakat & Hadith
export const calculateZakatMal = (totalAssets: number, shortTermDebts: number, goldPrice: number) =>
  invoke<ZakatMalCalculation>("cmd_calculate_zakat_mal", { totalAssets, shortTermDebts, goldPrice });

export const calculateZakatFitrah = (membersCount: number, pricePerPerson: number) =>
  invoke<ZakatFitrahCalculation>("cmd_calculate_zakat_fitrah", { membersCount, pricePerPerson });

export const getTodayHadith = () => invoke<Hadith>("cmd_get_today_hadith");

// Health Score
export const calculateHealthScore = (emergencyMonths: number, debtRatio: number, savingsRate: number, hasRiba: boolean) =>
  invoke<FinancialHealthScore>("cmd_calculate_health_score", { emergencyMonths, debtRatio, savingsRate, hasRiba });

// AI
export const askAi = (req: { user_prompt: string }) =>
  invoke<AskAiResponse>("cmd_ask_ai", { req });

// Export
export const exportTransactions = () =>
  invoke<string>("cmd_export_transactions");

// Settings
export const getAiSettings = () => invoke<AiSettings>("cmd_get_ai_settings");
export const saveAiSettings = (settings: AiSettings) =>
  invoke<void>("cmd_save_ai_settings", { settings });

// Format IDR Helper
export function formatIdr(amount: number): string {
  const isNeg = amount < 0;
  const absVal = Math.abs(amount);
  const s = absVal.toLocaleString("id-ID");
  return isNeg ? `-Rp ${s}` : `Rp ${s}`;
}
