# GOAT and MEAT Token Contracts

This repository contains two ERC20 tokens:

- **GOAT** (Guardian of Agricultural Trade) supports staking and compounding rewards. The designated MEAT contract may mint new GOAT tokens while holders can stake their balance to earn a high annualised reward.
- **MEAT** (Market-Enabled Agricultural Token) lets users mint tokens with native currency and swap to and from GOAT, acting as the on‑ramp to the ecosystem.

## Cara Kerja Token

Berikut gambaran umum alur penggunaan kedua token:

1. **Mint MEAT** – Kirim native token (misalnya ETH/BNB) langsung ke kontrak `MEAT`. Kontrak akan mencetak MEAT sesuai `DepositRate` dan mengirimkannya ke pengirim.
2. **Swap MEAT ⇄ GOAT** – Fitur swap aktif jika `swapEnabled` bernilai `true`. Rasio konversi ditentukan oleh konstanta `SwapRate` (default `85`). Fungsi `swapMEATForGOAT` menukar MEAT yang dimiliki pengguna menjadi GOAT, sedangkan `swapGOATForMEAT` melakukan sebaliknya.
3. **Stake GOAT** – Pemegang GOAT dapat memanggil `stake(amount)` pada kontrak GOAT untuk mulai memperoleh reward. Besarnya reward dihitung linier berdasarkan `rewardRate` dengan periode akrual `rewardInterval`.
4. **Claim atau Compound** – Setelah melewati `minClaimInterval`, pengguna dapat mencairkan reward melalui `claimReward` atau melakukan `compoundReward` agar hasilnya otomatis ditambahkan ke saldo staking.

## Deployment

