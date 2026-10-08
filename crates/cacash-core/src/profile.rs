use crate::db::get_connection;
use crate::error::CashError;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::Utc;
use rusqlite::params;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, Ordering};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Member {
    pub id: String,
    pub display_name: String,
    pub role: String, // 'owner', 'partner', 'child', 'member'
    pub birth_year: i64,
    pub avatar: String,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Deserialize)]
pub struct CreateMemberInput {
    pub display_name: String,
    pub role: String,
    pub birth_year: i64,
    pub pin: String,
    pub avatar: Option<String>,
}

pub fn list_members() -> Result<Vec<Member>, CashError> {
    ensure_default_family()?;
    let pool = get_connection()?;
    let conn = pool.lock();
    let mut stmt = conn.prepare(
        "SELECT id, display_name, role, birth_year, avatar, created_at, updated_at 
         FROM members ORDER BY created_at ASC",
    )?;

    let iter = stmt.query_map([], |row| {
        Ok(Member {
            id: row.get(0)?,
            display_name: row.get(1)?,
            role: row.get(2)?,
            birth_year: row.get(3)?,
            avatar: row.get(4)?,
            created_at: row.get(5)?,
            updated_at: row.get(6)?,
        })
    })?;

    let mut list = Vec::new();
    for m in iter {
        list.push(m?);
    }
    Ok(list)
}

pub fn create_member(input: CreateMemberInput) -> Result<Member, CashError> {
    let pool = get_connection()?;
    let conn = pool.lock();
    let now = Utc::now().timestamp_millis();
    let id = Uuid::new_v4().to_string();

    let pin_hash = hash_pin(&input.pin)?;

    let member = Member {
        id: id.clone(),
        display_name: input.display_name,
        role: input.role,
        birth_year: input.birth_year,
        avatar: input.avatar.unwrap_or_else(|| "user".to_string()),
        created_at: now,
        updated_at: now,
    };

    conn.execute(
        "INSERT INTO members (id, display_name, role, birth_year, pin_hash, avatar, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![
            member.id,
            member.display_name,
            member.role,
            member.birth_year,
            pin_hash,
            member.avatar,
            member.created_at,
            member.updated_at,
        ],
    )?;

    Ok(member)
}

pub fn verify_pin(member_id: &str, input_pin: &str) -> Result<bool, CashError> {
    let pool = get_connection()?;
    let conn = pool.lock();
    let pin_hash: String = conn.query_row(
        "SELECT pin_hash FROM members WHERE id = ?1",
        params![member_id],
        |r| r.get(0),
    ).map_err(|_| CashError::NotFound(format!("Member {member_id} not found")))?;

    let parsed_hash = PasswordHash::new(&pin_hash)
        .map_err(|e| CashError::Auth(format!("Invalid stored hash: {e}")))?;
    Ok(Argon2::default().verify_password(input_pin.as_bytes(), &parsed_hash).is_ok())
}

pub fn hash_pin(pin: &str) -> Result<String, CashError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(pin.as_bytes(), &salt)
        .map_err(|e| CashError::Auth(format!("Gagal hash PIN: {e}")))?
        .to_string();
    Ok(password_hash)
}

static INITIALIZED: AtomicBool = AtomicBool::new(false);

fn ensure_default_family() -> Result<(), CashError> {
    if INITIALIZED.load(Ordering::Relaxed) {
        return Ok(());
    }

    let pool = get_connection()?;
    let conn = pool.lock();
    let count: i64 = conn.query_row("SELECT count(*) FROM members", [], |r| r.get(0))?;
    if count == 0 {
        let now = Utc::now().timestamp_millis();
        let pin_hash = hash_pin("1234")?;

        // 1. Ayah (Owner)
        let _ = conn.execute(
            "INSERT INTO members (id, display_name, role, birth_year, pin_hash, avatar, created_at, updated_at)
             VALUES ('mem-ayah', 'Ayah (Kepala Keluarga)', 'owner', 1985, ?1, 'user-check', ?2, ?2)",
            params![pin_hash, now],
        );

        // 2. Ibu (Partner)
        let _ = conn.execute(
            "INSERT INTO members (id, display_name, role, birth_year, pin_hash, avatar, created_at, updated_at)
             VALUES ('mem-ibu', 'Ibu (Pasangan)', 'partner', 1988, ?1, 'heart', ?2, ?2)",
            params![pin_hash, now],
        );

        // 3. Anak (Child)
        let _ = conn.execute(
            "INSERT INTO members (id, display_name, role, birth_year, pin_hash, avatar, created_at, updated_at)
             VALUES ('mem-anak', 'Fathir (Anak)', 'child', 2016, ?1, 'smile', ?2, ?2)",
            params![pin_hash, now],
        );
    }

    INITIALIZED.store(true, Ordering::Relaxed);
    Ok(())
}
