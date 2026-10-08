use crate::db::get_connection;
use crate::error::CashError;
use rusqlite::params;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KidMission {
    pub id: String,
    pub member_id: String,
    pub title: String,
    pub reward_amount: i64,
    pub is_approved: bool,
    pub is_completed: bool,
}

pub fn list_kid_missions(member_id: &str) -> Result<Vec<KidMission>, CashError> {
    ensure_default_missions()?;
    let pool = get_connection()?;
    let conn = pool.lock();

    let mut stmt = conn.prepare(
        "SELECT id, member_id, title, reward_amount, is_approved, is_completed 
         FROM kid_missions WHERE member_id = ?1 ORDER BY is_completed ASC, reward_amount DESC",
    )?;

    let iter = stmt.query_map(params![member_id], |row| {
        Ok(KidMission {
            id: row.get(0)?,
            member_id: row.get(1)?,
            title: row.get(2)?,
            reward_amount: row.get(3)?,
            is_approved: row.get::<_, i64>(4)? != 0,
            is_completed: row.get::<_, i64>(5)? != 0,
        })
    })?;

    let mut list = Vec::new();
    for m in iter {
        list.push(m?);
    }
    Ok(list)
}

pub fn toggle_mission_completion(id: &str) -> Result<bool, CashError> {
    let pool = get_connection()?;
    let conn = pool.lock();
    let current: i64 = conn.query_row(
        "SELECT is_completed FROM kid_missions WHERE id = ?1",
        params![id],
        |r| r.get(0),
    )?;
    let new_val = if current == 0 { 1 } else { 0 };
    conn.execute(
        "UPDATE kid_missions SET is_completed = ?1 WHERE id = ?2",
        params![new_val, id],
    )?;
    Ok(new_val == 1)
}

fn ensure_default_missions() -> Result<(), CashError> {
    let pool = get_connection()?;
    let conn = pool.lock();
    let count: i64 = conn.query_row("SELECT count(*) FROM kid_missions", [], |r| r.get(0))?;
    if count == 0 {
        let missions = [
            ("mis-1", "mem-anak", "Nabung Rp 5.000 setiap hari selama seminggu", 15000),
            ("mis-2", "mem-anak", "Sedekahkan 10% uang saku ke kotak infaq masjid", 10000),
            ("mis-3", "mem-anak", "Bantu rapikan kamar dan tempat tidur sendiri 7 hari", 20000),
            ("mis-4", "mem-anak", "Bandingkan harga 2 barang sebelum memutuskan jajan", 5000),
        ];

        for (id, mem_id, title, reward) in missions {
            let _ = conn.execute(
                "INSERT OR IGNORE INTO kid_missions (id, member_id, title, reward_amount, is_approved, is_completed)
                 VALUES (?1, ?2, ?3, ?4, 1, 0)",
                params![id, mem_id, title, reward],
            );
        }
    }
    Ok(())
}
