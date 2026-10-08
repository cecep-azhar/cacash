mod commands;
mod window;

use tauri::Builder;

pub fn run() {
    let mut builder = Builder::default();

    #[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
    {
        builder = builder
            .plugin(tauri_plugin_dialog::init())
            .plugin(tauri_plugin_fs::init())
            .plugin(tauri_plugin_process::init());
    }

    builder
        .invoke_handler(tauri::generate_handler![
            // Window controls
            commands::window_minimize,
            commands::window_maximize,
            commands::window_close,
            commands::window_start_dragging,
            // Profile & Members
            commands::cmd_list_members,
            commands::cmd_create_member,
            commands::cmd_verify_pin,
            // Accounts
            commands::cmd_list_accounts,
            commands::cmd_create_account,
            // Transactions
            commands::cmd_list_transactions,
            commands::cmd_create_transaction,
            commands::cmd_delete_transaction,
            // Envelopes Budget
            commands::cmd_list_envelopes,
            // Goals
            commands::cmd_list_goals,
            commands::cmd_create_goal,
            // Investments & Debts
            commands::cmd_list_investments,
            commands::cmd_list_debts,
            // Kids
            commands::cmd_list_kid_missions,
            commands::cmd_toggle_mission,
            // Islamic Finance
            commands::cmd_calculate_zakat_mal,
            commands::cmd_calculate_zakat_fitrah,
            commands::cmd_get_today_hadith,
            // Health Score
            commands::cmd_calculate_health_score,
            // AI Financial Coach
            commands::cmd_ask_ai,
            // Export
            commands::cmd_export_transactions,
            // Settings
            commands::cmd_get_ai_settings,
            commands::cmd_save_ai_settings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running CACash application");
}
