pub mod account;
pub mod ai;
pub mod budget;
pub mod db;
pub mod error;
pub mod export;
pub mod goals;
pub mod hadith;
pub mod health;
pub mod investments;
pub mod kids;
pub mod money;
pub mod paths;
pub mod profile;
pub mod transaction;
pub mod zakat;

pub use error::CashError;
pub use money::Money;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_core_money_and_family_lifecycle() {
        // 1. Money Math Verification
        let m = Money::from_amount(100_000);
        assert_eq!(m.format_idr(), "Rp 100.000");

        let splits = m.allocate(&[50, 50]);
        assert_eq!(splits.len(), 2);
        assert_eq!(splits[0].amount, 50_000);
        assert_eq!(splits[1].amount, 50_000);

        // Splitting 100 with 3 shares (33, 33, 34)
        let split3 = Money::from_amount(100).allocate(&[1, 1, 1]);
        assert_eq!(split3[0].amount + split3[1].amount + split3[2].amount, 100);

        // 2. Members & Profile
        let members = profile::list_members().expect("List members succeeds");
        assert!(members.len() >= 3); // Ayah, Ibu, Anak

        // 3. Accounts & Computed Balances
        let accounts = account::list_accounts("mem-ayah", false).expect("List accounts succeeds");
        assert!(!accounts.is_empty());

        // 4. Quick Transaction Entry
        let tx = transaction::create_transaction(transaction::CreateTransactionInput {
            account_id: accounts[0].id.clone(),
            member_id: members[0].id.clone(),
            date: None,
            amount: 50_000,
            tx_type: "expense".to_string(),
            category_id: "cat-dapur".to_string(),
            payee: Some("Pasar Tradisional".to_string()),
            note: Some("Belanja sayur bayam & buah pisang".to_string()),
        })
        .expect("Create transaction succeeds");

        assert_eq!(tx.amount, 50_000);

        // 5. Zakat Calculation
        let z = zakat::calculate_zakat_mal(150_000_000, 10_000_000, 1_450_000);
        assert!(z.is_eligible); // net 140M > 85 * 1.45M (123.25M)
        assert_eq!(z.zakat_due, (140_000_000 * 25) / 1000);

        // 6. Curated Daily Hadith
        let h = hadith::get_today_hadith();
        assert!(!h.arabic.is_empty());
        assert!(!h.translation_id.is_empty());

        // 7. Financial Health Score
        let score = health::calculate_health_score(6.0, 15.0, 25.0, false);
        assert!(score.total_score >= 90);
    }
}
