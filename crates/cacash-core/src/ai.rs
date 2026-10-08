use crate::db::get_connection;
use crate::error::CashError;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiSettings {
    pub base_url: String,
    pub api_key: String,
    pub model: String,
}

#[derive(Debug, Deserialize)]
pub struct AskAiRequest {
    pub user_prompt: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AskAiResponse {
    pub text: String,
    pub model: String,
}

pub fn get_ai_settings() -> Result<AiSettings, CashError> {
    let pool = get_connection()?;
    let conn = pool.lock();

    let base_url: String = conn.query_row(
        "SELECT value FROM settings WHERE key = 'ai_base_url'",
        [],
        |r| r.get(0),
    ).unwrap_or_else(|_| "http://localhost:20128/v1".to_string());

    let api_key: String = conn.query_row(
        "SELECT value FROM settings WHERE key = 'ai_api_key'",
        [],
        |r| r.get(0),
    ).unwrap_or_default();

    let model: String = conn.query_row(
        "SELECT value FROM settings WHERE key = 'ai_model'",
        [],
        |r| r.get(0),
    ).unwrap_or_else(|_| "gpt-4o".to_string());

    Ok(AiSettings {
        base_url,
        api_key,
        model,
    })
}

pub fn save_ai_settings(settings: AiSettings) -> Result<(), CashError> {
    let pool = get_connection()?;
    let conn = pool.lock();
    conn.execute(
        "INSERT INTO settings (key, value) VALUES ('ai_base_url', ?1) ON CONFLICT(key) DO UPDATE SET value = excluded.value",
        rusqlite::params![settings.base_url],
    )?;
    conn.execute(
        "INSERT INTO settings (key, value) VALUES ('ai_api_key', ?1) ON CONFLICT(key) DO UPDATE SET value = excluded.value",
        rusqlite::params![settings.api_key],
    )?;
    conn.execute(
        "INSERT INTO settings (key, value) VALUES ('ai_model', ?1) ON CONFLICT(key) DO UPDATE SET value = excluded.value",
        rusqlite::params![settings.model],
    )?;
    Ok(())
}

pub fn ask_financial_coach(req: AskAiRequest) -> Result<AskAiResponse, CashError> {
    let settings = get_ai_settings()?;
    let base_url = settings.base_url.trim().trim_end_matches('/');

    if base_url.is_empty() {
        return Err(CashError::Ai("AI Base URL belum diisi di Pengaturan.".to_string()));
    }

    // Strict Guardrail System Prompt
    let system_instruction = r#"You are the CACash Family AI Financial Coach.
GUIDELINES (STRICT):
1. PRIVACY FIRST: You only give general educational guidance based on principles of family budgeting, emergency funds, and debt elimination.
2. NO RELIGIOUS HALLUCINATION: You MUST NOT invent, quote from unverified memory, or paraphrase Quranic verses, Hadiths, or Fatwas. For religious matters, strictly refer the user to certified Islamic scholars or official zakat institutions (BAZNAS).
3. NO DIRECT INVESTMENT ADVICE: Offer general literacy, not specific stock/crypto trading instructions.
4. Tone: Polite, warm, pragmatic, encouraging, Indonesian language (Bahasa Indonesia).
"#;

    let payload = serde_json::json!({
        "model": settings.model,
        "messages": [
            { "role": "system", "content": system_instruction },
            { "role": "user", "content": req.user_prompt }
        ],
        "temperature": 0.6,
        "stream": false
    });

    let url = format!("{base_url}/chat/completions");

    let config = ureq::Agent::config_builder()
        .timeout_global(Some(Duration::from_secs(60)))
        .build();
    let agent: ureq::Agent = config.into();

    let mut request = agent.post(&url).header("Content-Type", "application/json");
    if !settings.api_key.trim().is_empty() {
        request = request.header("Authorization", &format!("Bearer {}", settings.api_key.trim()));
    }

    let mut response = request.send_json(&payload).map_err(|e| {
        CashError::Ai(format!("Gagal menghubungi AI provider di {url}: {e}"))
    })?;

    let body = response.body_mut().read_to_string().map_err(|e| {
        CashError::Ai(format!("Gagal membaca respons AI: {e}"))
    })?;

    let res_json: serde_json::Value = serde_json::from_str(&body).map_err(|e| {
        CashError::Ai(format!("Respons AI bukan JSON valid: {e}"))
    })?;

    let content = res_json["choices"][0]["message"]["content"]
        .as_str()
        .ok_or_else(|| {
            CashError::Ai(format!("Tidak ditemukan konten teks dalam respons AI: {body}"))
        })?
        .to_string();

    Ok(AskAiResponse {
        text: content,
        model: settings.model,
    })
}
