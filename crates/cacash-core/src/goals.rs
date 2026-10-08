use crate::db::get_connection;
use crate::error::CashError;
use chrono::Utc;
use rusqlite::params;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Goal {
    pub id: String,
    pub title: String,
    pub target_amount: i64,
    pub current_amount: i64,
    pub deadline_date: String,
    pub is_ibadah: bool,
    pub percentage: f64,
}

#[derive(Debug, Deserialize)]
pub struct CreateGoalInput {
    pub title: String,
    pub target_amount: i64,
    pub current_amount: Option<i64>,
    pub deadline_date: String,
    pub is_ibadah: Option<bool>,
}

pub fn list_goals() -> Result<Vec<Goal>, CashError> {
    ensure_default_goals()?;
    let pool = get_connection()?;
    let conn = pool.lock();

    let mut stmt = conn.prepare(
        "SELECT id, title, target_amount, current_amount, deadline_date, is_ibadah FROM goals ORDER BY is_ibadah DESC, target_amount DESC",
    )?;

    let iter = stmt.query_map([], |row| {
        let target: i64 = row.get(2)?;
        let current: i64 = row.get(3)?;
        let pct = if target > 0 {
            (current as f64 / target as f64) * 100.0
        } else {
            0.0
        };

        Ok(Goal {
            id: row.get(0)?,
            title: row.get(1)?,
            target_amount: target,
            current_amount: current,
            deadline_date: row.get(4)?,
            is_ibadah: row.get::<_, i64>(5)? != 0,
            percentage: pct,
        })
    })?;

    let mut list = Vec::new();
    for g in iter {
        list.push(g?);
    }
    Ok(list)
}

pub fn create_goal(input: CreateGoalInput) -> Result<Goal, CashError> {
    let pool = get_connection()?;
    let conn = pool.lock();
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().timestamp_millis();
    let current = input.current_amount.unwrap_or(0);
    let is_ibadah = if input.is_ibadah.unwrap_or(false) { 1 } else { 0 };

    conn.execute(
        "INSERT INTO goals (id, title, target_amount, current_amount, deadline_date, is_ibadah, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![
            id,
            input.title,
            input.target_amount,
            current,
            input.deadline_date,
            is_ibadah,
            now,
        ],
    )?;

    let pct = if input.target_amount > 0 {
        (current as f64 / input.target_amount as f64) * 100.0
    } else {
        0.0
    };

    Ok(Goal {
        id,
        title: input.title,
        target_amount: input.target_amount,
        current_amount: current,
        deadline_date: input.deadline_date,
        is_ibadah: is_ibadah == 1,
        percentage: pct,
    })
}

fn ensure_default_goals() -> Result<(), CashError> {
    let pool = get_connection()?;
    let conn = pool.lock();
    let count: i64 = conn.query_row("SELECT count(*) FROM goals", [], |r| r.get(0))?;
    if count == 0 {
        let now = Utc::now().timestamp_millis();
        let _ = conn.execute(
            "INSERT INTO goals (id, title, target_amount, current_amount, deadline_date, is_ibadah, created_at)
             VALUES ('goal-emergency', 'Dana Darurat 6 Bulan', 60000000, 32000000, '2026-12-31', 0, ?1)",
            params![now],
        );
        let _ = conn.execute(
            "INSERT INTO goals (id, title, target_amount, current_amount, deadline_date, is_ibadah, created_at)
             VALUES ('goal-qurban', 'Tabungan Qurban Idul Adha 1448 H', 4500000, 3000000, '2027-05-15', 1, ?1)",
            params![now],
        );
        let _ = conn.execute(
            "INSERT INTO goals (id, title, target_amount, current_amount, deadline_date, is_ibadah, created_at)
             VALUES ('goal-umrah', 'Tabungan Umrah Keluarga 4 Orang', 120000000, 45000000, '2027-11-20', 1, ?1)",
            params![now],
        );
    }
    Ok(())
}
