use crate::db::get_connection;
use crate::error::CashError;
use chrono::Utc;
use rusqlite::params;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, Ordering};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: String,
    pub name: String,
    pub acc_type: String, // 'bank', 'ewallet', 'cash', 'credit_card'
    pub currency: String,
    pub visibility: String, // 'shared', 'private_summary', 'private'
    pub opening_balance: i64,
    pub current_balance: i64, // calculated
    pub is_archived: bool,
    pub created_at: i64,
}

#[derive(Debug, Deserialize)]
pub struct CreateAccountInput {
    pub name: String,
    pub acc_type: String,
    pub currency: Option<String>,
    pub visibility: Option<String>,
    pub opening_balance: Option<i64>,
    pub owner_member_id: String,
}

pub fn list_accounts(requester_member_id: &str, is_child: bool) -> Result<Vec<Account>, CashError> {
    ensure_default_accounts()?;
    let pool = get_connection()?;
    let conn = pool.lock();

    let mut stmt = conn.prepare(
        "SELECT a.id, a.name, a.acc_type, a.currency, a.visibility, a.opening_balance, a.is_archived, a.created_at,
                (a.opening_balance + COALESCE((
                    SELECT SUM(CASE 
                        WHEN t.tx_type = 'income' THEN t.amount 
                        WHEN t.tx_type IN ('expense', 'nafkah', 'sedekah', 'zakat') THEN -t.amount 
                        ELSE 0 
                    END) FROM transactions t WHERE t.account_id = a.id
                ), 0)) as current_balance,
                EXISTS(SELECT 1 FROM account_shares s WHERE s.account_id = a.id AND s.member_id = ?1) as is_owner
         FROM accounts a WHERE a.is_archived = 0 ORDER BY a.created_at ASC",
    )?;

    let iter = stmt.query_map(params![requester_member_id], |row| {
        let is_owner: bool = row.get(9)?;
        let visibility: String = row.get(4)?;

        // Filter based on visibility
        if is_child {
            // Children can only see accounts they own (e.g. kid piggy bank)
            if !is_owner {
                return Ok(None);
            }
        } else if visibility == "private" && !is_owner {
            // Private account is completely hidden from non-owners
            return Ok(None);
        }

        let acc = Account {
            id: row.get(0)?,
            name: row.get(1)?,
            acc_type: row.get(2)?,
            currency: row.get(3)?,
            visibility,
            opening_balance: row.get(5)?,
            is_archived: row.get::<_, i64>(6)? != 0,
            created_at: row.get(7)?,
            current_balance: row.get(8)?,
        };
        Ok(Some(acc))
    })?;

    let mut list = Vec::new();
    for a in iter {
        if let Some(acc) = a? {
            list.push(acc);
        }
    }
    Ok(list)
}

pub fn create_account(input: CreateAccountInput) -> Result<Account, CashError> {
    let pool = get_connection()?;
    let conn = pool.lock();
    let now = Utc::now().timestamp_millis();
    let id = Uuid::new_v4().to_string();
    let opening = input.opening_balance.unwrap_or(0);

    let acc = Account {
        id: id.clone(),
        name: input.name,
        acc_type: input.acc_type,
        currency: input.currency.unwrap_or_else(|| "IDR".to_string()),
        visibility: input.visibility.unwrap_or_else(|| "shared".to_string()),
        opening_balance: opening,
        current_balance: opening,
        is_archived: false,
        created_at: now,
    };

    conn.execute(
        "INSERT INTO accounts (id, name, acc_type, currency, visibility, opening_balance, is_archived, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, 0, ?7, ?7)",
        params![
            acc.id,
            acc.name,
            acc.acc_type,
            acc.currency,
            acc.visibility,
            acc.opening_balance,
            now,
        ],
    )?;

    // Insert 100% share for creator
    conn.execute(
        "INSERT INTO account_shares (account_id, member_id, share_percent) VALUES (?1, ?2, 100)",
        params![acc.id, input.owner_member_id],
    )?;

    Ok(acc)
}

static ACCOUNTS_INITIALIZED: AtomicBool = AtomicBool::new(false);

fn ensure_default_accounts() -> Result<(), CashError> {
    if ACCOUNTS_INITIALIZED.load(Ordering::Relaxed) {
        return Ok(());
    }

    let pool = get_connection()?;
    let conn = pool.lock();
    let count: i64 = conn.query_row("SELECT count(*) FROM accounts", [], |r| r.get(0))?;
    if count == 0 {
        let now = Utc::now().timestamp_millis();

        // 1. Rekening Bersama Bank BCA (Shared 50% Ayah, 50% Ibu)
        let _ = conn.execute(
            "INSERT INTO accounts (id, name, acc_type, currency, visibility, opening_balance, is_archived, created_at, updated_at)
             VALUES ('acc-bca', 'BCA Operasional Keluarga', 'bank', 'IDR', 'shared', 15000000, 0, ?1, ?1)",
            params![now],
        );
        let _ = conn.execute(
            "INSERT INTO account_shares (account_id, member_id, share_percent) VALUES ('acc-bca', 'mem-ayah', 50)",
            [],
        );
        let _ = conn.execute(
            "INSERT INTO account_shares (account_id, member_id, share_percent) VALUES ('acc-bca', 'mem-ibu', 50)",
            [],
        );

        // 2. Dompet Tunai / Cash (Shared)
        let _ = conn.execute(
            "INSERT INTO accounts (id, name, acc_type, currency, visibility, opening_balance, is_archived, created_at, updated_at)
             VALUES ('acc-cash', 'Dompet Tunai Brankas', 'cash', 'IDR', 'shared', 2500000, 0, ?1, ?1)",
            params![now],
        );
        let _ = conn.execute(
            "INSERT INTO account_shares (account_id, member_id, share_percent) VALUES ('acc-cash', 'mem-ibu', 100)",
            [],
        );

        // 3. Tabungan Pribadi Ibu (Private)
        let _ = conn.execute(
            "INSERT INTO accounts (id, name, acc_type, currency, visibility, opening_balance, is_archived, created_at, updated_at)
             VALUES ('acc-ibu-priv', 'BSI Tabungan Mandiri Ibu', 'bank', 'IDR', 'private', 8000000, 0, ?1, ?1)",
            params![now],
        );
        let _ = conn.execute(
            "INSERT INTO account_shares (account_id, member_id, share_percent) VALUES ('acc-ibu-priv', 'mem-ibu', 100)",
            [],
        );

        // 4. Celengan Fathir (Child)
        let _ = conn.execute(
            "INSERT INTO accounts (id, name, acc_type, currency, visibility, opening_balance, is_archived, created_at, updated_at)
             VALUES ('acc-anak-celengan', 'Celengan Ayam Fathir', 'cash', 'IDR', 'shared', 350000, 0, ?1, ?1)",
            params![now],
        );
        let _ = conn.execute(
            "INSERT INTO account_shares (account_id, member_id, share_percent) VALUES ('acc-anak-celengan', 'mem-anak', 100)",
            [],
        );
    }

    ACCOUNTS_INITIALIZED.store(true, Ordering::Relaxed);
    Ok(())
}
