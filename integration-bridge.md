# Jembatan Integrasi

Integrasi antara smart contract dan aplikasi eksternal ditangani melalui backend Node/Hardhat yang ringan.

1. **Skrip Deploy** – `scripts/deploy.js` mengompilasi dan mendepoy GOAT serta MEAT tanpa perlu memanggil `setMEATAddress` ataupun menautkan alamat apa pun.
2. **Artefak ABI** – Hardhat menghasilkan ABI dan bytecode yang dapat diimpor layanan backend atau langsung digunakan frontend untuk membangun instance `ethers.Contract`.
3. **Lapisan API** – Server Express opsional dapat menyediakan endpoint REST untuk memanggil fungsi kontrak seperti `stake`. Cara ini menjaga kunci privat di server sementara frontend menandatangani transaksi di sisi klien bila diperlukan.
4. **Pemantauan Event** – Kedua lapisan memantau event penting (`MintedWithNative`, `Staked`, `Unstaked`, `EmergencyUnstaked`, dll.) agar UI tetap sinkron.

## Backend Setup

Detail langkah konfigurasi backend dijelaskan pada [backend/README.md](backend/README.md).
Buat file `.env` di folder `backend` dan isi variabel berikut:

- `RPC_URL`
- `GOAT_ADDRESS`
- `MEAT_ADDRESS`
- `PORT`

Setelah environment siap, jalankan server dengan perintah:

```bash
npm run start:server
```

Jembatan ini memastikan koordinasi mulus antara logika on-chain dan antarmuka pengguna tanpa menduplikasi aturan bisnis.
