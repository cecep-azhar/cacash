use crate::db::get_connection;
use crate::error::CashError;
use chrono::Utc;
use rusqlite::params;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Investment {
    pub id: String,
    pub title: String,
    pub inv_type: String, // 'gold', 'stocks', 'mutual_fund', 'property', 'vehicle'
    pub units: String,
    pub cost_basis: i64,
    pub current_value: i64,
    pub gain_loss: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Debt {
    pub id: String,
    pub title: String,
    pub principal_amount: i64,
    pub remaining_amount: i64,
    pub interest_rate: String,
    pub is_riba: bool,
    pub due_date: String,
}

pub fn list_investments() -> Result<Vec<Investment>, CashError> {
    ensure_default_investments()?;
    let pool = get_connection()?;
    let conn = pool.lock();

    let mut stmt = conn.prepare(
        "SELECT id, title, inv_type, units, cost_basis, current_value FROM investments ORDER BY current_value DESC",
    )?;

    let iter = stmt.query_map([], |row| {
        let cost: i64 = row.get(4)?;
        let curr: i64 = row.get(5)?;
        Ok(Investment {
            id: row.get(0)?,
            title: row.get(1)?,
            inv_type: row.get(2)?,
            units: row.get(3)?,
            cost_basis: cost,
            current_value: curr,
            gain_loss: curr - cost,
        })
    })?;

    let mut list = Vec::new();
    for inv in iter {
        list.push(inv?);
    }
    Ok(list)
}

pub fn list_debts() -> Result<Vec<Debt>, CashError> {
    ensure_default_debts()?;
    let pool = get_connection()?;
    let conn = pool.lock();

    let mut stmt = conn.prepare(
        "SELECT id, title, principal_amount, remaining_amount, interest_rate, is_riba, due_date FROM debts ORDER BY remaining_amount DESC",
    )?;

    let iter = stmt.query_map([], |row| {
        Ok(Debt {
            id: row.get(0)?,
            title: row.get(1)?,
            principal_amount: row.get(2)?,
            remaining_amount: row.get(3)?,
            interest_rate: row.get(4)?,
            is_riba: row.get::<_, i64>(5)? != 0,
            due_date: row.get(6)?,
        })
    })?;

    let mut list = Vec::new();
    for d in iter {
        list.push(d?);
    }
    Ok(list)
}

fn ensure_default_investments() -> Result<(), CashError> {
    let pool = get_connection()?;
    let conn = pool.lock();
    let count: i64 = conn.query_row("SELECT count(*) FROM investments", [], |r| r.get(0))?;
    if count == 0 {
        let now = Utc::now().timestamp_millis();
        let _ = conn.execute(
            "INSERT INTO investments (id, title, inv_type, units, cost_basis, current_value, updated_at)
             VALUES ('inv-gold', 'Emas Batangan Antam Logam Mulia', 'gold', '100 gram', 125000000, 145000000, ?1)",
            params![now],
        );
        let _ = conn.execute(
            "INSERT INTO investments (id, title, inv_type, units, cost_basis, current_value, updated_at)
             VALUES ('inv-sukuk', 'Sukuk Tabungan Ritel ST011 Syariah', 'mutual_fund', '25 unit', 25000000, 25000000, ?1)",
            params![now],
        );
        let _ = conn.execute(
            "INSERT INTO investments (id, title, inv_type, units, cost_basis, current_value, updated_at)
             VALUES ('inv-vario', 'Motor Honda Vario 125', 'vehicle', '1 unit', 18000000, 14000000, ?1)",
            params![now],
        );
    }
    Ok(())
}

fn ensure_default_debts() -> Result<(), CashError> {
    let pool = get_connection()?;
    let conn = pool.lock();
    let count: i64 = conn.query_row("SELECT count(*) FROM debts", [], |r| r.get(0))?;
    if count == 0 {
        let now = Utc::now().timestamp_millis();
        let _ = conn.execute(
            "INSERT INTO debts (id, title, principal_amount, remaining_amount, interest_rate, is_riba, due_date, created_at)
             VALUES ('debt-kpr-syariah', 'Cicilan Rumah KPRS Hasanah City', 350000000, 210000000, '0% (Akad Murabahah Margin Tetap)', 0, '2032-12-01', ?1)",
            params![now],
        );
        let _ = conn.execute(
            "INSERT INTO debts (id, title, principal_amount, remaining_amount, interest_rate, is_riba, due_date, created_at)
             VALUES ('debt-cc-lama', 'Sisa Tagihan Kartu Kredit Lama (Prioritas Pelunasan!)', 5000000, 1850000, '2.25%/bulan', 1, '2026-11-15', ?1)",
            params![now],
        );
    }
    Ok(())
}
