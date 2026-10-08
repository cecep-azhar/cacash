use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Money {
    /// Nominal in integer minor units (for IDR: 1 unit = Rp 1)
    pub amount: i64,
}

impl Money {
    pub const fn zero() -> Self {
        Self { amount: 0 }
    }

    pub const fn from_amount(amount: i64) -> Self {
        Self { amount }
    }

    pub fn format_idr(&self) -> String {
        let is_negative = self.amount < 0;
        let abs_val = self.amount.abs();
        let s = abs_val.to_string();
        let mut out = String::new();
        let len = s.len();

        for (i, c) in s.chars().enumerate() {
            out.push(c);
            let remaining = len - 1 - i;
            if remaining > 0 && remaining % 3 == 0 {
                out.push('.');
            }
        }

        if is_negative {
            format!("-Rp {}", out)
        } else {
            format!("Rp {}", out)
        }
    }

    /// Allocates money according to percentage shares (summing to 100), distributing remainder deterministically.
    pub fn allocate(&self, shares: &[u32]) -> Vec<Money> {
        if shares.is_empty() {
            return Vec::new();
        }

        let total_shares: u32 = shares.iter().sum();
        if total_shares == 0 {
            return vec![Money::zero(); shares.len()];
        }

        let mut results = Vec::with_capacity(shares.len());
        let mut allocated = 0i64;

        for &share in shares {
            let part = (self.amount * share as i64) / total_shares as i64;
            results.push(Money::from_amount(part));
            allocated += part;
        }

        // Distribute remainder cent by cent to the highest shares
        let remainder = self.amount - allocated;
        if remainder > 0 {
            for i in 0..(remainder as usize).min(results.len()) {
                if let Some(r) = results.get_mut(i) {
                    r.amount += 1;
                }
            }
        }

        results
    }
}
