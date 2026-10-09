# CACash Family — Sovereign Family Cashflow & Islamic Wealth Engine

[![Release](https://img.shields.io/badge/release-v0.1.0-10b981.svg)](https://github.com/cecep-azhar/cacash)
[![License](https://img.shields.io/badge/license-Proprietary-blue.svg)]()
[![Platform](https://img.shields.io/badge/platform-Linux%20|%20Windows%20|%20macOS%20|%20Android-green.svg)]()
[![CADS](https://img.shields.io/badge/CADS-v1.0%20Compliant-purple.svg)]()

> Sovereign, Local-First Family Finance & Islamic Wealth Management Studio. Zero-Knowledge, perlindungan privasi multi-profil, kalkulator Zakat Mal/Fitrah presisi tinggi, pelacak hutang anti-riba, dan literasi finansial anak.

---

## 🌟 Fitur Utama

- **Multi-Profile & PIN Access Control**:
  - Profil keluarga mandiri: Kepala Keluarga (Ayah), Pasangan (Ibu), Anak (Kids), dan Anggota.
  - Visibilitas data bertingkat (`shared`, `private_summary`, `private`). Privasi pasangan terjaga tanpa mengorbankan gambaran keuangan keluarga.
- **Strict Integer Money Math**:
  - Nol floating-point drift. Seluruh perhitungan uang menggunakan minor unit integer murni (Rupiah/Sen).
- **Amplop Anggaran (Envelope Budgeting)**:
  - Alokasi anggaran bulanan zero-based dengan peringatan ambang batas konsumsi (80% dan 100%).
- **Multi-Rekening & Kepemilikan Bersama**:
  - Rekening bank, e-wallet, brankas tunai fisik, dan saham kepemilikan bersama (joint shares misal 50/50 Ayah & Ibu).
- **Modul Keuangan Syariah & Zakat**:
  - Mesin hitung Zakat Mal otomatis berdasarkan nishab 85 gram emas dan haul 1 tahun Hijriah.
  - Kalkulator Zakat Fitrah keluarga Ramadhan.
  - Pelacakan Nafkah keluarga dan Sedekah/Wakaf.
  - Deteksi dan peringatan instrumen hutang berbunga (Anti-Riba Flag).
  - Koleksi offline 365 Hadits Harian etika muamalah & rezeki (teks Arab RTL + terjemahan Indonesia).
- **Literasi Finansial & Celengan Anak**:
  - Uang saku terjadwal, misi kebaikan/hemat dengan persetujuan orang tua, lencana penghargaan, dan simulasi pinjaman internal.
- **Skor Kesehatan Finansial (Health Score)**:
  - Indeks 0–100 mengukur rasio likuiditas, dana darurat, dan beban hutang dengan rekomendasi prioritas.
- **Local AI Financial Coach**:
  - Konsultasi perencanaan finansial berbasis AI dengan scrub privasi otomatis (hanya mengirim ringkasan teranonimkan) dan guardrail anti-halusinasi syariah.
- **CADS v1.0 UI Standard**:
  - Dark Obsidian shell, Emerald Green accent (`#10b981`), SvelteKit native routing.

---

## 🏗️ Arsitektur Teknologi

- **Backend / Core**: Rust 2024 (`cacash-core`, `cacash-app`), Tauri v2.
- **Frontend**: Svelte 5 (Runes), Tailwind CSS v4, CADS v1.0 Tokens.
- **Database**: SQLite lokal / SQLCipher dengan schema sync-ready UUIDv7.

---

## 🚀 Panduan Menjalankan

### Development
```bash
cd ~/Product/cacash
cargo tauri dev
```

### Build Rilis
```bash
cargo tauri build
```
