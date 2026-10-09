# CACash Family Architecture

## 1. Overview
CACash adalah aplikasi manajemen keuangan keluarga dan kekayaan syariah berdaulat (*sovereign*) yang beroperasi secara *local-first*. Keamanan privasi multi-anggota keluarga dijamin melalui enkripsi lokal dan matriks visibilitas ketat.

## 2. Diagram Arsitektur

```
+-------------------------------------------------------------+
|                CADS v1.0 UI (Svelte 5 Runes)                |
|  - SvelteKit Native Routing:                                |
|    /dashboard     -> Ringkasan Kas & Arus Kas               |
|    /transactions  -> Buku Kas & Pencatatan Cepat            |
|    /accounts      -> Rekening, Dompet, Joint Shares         |
|    /budgets       -> Amplop Anggaran Bulanan                |
|    /goals         -> Target Tabungan & Ibadah (Haji/Qurban) |
|    /islamic       -> Zakat Mal/Fitrah & Anti-Riba Monitor   |
|    /kids          -> Celengan & Misi Literasi Anak          |
|    /investments   -> Portofolio Aset & Monitoring Hutang    |
|    /ai-coach      -> Asisten Finansial Berbasis Privasi     |
|    /settings      -> Profil, PIN, dan Konfigurasi Kunci     |
|  - Tokens CADS v1.0 (Dark Obsidian, Emerald #10b981)        |
+-------------------------------------------------------------+
                             |  IPC (Tauri Commands)
+-------------------------------------------------------------+
|                    Tauri v2 Application                     |
|  - Window Frame & Titlebar Kontrol                          |
|  - IPC Dispatcher (`cacash-app`)                            |
+-------------------------------------------------------------+
                             |
+-------------------------------------------------------------+
|                   cacash-core (Rust 2024)                   |
|  - Money Type Engine (Integer Minor Unit Math)              |
|  - Multi-Profile & PIN Authentication Matrix                |
|  - Data Visibility Layer (shared / summary / private)       |
|  - Islamic Engine (Zakat Mal 85g Emas, Fitrah, Haul)        |
|  - Offline Hadith Bank (365 Hadits RTL Arabic + ID)         |
|  - Kids Mission & Badges Engine                             |
|  - Financial Health Scoring Engine                          |
|  - Local SQLite Store (UUIDv7, Sync-Ready Logs)             |
+-------------------------------------------------------------+
```

## 3. Prinsip Inti
1. **Integer Money Math**: Tidak ada tipe floating-point (`f32`/`f64`) untuk nilai uang. Seluruh nominal disimpan dan dihitung dalam satuan integer terkecil (Rupiah murni).
2. **Access Matrix**: Profil anak dibatasi dari akses data finansial dewasa di level Rust core. Data private pasangan difilter secara ketat.
3. **Syariah Integrity**: Engine Zakat dihitung secara deterministik; AI tidak diizinkan mengarang hadits atau ayat secara halusinasi.
