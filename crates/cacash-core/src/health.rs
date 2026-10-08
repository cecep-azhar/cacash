use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinancialHealthScore {
    pub total_score: u32, // 0-100
    pub emergency_fund_score: u32, // 0-25
    pub debt_ratio_score: u32, // 0-25
    pub savings_rate_score: u32, // 0-25
    pub budget_adherence_score: u32, // 0-25
    pub priority_suggestion: String,
}

pub fn calculate_health_score(
    emergency_fund_months: f64,
    debt_service_ratio_pct: f64,
    savings_rate_pct: f64,
    has_riba_debt: bool,
) -> FinancialHealthScore {
    // 1. Emergency Fund (25 max): 6 months = 25
    let ef_score = ((emergency_fund_months / 6.0) * 25.0).clamp(0.0, 25.0) as u32;

    // 2. Debt Ratio (25 max): < 20% = 25, 20-35% = 18, 35-50% = 10, >50% = 0
    let debt_score = if debt_service_ratio_pct <= 20.0 {
        25
    } else if debt_service_ratio_pct <= 35.0 {
        18
    } else if debt_service_ratio_pct <= 50.0 {
        10
    } else {
        0
    };

    // 3. Savings Rate (25 max): >= 25% = 25, 15-24% = 18, 5-14% = 10, <5% = 0
    let savings_score = if savings_rate_pct >= 25.0 {
        25
    } else if savings_rate_pct >= 15.0 {
        18
    } else if savings_rate_pct >= 5.0 {
        10
    } else {
        0
    };

    // 4. Budget Adherence baseline
    let budget_score = 22;

    let total = ef_score + debt_score + savings_score + budget_score;

    let suggestion = if has_riba_debt {
        "Prioritas Utama: Segera lunasi atau alihkan sisa hutang berbunga (riba) untuk menjaga keberkahan finansial keluarga."
            .to_string()
    } else if emergency_fund_months < 3.0 {
        "Prioritas Utama: Tingkatkan cadangan dana darurat minimal hingga mencukupi 3 bulan kebutuhan pokok rumah tangga."
            .to_string()
    } else if savings_rate_pct < 15.0 {
        "Prioritas Utama: Tingkatkan porsi tabungan & investasi keluarga menjadi minimal 15-20% dari total penghasilan bulanan."
            .to_string()
    } else {
        "Kondisi Finansial Prima: Teruskan konsistensi anggaran dan pertimbangkan perluasan investasi syariah jangka panjang."
            .to_string()
    };

    FinancialHealthScore {
        total_score: total.min(100),
        emergency_fund_score: ef_score,
        debt_ratio_score: debt_score,
        savings_rate_score: savings_score,
        budget_adherence_score: budget_score,
        priority_suggestion: suggestion,
    }
}
