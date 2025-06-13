# Arsitektur Sistem

Proyek ini berpusat pada dua kontrak ERC20—**GOAT** dan **MEAT**—yang dideploy menggunakan Hardhat. Skrip backend menangani proses deployment serta konfigurasi kontrak, sementara frontend berinteraksi melalui metode yang disediakan.

## Lapisan On-Chain

* **GOAT (`contracts/GOAT.sol`)** – menyediakan staking dengan reward berbasis waktu dan kontrol konfigurasi. Suplai GOAT dicetak melalui `GoatNFTWrapper`.
* **MEAT (`contracts/MEAT.sol`)** – token ERC20 yang dicetak melalui `mintSubtype()` oleh kontrak terotorisasi. Kontrak ini menyimpan saldo per subtype dan mendukung pembakaran.
* **GoatNFT (`contracts/GoatNFT.sol`)** – menyimpan detail kambing on-chain dalam `goatMetadata` sebagai `GoatData` (`nfcId`, `breed`, `birthYear`, `weight`, `mintedAt`). Setiap `nfcId` dipetakan ke token ID sehingga duplikasi ditolak saat mint. Pemilik dapat memanggil `updateWeight` untuk memperbarui berat (menerbitkan `WeightUpdated`). `burn` mensyaratkan pembaruan berat dalam 7 hari terakhir, memicu `GoatNFTBurnHook` dan `GoatBurned` sambil menghapus pemetaan NFC.
* **GoatNFTBurnHook (`contracts/GoatNFTBurnHook.sol`)** – dipanggil oleh `GoatNFT` saat token dibakar untuk mencetak `GOATMEAT` berdasarkan bobot.
* **GoatNFTWrapper (`contracts/GoatNFTWrapper.sol`)** – kontrak pembungkus yang mengunci GoatNFT dan mencetak GOAT setara. NFT dapat diambil kembali setelah jumlah GOAT yang sama dibakar.
* **RateHandler (`contracts/RateHandler.sol`)** – menghitung paritas barter PRODUCT↔PRODUCT berdasarkan nilai LOD yang disimpan untuk setiap komoditas. Data diregistrasi melalui `setCommodityRepresentation` dan dipakai `BarterEngine` saat swap.
* **BarterEngine (`contracts/BarterEngine.sol`)** – kontrak swap yang membakar subtype MEAT asal lalu mencetak subtype tujuan berdasarkan rasio yang dihitung `RateHandler`. Swap hanya berlaku untuk PRODUCT↔PRODUCT dan token GOAT tidak dapat dipertukarkan.
* **Interface dan mock** – `IGOAT` mendefinisikan fungsi `mintTo` untuk dipanggil `GoatNFTWrapper`, sedangkan `FailingGOAT` membantu pengujian dengan mensimulasikan kegagalan transfer.

## Backend

Lingkungan Hardhat menjalankan proses deployment (`scripts/deploy.js`) dan pengujian di folder `test/`. Layanan backend dapat mengimpor kontrak terkompilasi dan memakai `ethers` untuk memanggil fungsi seperti `stake`.

## Frontend

Frontend berkomunikasi dengan kontrak yang telah dideploy menggunakan `ethers.js`, menyediakan alur untuk mencetak MEAT serta melakukan staking dan klaim reward. Frontend mengandalkan alamat deployment dan artefak ABI yang dihasilkan Hardhat.

Gabungan komponen ini membentuk stack lengkap di mana pengguna berinteraksi melalui frontend, yang selanjutnya terhubung ke blockchain lewat kontrak GOAT dan MEAT.

## Token Layer Separation & LOD Engine Enforcement

```mermaid
graph TD
    subgraph LIVESTOCK_CAPITAL_LAYER ["🐐 LIVESTOCK CAPITAL LAYER"]
        GoatNFT[GoatNFT (ERC721)<br>Physical Goat Identity] -- wrapToGOAT --> GOAT[GOAT Token (ERC20)<br>Financial Layer Only]
        GOAT -- unwrapToNFT --> GoatNFT
        GOAT -- staking --> RewardPool[Reward Pool / Governance]
    end

    subgraph PRODUCT_LAYER ["🥩 PRODUCT LAYER"]
        GoatNFT -- burnForMeat --> GOATMEAT[MEAT.sol<br>subtype=GOATMEAT<br>Product Token]
        GOATMEAT -- barter/sell/deliver --> RealEconomy[Real Economy<br>(Barter / Swap / Deliver)]
    end

    subgraph LOD_MASTER ["📚 LOD MASTER"]
        LOD[LOD Engine<br>commodity_type=HEWAN] -->|LOD parity| GOATMEAT
    end

    subgraph FORBIDDEN_PATH ["🚫 FORBIDDEN PATH (Explicit Blocked)"]
        GOAT -.X.-> GOATMEAT
        GOATMEAT -.X.-> GOAT
        RateHandler -.X.-> CrossLayerSwap[Cross Layer Swap<br>FORBIDDEN]
    end
```

**🐐 LIVESTOCK CAPITAL LAYER**

GoatNFT → wrap → GOAT token → Financial token only → untuk staking/gov/ROI dan tidak digunakan di RateHandler.

GOAT token → unwrap → mengembalikan GoatNFT secara reversibel.

Staking GOAT → lapisan finansial → mengalir ke Reward Pool.

**🥩 PRODUCT LAYER**

GoatNFT dibakar → mencetak GOATMEAT sebagai hasil penyembelihan.

GOATMEAT diperdagangkan → barter, jual, atau dikirim ke ekonomi riil.

**📚 LOD MASTER**

LOD Engine mengawasi paritas → commodity_type=HEWAN → dipetakan ke subtype GOATMEAT.

**🚫 FORBIDDEN PATH**

Pertukaran langsung GOAT ↔ GOATMEAT dilarang. RateHandler hanya memperbolehkan swap PRODUCT↔PRODUCT agar nilai tetap hidup dan tidak bocor.
