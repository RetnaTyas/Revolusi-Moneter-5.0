# Moneter 5.0 — Agents Instructions & Permission Flow

---

## 🎯 Purpose

Dokumen ini mendefinisikan role-based agents beserta permission flow yang berlaku pada kontrak Moneter 5.0:

- Mencegah abuse.
- Memastikan **Value Must Remain Alive** sehingga traceability terjaga.
- Menjadi acuan untuk test scripts, deployment pipeline, external integration, dan proses audit.

---

## 🗺️ Agents Table

| Agent Name | Description | Permissions |
|------------|-------------|-------------|
| **Governance Agent** | Owner of contracts → LOD config, Redeem config, emergency withdraw | `setMinter()`, `setBurner()`, `transferOwnership()`, `emergencyWithdrawMEATSubtype()` |
| **Wrapper Agent** | `GoatNFTWrapper` / `SapiNFTWrapper` → mint GOAT token | Calls `GOAT.mint()` |
| **NFTBurnHook Agent** | `GoatNFTBurnHook` / `SapiNFTBurnHook` → mint MEAT Subtype | Calls `MEAT.mintSubtype()` |
| **BarterEngine Agent** | `BarterContract` → Product ↔ Product barter flow | Calls `MEAT.burnSubtype()`, `MEAT.mintSubtype()` |
| **RedeemEngine Agent** | `RedeemEngine` → burn MEAT Subtype for physical redeem | Calls `MEAT.burnSubtype()` |
| **User Agent** | End user → wallet interacting with system | Stake GOAT, transfer GOAT/MEAT, redeem physical goods |
| **Audit Agent** | Off-chain audit systems / backend | Calls `balanceOfSubtypeWithLineage()`, `getLODPerDay()`, `computeBarterRate()` |

---

## 🔐 Flow Guarantees

- `GOAT.mint()` **hanya** dapat dipanggil oleh:
  - `GoatNFTWrapper` Agent
  - `SapiNFTWrapper` Agent
- `MEAT.mintSubtype()` **hanya** dapat dipanggil oleh:
  - `GoatNFTBurnHook` Agent (→ `GOATMEAT`)
  - `SapiNFTBurnHook` Agent (→ `BEEFMEAT`)
  - Future hooks untuk subtype lain jika ada
- `MEAT.burnSubtype()` dapat dipanggil oleh:
  - `BarterEngine` Agent (saat barter)
  - `RedeemEngine` Agent (saat redeem)
- `MEAT.mintSubtype()` saat barter **hanya** boleh dilakukan oleh `BarterEngine` Agent.
- `RedeemEngine` **harus**:
  - Memverifikasi lineage subtype menggunakan `balanceOfSubtypeWithLineage()`.
  - Membakar hanya subtype dan lineage yang valid.
- `BarterEngine` **harus**:
  - Memverifikasi lineage subtype menggunakan `balanceOfSubtypeWithLineage()`.
  - Menegakkan swap **PRODUCT ↔ PRODUCT** dengan `computeBarterRate()`.

---

## 🛡️ Security Rules

- Tidak ada agent yang boleh mencetak token MEAT default (`_mint`) secara langsung—hanya `mintSubtype()` yang diizinkan.
- Hanya **Governance Agent** yang dapat memberikan izin melalui `setMinter()` / `setBurner()`.
- `emergencyWithdrawMEATSubtype()` hanya boleh dipicu oleh **Governance Agent**.
- Semua agent harus beroperasi dalam peran yang ditentukan; penyalahgunaan agent membatalkan integritas nilai.

---

## 🚀 Agent Integration Guide

### For Test Automation

Tugaskan wallet uji sebagai berikut:

- `WalletA` → GovernanceAgent
- `WalletB` → WrapperAgent
- `WalletC` → NFTBurnHookAgent
- `WalletD` → BarterEngineAgent
- `WalletE` → RedeemEngineAgent
- `WalletU` → UserAgent (wallet test user)

### For Codex Instructions

- Hanya `WrapperAgent` yang boleh memanggil `GOAT.mint()`.
- Hanya `BurnHookAgent` yang boleh memanggil `MEAT.mintSubtype()`.
- Hanya `BarterEngineAgent` yang boleh melakukan barter subtype.
- Hanya `RedeemEngineAgent` yang boleh melakukan redeem.
- `GovernanceAgent` mengelola konfigurasi dan emergency withdraw.

---

## Summary: Why This Architecture Enforces Value Integrity

- ✅ Menjaga **Value Must Remain Alive**
- ✅ Mencegah penyalahgunaan token MEAT
- ✅ Menjamin traceability lineage subtype
- ✅ Memblokir jalur kritis mint/burn di balik agent terotorisasi
- ✅ Menyelaraskan alur test dan pipeline governance

