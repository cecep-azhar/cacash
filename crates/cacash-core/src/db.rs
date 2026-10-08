use crate::error::CashError;
use crate::paths;
use rusqlite::Connection;
use std::sync::OnceLock;

static DB_POOL: OnceLock<parking_lot::Mutex<Connection>> = OnceLock::new();

pub fn get_connection() -> Result<&'static parking_lot::Mutex<Connection>, CashError> {
    if let Some(pool) = DB_POOL.get() {
        return Ok(pool);
    }

    let path = paths::db_path();
    let conn = Connection::open(&path).map_err(|e| CashError::Db(e.to_string()))?;

    // SQLCipher default encryption key if configured or default local zero-knowledge key
    let vault_key = std::env::var("CACASH_VAULT_KEY").unwrap_or_else(|_| "cacash-sovereign-vault-key-2026".to_string());
    let _ = conn.execute_batch(&format!("PRAGMA key = '{}';", vault_key.replace('\'', "''")));

    conn.execute_batch(
        r#"
        PRAGMA journal_mode = WAL;
        PRAGMA synchronous = NORMAL;
        PRAGMA foreign_keys = ON;
        "#,
    )?;

    init_schema(&conn)?;

    let mutex = parking_lot::Mutex::new(conn);
    let _ = DB_POOL.set(mutex);
    Ok(DB_POOL.get().expect("DB pool initialized"))
}

