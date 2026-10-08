use crate::db::get_connection;
use crate::error::CashError;
use chrono::Utc;
use rusqlite::params;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvelopeItem {
    pub id: String,
    pub category_id: String,
    pub category_name: String,
    pub monthly_budget: i64,
    pub spent_amount: i64,
    pub percentage_used: f64,
}

pub fn list_envelopes() -> Result<Vec<EnvelopeItem>, CashError> {
    let pool = get_connection()?;
    let conn = pool.lock();
    let current_month = Utc::now().format("%Y-%m").to_string();

    let mut stmt = conn.prepare(
        "SELECT e.id, e.category_id, c.name, e.monthly_budget,
                COALESCE((
                    SELECT SUM(t.amount) FROM transactions t 
                    WHERE t.category_id = e.category_id 
                      AND t.tx_type IN ('expense', 'nafkah', 'sedekah', 'zakat')
                      AND strftime('%Y-%m', t.date) = ?1
                ), 0) as spent
         FROM envelopes e
         JOIN categories c ON c.id = e.category_id
         ORDER BY e.monthly_budget DESC",
    )?;

    let iter = stmt.query_map(params![current_month], |row| {
        let budget: i64 = row.get(3)?;
        let spent: i64 = row.get(4)?;
        let pct = if budget > 0 {
            (spent as f64 / budget as f64) * 100.0
        } else {
            0.0
        };

        Ok(EnvelopeItem {
            id: row.get(0)?,
            category_id: row.get(1)?,
            category_name: row.get(2)?,
            monthly_budget: budget,
            spent_amount: spent,
            percentage_used: pct,
        })
    })?;

    let mut list = Vec::new();
    for e in iter {
        list.push(e?);
    }

    if list.is_empty() {
        // Create initial default envelopes if empty
        ensure_default_envelopes(&conn)?;
        return list_envelopes();
    }

    Ok(list)
}

fn ensure_default_envelopes(conn: &rusqlite::Connection) -> Result<(), CashError> {
    let defaults = [
        ("env-dapur", "cat-dapur", 4500000),
        ("env-tagihan", "cat-tagihan", 1500000),
        ("env-transport", "cat-transport", 1000000),
        ("env-sedekah", "cat-sedekah", 500000),
        ("env-pendidikan", "cat-pendidikan", 2000000),
    ];

    for (id, cat_id, budget) in defaults {
        let _ = conn.execute(
            "INSERT OR IGNORE INTO envelopes (id, category_id, monthly_budget, rollover) VALUES (?1, ?2, ?3, 0)",
            params![id, cat_id, budget],
        );
    }
    Ok(())
}
