use crate::window;
use cacash_core::account::{create_account, list_accounts, Account, CreateAccountInput};
use cacash_core::ai::{ask_financial_coach, get_ai_settings, save_ai_settings, AiSettings, AskAiRequest, AskAiResponse};
use cacash_core::budget::{list_envelopes, EnvelopeItem};
use cacash_core::error::CashError;
use cacash_core::export::export_transactions_csv;
use cacash_core::goals::{create_goal, list_goals, CreateGoalInput, Goal};
use cacash_core::hadith::{get_today_hadith, Hadith};
use cacash_core::health::{calculate_health_score, FinancialHealthScore};
use cacash_core::investments::{list_debts, list_investments, Debt, Investment};
use cacash_core::kids::{list_kid_missions, toggle_mission_completion, KidMission};
use cacash_core::profile::{create_member, list_members, verify_pin, CreateMemberInput, Member};
use cacash_core::transaction::{create_transaction, delete_transaction, list_transactions, CreateTransactionInput, Transaction};
use cacash_core::zakat::{calculate_zakat_fitrah, calculate_zakat_mal, ZakatFitrahCalculation, ZakatMalCalculation};
use tauri::Window;

// Window Controls
#[tauri::command]
pub fn window_minimize(window: Window) {
    window::minimize(&window);
}

#[tauri::command]
pub fn window_maximize(window: Window) {
    window::maximize_or_unmaximize(&window);
}

#[tauri::command]
pub fn window_close(window: Window) {
    window::close(&window);
}

#[tauri::command]
pub fn window_start_dragging(window: Window) {
    window::start_dragging(&window);
}

// Profile & Members
#[tauri::command]
pub fn cmd_list_members() -> Result<Vec<Member>, CashError> {
    list_members()
}

#[tauri::command]
pub fn cmd_create_member(input: CreateMemberInput) -> Result<Member, CashError> {
    create_member(input)
}

#[tauri::command]
pub fn cmd_verify_pin(member_id: String, pin: String) -> Result<bool, CashError> {
    verify_pin(&member_id, &pin)
}

// Accounts
#[tauri::command]
pub fn cmd_list_accounts(requester_member_id: String, is_child: bool) -> Result<Vec<Account>, CashError> {
    list_accounts(&requester_member_id, is_child)
}

#[tauri::command]
pub fn cmd_create_account(input: CreateAccountInput) -> Result<Account, CashError> {
    create_account(input)
}

// Transactions
#[tauri::command]
pub fn cmd_list_transactions(limit: usize) -> Result<Vec<Transaction>, CashError> {
    list_transactions(limit)
}

#[tauri::command]
pub fn cmd_create_transaction(input: CreateTransactionInput) -> Result<Transaction, CashError> {
    create_transaction(input)
}

#[tauri::command]
pub fn cmd_delete_transaction(id: String) -> Result<(), CashError> {
    delete_transaction(&id)
}

// Envelopes Budget
#[tauri::command]
pub fn cmd_list_envelopes() -> Result<Vec<EnvelopeItem>, CashError> {
    list_envelopes()
}

// Goals
#[tauri::command]
pub fn cmd_list_goals() -> Result<Vec<Goal>, CashError> {
    list_goals()
}

#[tauri::command]
pub fn cmd_create_goal(input: CreateGoalInput) -> Result<Goal, CashError> {
    create_goal(input)
}

// Investments & Debts
#[tauri::command]
pub fn cmd_list_investments() -> Result<Vec<Investment>, CashError> {
    list_investments()
}

#[tauri::command]
pub fn cmd_list_debts() -> Result<Vec<Debt>, CashError> {
    list_debts()
}

// Kids Module
#[tauri::command]
pub fn cmd_list_kid_missions(member_id: String) -> Result<Vec<KidMission>, CashError> {
    list_kid_missions(&member_id)
}

#[tauri::command]
pub fn cmd_toggle_mission(id: String) -> Result<bool, CashError> {
    toggle_mission_completion(&id)
}

// Islamic Finance: Zakat & Hadith
#[tauri::command]
pub fn cmd_calculate_zakat_mal(total_assets: i64, short_term_debts: i64, gold_price: i64) -> ZakatMalCalculation {
    calculate_zakat_mal(total_assets, short_term_debts, gold_price)
}

#[tauri::command]
pub fn cmd_calculate_zakat_fitrah(members_count: u32, price_per_person: i64) -> ZakatFitrahCalculation {
    calculate_zakat_fitrah(members_count, price_per_person)
}

#[tauri::command]
pub fn cmd_get_today_hadith() -> Hadith {
    get_today_hadith()
}

// Health Score
#[tauri::command]
pub fn cmd_calculate_health_score(
    emergency_months: f64,
    debt_ratio: f64,
    savings_rate: f64,
    has_riba: bool,
) -> FinancialHealthScore {
    calculate_health_score(emergency_months, debt_ratio, savings_rate, has_riba)
}

// AI Financial Coach
#[tauri::command]
pub async fn cmd_ask_ai(req: AskAiRequest) -> Result<AskAiResponse, CashError> {
    tokio::task::spawn_blocking(move || ask_financial_coach(req))
        .await
        .map_err(|e| CashError::Ai(e.to_string()))?
}

// Export
#[tauri::command]
pub fn cmd_export_transactions() -> Result<String, CashError> {
    let p = export_transactions_csv()?;
    Ok(p.to_string_lossy().to_string())
}

// Settings
#[tauri::command]
pub fn cmd_get_ai_settings() -> Result<AiSettings, CashError> {
    get_ai_settings()
}

#[tauri::command]
pub fn cmd_save_ai_settings(settings: AiSettings) -> Result<(), CashError> {
    save_ai_settings(settings)
}
