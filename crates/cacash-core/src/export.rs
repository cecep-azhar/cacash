use crate::error::CashError;
use crate::paths;
use crate::transaction::list_transactions;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

pub fn export_transactions_csv() -> Result<PathBuf, CashError> {
    let txs = list_transactions(5000)?;
    let filename = format!("transaksi_{}.csv", chrono::Utc::now().format("%Y%m%d_%H%M%S"));
    let out_path = paths::export_dir().join(&filename);

    let mut file = File::create(&out_path)?;
    writeln!(file, "ID,Tanggal,Tipe,Kategori,Akun,Anggota,Nominal,Pihak,Catatan")?;

    for t in txs {
        writeln!(
            file,
            "\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",{},\"{}\",\"{}\"",
            t.id, t.date, t.tx_type, t.category_name, t.account_name, t.member_name, t.amount, t.payee, t.note
        )?;
    }

    Ok(out_path)
}
