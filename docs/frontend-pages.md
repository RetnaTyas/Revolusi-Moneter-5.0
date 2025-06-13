# Halaman Frontend

Dokumen ini merangkum halaman-halaman utama yang akan ada pada UI penuh dan hubungannya dengan fungsi kontrak. Halaman tersebut hanya dimock di template `frontend/`, namun alurnya mencerminkan perilaku yang diuji pada test Solidity. Sebuah landing page statis juga tersedia di `frontend/static/` untuk keperluan demo GitHub Pages.

## `/dashboard`
*Gambaran*: halaman awal menampilkan statistik umum seperti total suplai dan total staking.
- **Data**: diambil dari endpoint backend `/stats` yang memanggil `totalSupply()` pada kedua token serta `balanceOf()` pada alamat kontrak GOAT.
- **Input Pengguna**: tidak ada; hanya tampilan.
- **Panggilan Kontrak**: tidak ada dari sisi pengguna; statistik berasal dari backend.

## `/goat`
*Gambaran*: menampilkan saldo GOAT pengguna.
- **Input Pengguna**: tidak ada aksi khusus.
- **Panggilan Kontrak**: hanya pembacaan saldo dari `GOAT.sol`.

## `/stake`
*Gambaran*: antarmuka staking GOAT dan melihat reward yang menunggu.
- **Input Pengguna**: jumlah GOAT yang akan di-stake.
- **Panggilan Kontrak**: `stake(amount)` pada `GOAT.sol`.

## `/rewards`
*Gambaran*: klaim atau compound reward staking serta melakukan unstake.
- **Input Pengguna**:
  - Pilihan antara `claim`, `compound` atau `unstake`.
- **Panggilan Kontrak**:
  - `claimReward()` – menarik reward tanpa menyentuh pokok.
  - `compoundReward()` – menanamkan kembali reward ke `stakingBalance`.
  - `unstake()` – menarik pokok beserta reward setelah `minClaimInterval`.

## `/burn`
*Gambaran*: membakar `GoatNFT` untuk mencetak `GOATMEAT` berdasarkan berat ternak.
- **Input Pengguna**: `tokenId` NFT yang akan dibakar.
- **Panggilan Kontrak**: `burn(tokenId)` pada `GoatNFT` yang memicu `GoatNFTBurnHook`.

Halaman-halaman ini mengikuti alur yang diuji di `test/` seperti `wrapper.test.js`, `goatMeatHook.test.js`, dan `nftBurnMint.test.js`, memastikan aksi UI sesuai dengan perilaku on-chain.