fn init_schema(conn: &Connection) -> Result<(), CashError> {
    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS settings (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS members (
            id TEXT PRIMARY KEY,
            display_name TEXT NOT NULL,
            role TEXT NOT NULL, -- 'owner', 'partner', 'child', 'member'
            birth_year INTEGER NOT NULL,
            pin_hash TEXT NOT NULL,
            avatar TEXT NOT NULL DEFAULT '',
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL
        );

        CREATE TABLE IF NOT EXISTS accounts (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            acc_type TEXT NOT NULL, -- 'bank', 'ewallet', 'cash', 'credit_card'
            currency TEXT NOT NULL DEFAULT 'IDR',
            visibility TEXT NOT NULL DEFAULT 'shared', -- 'shared', 'private_summary', 'private'
            opening_balance INTEGER NOT NULL DEFAULT 0,
            is_archived INTEGER NOT NULL DEFAULT 0,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL
        );

        CREATE TABLE IF NOT EXISTS account_shares (
            account_id TEXT NOT NULL,
            member_id TEXT NOT NULL,
            share_percent INTEGER NOT NULL, -- 0-100
            PRIMARY KEY (account_id, member_id),
            FOREIGN KEY (account_id) REFERENCES accounts(id) ON DELETE CASCADE,
            FOREIGN KEY (member_id) REFERENCES members(id) ON DELETE CASCADE
        );

        CREATE TABLE IF NOT EXISTS categories (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            cat_type TEXT NOT NULL, -- 'income', 'expense'
            icon TEXT NOT NULL DEFAULT 'tag',
            is_system INTEGER NOT NULL DEFAULT 0
        );

        CREATE TABLE IF NOT EXISTS transactions (
            id TEXT PRIMARY KEY,
            account_id TEXT NOT NULL,
            member_id TEXT NOT NULL,
            date TEXT NOT NULL,
            amount INTEGER NOT NULL, -- integer minor units (Rp)
            tx_type TEXT NOT NULL, -- 'income', 'expense', 'transfer', 'nafkah', 'sedekah', 'zakat'
            category_id TEXT NOT NULL,
            payee TEXT NOT NULL DEFAULT '',
            note TEXT NOT NULL DEFAULT '',
            tags TEXT NOT NULL DEFAULT '',
            created_at INTEGER NOT NULL,
            FOREIGN KEY (account_id) REFERENCES accounts(id) ON DELETE CASCADE,
            FOREIGN KEY (member_id) REFERENCES members(id) ON DELETE CASCADE
        );

        CREATE TABLE IF NOT EXISTS envelopes (
            id TEXT PRIMARY KEY,
            category_id TEXT NOT NULL,
            member_id TEXT,
            monthly_budget INTEGER NOT NULL,
            rollover INTEGER NOT NULL DEFAULT 0,
            FOREIGN KEY (category_id) REFERENCES categories(id) ON DELETE CASCADE
        );

        CREATE TABLE IF NOT EXISTS goals (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            target_amount INTEGER NOT NULL,
            current_amount INTEGER NOT NULL DEFAULT 0,
            deadline_date TEXT NOT NULL,
            is_ibadah INTEGER NOT NULL DEFAULT 0,
            created_at INTEGER NOT NULL
        );

        CREATE TABLE IF NOT EXISTS investments (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            inv_type TEXT NOT NULL, -- 'gold', 'stocks', 'mutual_fund', 'property', 'vehicle'
            units TEXT NOT NULL DEFAULT '1',
            cost_basis INTEGER NOT NULL,
            current_value INTEGER NOT NULL,
            updated_at INTEGER NOT NULL
        );

        CREATE TABLE IF NOT EXISTS debts (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            principal_amount INTEGER NOT NULL,
            remaining_amount INTEGER NOT NULL,
            interest_rate TEXT NOT NULL DEFAULT '0%',
            is_riba INTEGER NOT NULL DEFAULT 0,
            due_date TEXT NOT NULL,
            created_at INTEGER NOT NULL
        );

        CREATE TABLE IF NOT EXISTS kid_missions (
            id TEXT PRIMARY KEY,
            member_id TEXT NOT NULL,
            title TEXT NOT NULL,
            reward_amount INTEGER NOT NULL,
            is_approved INTEGER NOT NULL DEFAULT 0,
            is_completed INTEGER NOT NULL DEFAULT 0,
            FOREIGN KEY (member_id) REFERENCES members(id) ON DELETE CASCADE
        );

        CREATE TABLE IF NOT EXISTS hadiths (
            id TEXT PRIMARY KEY,
            number INTEGER NOT NULL,
            narrator TEXT NOT NULL,
            arabic TEXT NOT NULL,
            translation_id TEXT NOT NULL,
            theme TEXT NOT NULL
        );

        CREATE INDEX IF NOT EXISTS idx_tx_account ON transactions(account_id);
        CREATE INDEX IF NOT EXISTS idx_tx_member ON transactions(member_id);
        CREATE INDEX IF NOT EXISTS idx_tx_date ON transactions(date);
        "#,
    )?;

    // Insert Default System Settings & Categories
    conn.execute(
        r#"INSERT OR IGNORE INTO settings (key, value) VALUES ('mode', 'muslim')"#,
        [],
    )?;
    conn.execute(
        r#"INSERT OR IGNORE INTO settings (key, value) VALUES ('base_currency', 'IDR')"#,
        [],
    )?;
    conn.execute(
        r#"INSERT OR IGNORE INTO settings (key, value) VALUES ('ai_base_url', 'http://localhost:20128/v1')"#,
        [],
    )?;
    conn.execute(
        r#"INSERT OR IGNORE INTO settings (key, value) VALUES ('gold_price_per_gram', '1450000')"#,
        [],
    )?;

    // Insert Default Categories
    let default_cats = [
        ("cat-gaji", "Gaji & Pemasukan Utama", "income", "briefcase", 1),
        ("cat-bisnis", "Bisnis & Sampingan", "income", "trending-up", 1),
        ("cat-dapur", "Kebutuhan Dapur & Belanja", "expense", "shopping-cart", 1),
        ("cat-tagihan", "Listrik, Air & Internet", "expense", "zap", 1),
        ("cat-transport", "Bensin & Transportasi", "expense", "truck", 1),
        ("cat-pendidikan", "Sekolah & Kursus Anak", "expense", "book", 1),
        ("cat-kesehatan", "Obat & Kesehatan", "expense", "activity", 1),
        ("cat-nafkah", "Nafkah Pasangan & Anak", "expense", "heart", 1),
        ("cat-sedekah", "Infaq & Sedekah Harian", "expense", "gift", 1),
        ("cat-zakat", "Zakat Mal & Fitrah", "expense", "award", 1),
        ("cat-hiburan", "Jajan & Rekreasi Keluarga", "expense", "smile", 1),
    ];

    for (id, name, cat_type, icon, is_sys) in default_cats {
        conn.execute(
            "INSERT OR IGNORE INTO categories (id, name, cat_type, icon, is_system) VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![id, name, cat_type, icon, is_sys],
        )?;
    }

    Ok(())
}
