# Server Backend

Folder ini berisi server Express sederhana yang menyajikan statistik cache untuk kontrak token GOAT dan MEAT. Server membaca data on-chain melalui penyedia RPC dan menyediakan endpoint JSON yang digunakan frontend.

## Persiapan

1. Instal dependensi di root repositori:
   ```bash
   npm install
   ```
   Jalankan perintah di atas sebelum mengeksekusi Hardhat test apa pun.
2. Kompilasi kontrak untuk menghasilkan file ABI yang digunakan server:
   ```bash
   npx hardhat compile
   ```
   Salin ABI JSON yang diperbarui dari `artifacts/contracts/` ke `backend/abi/` ketika Anda memodifikasi kontrak.
3. Salin template environment dan atur variabel yang diperlukan:
   ```bash
   cp backend/.env.example backend/.env
   ```
   (Jika Anda berada di dalam direktori `backend/`, jalankan `cp .env.example .env`.)
   Atur `RPC_URL`, `GOAT_ADDRESS`, `MEAT_ADDRESS` dan `PORT` di `backend/.env`.

## Menjalankan

Jalankan server dengan:

```bash
npm run start:server
```

Endpoint tambahan disediakan untuk mengakses data LOD per komoditas:

```bash
curl http://localhost:3001/api/LOD/KAMBING
```

Untuk gambaran lengkap arsitektur dan interaksi kontrak, lihat [README](../README.md) utama.
