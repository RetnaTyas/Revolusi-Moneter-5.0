# Arsitektur Sistem

Proyek ini berpusat pada dua kontrak ERC20—**GOAT** dan **MEAT**—yang dideploy menggunakan Hardhat. Skrip backend menangani proses deployment serta konfigurasi kontrak, sementara frontend berinteraksi melalui metode yang disediakan.

## Lapisan On-Chain

* **GOAT (`contracts/GOAT.sol`)** – menyediakan staking dengan reward berbasis waktu, memungkinkan kontrak MEAT mencetak suplai baru, dan menawarkan kontrol pemilik seperti pengaturan reward serta alamat MEAT.
* **MEAT (`contracts/MEAT.sol`)** – mencetak token saat menerima mata uang native (menggunakan `DepositRate` per 1000 unit), mengendalikan deposit rate, serta dapat menarik saldo native yang terkumpul.
* **GoatNFT (`contracts/GoatNFT.sol`)** – menyimpan detail kambing on-chain dalam `goatMetadata` sebagai `GoatData` (`nfcId`, `breed`, `birthYear`, `weight`, `mintedAt`). Setiap `nfcId` dipetakan ke token ID sehingga duplikasi ditolak saat mint. Pemilik dapat memanggil `updateWeight` untuk memperbarui berat (menerbitkan `WeightUpdated`). `burn` mensyaratkan pembaruan berat dalam 7 hari terakhir, secara otomatis mencetak GOAT dan memicu `GoatBurned` sambil menghapus pemetaan NFC.
* **GoatNFTBurnHook (`contracts/GoatNFTBurnHook.sol`)** – dipanggil oleh `GoatNFT` saat token dibakar untuk mencetak `GOATMEAT` berdasarkan bobot.
* **GoatNFTWrapper (`contracts/GoatNFTWrapper.sol`)** – kontrak pembungkus yang mengunci GoatNFT dan mencetak GOAT setara. NFT dapat diambil kembali setelah jumlah GOAT yang sama dibakar.
* **RateHandler (`contracts/RateHandler.sol`)** – menyimpan `dynamicRate` yang dipakai MEAT dan GoatNFT. Pemilik dapat menetapkan nilai baru melalui `updateRate` (memicu event `RateUpdated`) atau menonaktifkan rate dengan `invalidateRate` sehingga kontrak kembali menggunakan `SWAP_RATE` dari `SwapConfig` serta memancarkan `RateInvalidated`. Kepemilikan dapat dialihkan lewat `transferOwnership`.
* **Interface dan mock** – `IGOAT` mendefinisikan hook pencetakan yang digunakan MEAT, sedangkan `FailingGOAT` membantu pengujian dengan mensimulasikan kegagalan transfer.

## Backend

Lingkungan Hardhat menjalankan proses deployment (`scripts/deploy.js`) dan pengujian di folder `test/`. Layanan backend dapat mengimpor kontrak terkompilasi dan memakai `ethers` untuk memanggil fungsi seperti `stake`.

## Frontend

Frontend berkomunikasi dengan kontrak yang telah dideploy menggunakan `ethers.js`, menyediakan alur untuk mencetak MEAT serta melakukan staking dan klaim reward. Frontend mengandalkan alamat deployment dan artefak ABI yang dihasilkan Hardhat.

Gabungan komponen ini membentuk stack lengkap di mana pengguna berinteraksi melalui frontend, yang selanjutnya terhubung ke blockchain lewat kontrak GOAT dan MEAT.