1. Install [Hardhat](https://hardhat.org/) and initialise a project:
   ```bash
   npm install --save-dev hardhat
   npx hardhat init
   ```
2. Install the OpenZeppelin contracts used by GOAT:
   ```bash
   npm install @openzeppelin/contracts
   ```
3. Copy `contracts/GOAT.sol` into your project's `contracts` folder and compile:
   ```bash
   npx hardhat compile
   ```
4. Deploy the contracts with your preferred Hardhat network configuration. A simple script might look like:
   ```javascript
   const GOAT = await ethers.getContractFactory('GOAT');
   const goat = await GOAT.deploy(ethers.ZeroAddress);
   await goat.deployed();

   const MEAT = await ethers.getContractFactory('MEAT');
   const meat = await MEAT.deploy(goat.address);
   await meat.deployed();

   await goat.setMEATAddress(meat.address);

   console.log('GOAT deployed to:', goat.address);
   console.log('MEAT deployed to:', meat.address);
   ```
   Run using `npx hardhat run scripts/deploy.js --network <network>` and specify your desired Hardhat network with the `--network` option.

## Contoh Konfigurasi Hardhat

Tambahkan pengaturan jaringan pada `hardhat.config.js` agar skrip `deploy.js` dapat dijalankan ke testnet. Misalnya untuk jaringan Sepolia:

```javascript
require("@nomicfoundation/hardhat-toolbox");

module.exports = {
  solidity: "0.8.29",
  networks: {
    sepolia: {
      url: "https://rpc.sepolia.org",
      accounts: ["0xYOUR_PRIVATE_KEY"]
    }
  }
};
```

Jalankan perintah berikut untuk melakukan deploy:

```bash
npx hardhat run scripts/deploy.js --network sepolia
```

## Parameter Penting

- **SwapRate** – konstanta di kontrak MEAT yang menentukan rasio penukaran antar
  kedua token. Nilai default `85` berarti 1 GOAT setara dengan 85 MEAT.
- **rewardRate** – tingkat imbal hasil tahunan di kontrak GOAT (dalam skala
  `1e18`). Nilai ini dikombinasikan dengan `rewardInterval` untuk menghitung
  reward harian pengguna.
- **minClaimInterval** – interval minimum pengguna dapat mengklaim atau
  meng-unstake dengan reward. Default-nya `7 days`.

## Running Tests

Hardhat tests for both contracts live in the `test/` directory. Run them with:

```bash
npx hardhat test
```

## 🧱 Struktur Kontrak

Struktur dan hubungan antar kontrak:
- `GOAT` (`contracts/GOAT.sol`) mewarisi `ERC20` OpenZeppelin dan menambahkan
  fungsi staking, klaim, kompaun, serta konfigurasi reward. Hanya kontrak `MEAT`
  yang dapat mencetak GOAT melalui `mintTo`.
- `MEAT` (`contracts/MEAT.sol`) adalah token `ERC20` yang menerima native token
  untuk mint, memungkinkan penukaran MEAT ↔ GOAT, dan mengontrol fitur swap.
- `IGOAT` (`contracts/interfaces/IGOAT.sol`) mendefinisikan fungsi `mintTo`
  untuk dipanggil MEAT saat membutuhkan GOAT baru.
- `FailingGOAT` (`contracts/mocks/FailingGOAT.sol`) digunakan pada unit test
  guna mensimulasikan kegagalan `transfer`.

Alur panggilan eksternal–internal secara ringkas:
1. `swapMEATForGOAT` memanggil `transferFrom` MEAT dan `mintTo` GOAT jika saldo
   tidak mencukupi.
2. `stake` mentransfer GOAT ke kontrak lalu mencatat waktu. Perhitungan reward
   dilakukan fungsi internal `calculateReward`.
3. `claimReward`, `compoundReward`, dan `unstake` sama-sama membaca reward dari
   `calculateReward` sebelum mentransfer atau mencetak token ke pengguna.

## 🔁 Flow Logic

Simulasi tahapan staking hingga unstake:

1. Pengguna memperoleh GOAT lalu memanggil `stake(amount)`.
2. Setelah `minClaimInterval` terlewati, pengguna dapat:
   - `claimReward` untuk mengambil hasil tanpa menarik pokok;
   - `compoundReward` agar hasil otomatis ditambahkan ke saldo staking.
3. Ketika keluar, panggil `unstake` sehingga pokok dan reward terkirim dan data
   staking dihapus.

Reward dihitung berbasis waktu (detik) sehingga tidak bergantung pada jumlah
blok. Perubahan state utama terjadi pada `stakingBalance`, `lastStakedTime`, dan
saldo token pengguna.

## 🧠 Perubahan Terakhir

Commit *Simplify MEAT swap allowance logic (#12)* merombak mekanisme swap:

- Penghapusan pemeriksaan allowance manual di fungsi-fungsi swap MEAT.
- Mengandalkan revert bawaan `transferFrom`/`transfer` bila allowance kurang.
- Mock `FailingGOAT` diperbarui agar dapat mensimulasikan kegagalan transfer.
- Test `meat.test.js` menyesuaikan dengan perilaku revert baru.

Perubahan ini menyederhanakan kode dan memastikan kegagalan transfer terdeteksi
otomatis.

## 🧪 Testing

Cakupan unit test meliputi:

- Deployment GOAT dan MEAT beserta kepemilikan serta suplai awal.
- Proses staking, klaim, unstake, dan swap (`stakingSwap.test.js`).
- Pengujian interval klaim (`claim.test.js`).
- Simulasi kegagalan transfer pada swap (`meat.test.js`).

Assertion penting mengecek perubahan saldo, event tidak ter-revert, dan update
waktu klaim. Belum tersedia tes batas untuk jumlah ekstrem atau konsumsi gas.

## 💬 Filosofi & Manifestasi

Kontrak dirancang menjaga nilai GOAT dan MEAT dengan imbal hasil tinggi namun
terkontrol. Reward dihitung per detik (setara harian) sehingga tidak terikat
kecepatan blok dan cocok lintas jaringan.

Pemanggilan `claimReward` tidak otomatis menambah stake agar pengguna bebas
menarik hasil tanpa memperpanjang penguncian. Fleksibilitas ini diharapkan
mencegah inflasi berlebih sekaligus memberi kontrol penuh pada komunitas.

---

Bukan sekadar dokumen, README ini menjadi "suara sistem" yang terus diperbarui
dan dipertanggungjawabkan.
