use crate::db::get_connection;
use crate::error::CashError;
use chrono::Utc;
use rusqlite::params;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: String,
    pub account_id: String,
    pub account_name: String,
    pub member_id: String,
    pub member_name: String,
    pub date: String,
    pub amount: i64,
    pub tx_type: String, // 'income', 'expense', 'transfer', 'nafkah', 'sedekah', 'zakat'
    pub category_id: String,
    pub category_name: String,
    pub payee: String,
    pub note: String,
    pub created_at: i64,
}

#[derive(Debug, Deserialize)]
pub struct CreateTransactionInput {
    pub account_id: String,
    pub member_id: String,
    pub date: Option<String>,
    pub amount: i64,
    pub tx_type: String,
    pub category_id: String,
    pub payee: Option<String>,
    pub note: Option<String>,
}

pub fn list_transactions(limit: usize) -> Result<Vec<Transaction>, CashError> {
    let pool = get_connection()?;
    let conn = pool.lock();

    let mut stmt = conn.prepare(
        "SELECT t.id, t.account_id, COALESCE(a.name, ''), t.member_id, COALESCE(m.display_name, ''),
                t.date, t.amount, t.tx_type, t.category_id, COALESCE(c.name, ''),
                t.payee, t.note, t.created_at
         FROM transactions t
         LEFT JOIN accounts a ON a.id = t.account_id
         LEFT JOIN members m ON m.id = t.member_id
         LEFT JOIN categories c ON c.id = t.category_id
         ORDER BY t.date DESC, t.created_at DESC
         LIMIT ?1",
    )?;

    let iter = stmt.query_map(params![limit as i64], |row| {
        Ok(Transaction {
            id: row.get(0)?,
            account_id: row.get(1)?,
            account_name: row.get(2)?,
            member_id: row.get(3)?,
            member_name: row.get(4)?,
            date: row.get(5)?,
            amount: row.get(6)?,
            tx_type: row.get(7)?,
            category_id: row.get(8)?,
            category_name: row.get(9)?,
            payee: row.get(10)?,
            note: row.get(11)?,
            created_at: row.get(12)?,
        })
    })?;

    let mut list = Vec::new();
    for t in iter {
        list.push(t?);
    }
    Ok(list)
}

pub fn create_transaction(input: CreateTransactionInput) -> Result<Transaction, CashError> {
    if input.amount <= 0 {
        return Err(CashError::Validation("Nominal transaksi harus lebih dari 0".to_string()));
    }

    let pool = get_connection()?;
    let conn = pool.lock();
    let now = Utc::now().timestamp_millis();
    let id = Uuid::new_v4().to_string();
    let date = input.date.unwrap_or_else(|| Utc::now().format("%Y-%m-%d").to_string());

    conn.execute(
        "INSERT INTO transactions (id, account_id, member_id, date, amount, tx_type, category_id, payee, note, tags, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, '', ?10)",
        params![
            id,
            input.account_id,
            input.member_id,
            date,
            input.amount,
            input.tx_type,
            input.category_id,
            input.payee.unwrap_or_default(),
            input.note.unwrap_or_default(),
            now,
        ],
    )?;

    // Fetch newly created
    let mut stmt = conn.prepare(
        "SELECT t.id, t.account_id, COALESCE(a.name, ''), t.member_id, COALESCE(m.display_name, ''),
                t.date, t.amount, t.tx_type, t.category_id, COALESCE(c.name, ''),
                t.payee, t.note, t.created_at
         FROM transactions t
         LEFT JOIN accounts a ON a.id = t.account_id
         LEFT JOIN members m ON m.id = t.member_id
         LEFT JOIN categories c ON c.id = t.category_id
         WHERE t.id = ?1",
    )?;

    stmt.query_row(params![id], |row| {
        Ok(Transaction {
            id: row.get(0)?,
            account_id: row.get(1)?,
            account_name: row.get(2)?,
            member_id: row.get(3)?,
            member_name: row.get(4)?,
            date: row.get(5)?,
            amount: row.get(6)?,
            tx_type: row.get(7)?,
            category_id: row.get(8)?,
            category_name: row.get(9)?,
            payee: row.get(10)?,
            note: row.get(11)?,
            created_at: row.get(12)?,
        })
    }).map_err(|e| CashError::Db(e.to_string()))
}

pub fn delete_transaction(id: &str) -> Result<(), CashError> {
    let pool = get_connection()?;
    let conn = pool.lock();
    conn.execute("DELETE FROM transactions WHERE id = ?1", params![id])?;
    Ok(())
}
