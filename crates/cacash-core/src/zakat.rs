use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZakatMalCalculation {
    pub total_zakatable_assets: i64,
    pub deductible_short_term_debts: i64,
    pub net_wealth: i64,
    pub gold_price_per_gram: i64,
    pub nishab_threshold: i64, // 85 * gold_price_per_gram
    pub is_eligible: bool,
    pub zakat_due: i64, // 2.5% of net_wealth if eligible
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZakatFitrahCalculation {
    pub household_members_count: u32,
    pub price_per_person: i64,
    pub total_due: i64,
}

/// Pure Zakat Mal calculation engine without database IO
pub fn calculate_zakat_mal(
    total_assets: i64,
    short_term_debts: i64,
    gold_price_per_gram: i64,
) -> ZakatMalCalculation {
    let net = (total_assets - short_term_debts).max(0);
    let nishab = 85 * gold_price_per_gram;
    let is_eligible = net >= nishab;
    let zakat_due = if is_eligible { (net * 25) / 1000 } else { 0 };

    ZakatMalCalculation {
        total_zakatable_assets: total_assets,
        deductible_short_term_debts: short_term_debts,
        net_wealth: net,
        gold_price_per_gram,
        nishab_threshold: nishab,
        is_eligible,
        zakat_due,
    }
}

pub fn calculate_zakat_fitrah(
    members_count: u32,
    price_per_person: i64,
) -> ZakatFitrahCalculation {
    ZakatFitrahCalculation {
        household_members_count: members_count,
        price_per_person,
        total_due: (members_count as i64) * price_per_person,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zakat_golden_cases() {
        let gold_price = 1_400_000; // Rp 1.400.000 / gram
        let nishab = 85 * gold_price; // Rp 119.000.000

        // Case 1: Below nishab (zero zakat)
        let c1 = calculate_zakat_mal(100_000_000, 0, gold_price);
        assert!(!c1.is_eligible);
        assert_eq!(c1.zakat_due, 0);

        // Case 2: Exactly at nishab (eligible)
        let c2 = calculate_zakat_mal(nishab, 0, gold_price);
        assert!(c2.is_eligible);
        assert_eq!(c2.zakat_due, (nishab * 25) / 1000);

        // Case 3: Above nishab (Rp 200 juta)
        let c3 = calculate_zakat_mal(200_000_000, 0, gold_price);
        assert!(c3.is_eligible);
        assert_eq!(c3.zakat_due, 5_000_000); // 2.5% of 200M

        // Case 4: Above nishab before debt, but below after debt
        let c4 = calculate_zakat_mal(130_000_000, 20_000_000, gold_price);
        assert!(!c4.is_eligible); // net 110M < 119M
        assert_eq!(c4.zakat_due, 0);

        // Case 5: Large wealth (Rp 1 Miliar)
        let c5 = calculate_zakat_mal(1_000_000_000, 0, gold_price);
        assert!(c5.is_eligible);
        assert_eq!(c5.zakat_due, 25_000_000);

        // Fitrah Cases
        let f1 = calculate_zakat_fitrah(4, 45_000);
        assert_eq!(f1.total_due, 180_000);

        let f2 = calculate_zakat_fitrah(5, 50_000);
        assert_eq!(f2.total_due, 250_000);
    }
}
