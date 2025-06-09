# Siklus Hidup Token GOAT & MEAT

Kedua token membentuk loop tertutup yang memungkinkan nilai masuk melalui MEAT dan diberikan imbalan lewat staking GOAT.

- Setiap **GoatNFT** mencatat *ras*, *NFC tag*, *tahun lahir*, dan *berat* terkini.
- Pemilik bebas memindahkan NFT dan memperbarui berat seiring pertumbuhan hewan.
- Sebelum kambing disembelih, token dapat **dibakar** dengan berat terbaru yang mencetak `GOATMEAT` melalui `GoatNFTBurnHook`.
- GOAT diperoleh dengan mengunci NFT pada `GoatNFTWrapper` dan digunakan untuk staking.
- MEAT pada akhirnya dibakar menggunakan `redeemForMeat` untuk menebus daging fisik.

1. **Mencetak MEAT**
   * Pengguna mengirim mata uang native ke kontrak MEAT. Fungsi `receive()` mencetak MEAT ke pengirim berdasarkan `DepositRate`, dihitung per 1000 unit (default `100`, artinya 100 MEAT per 1000 native).
   * Kontrak memancarkan `MintedWithNative(user, nativeReceived, meatMinted)` yang mencatat siapa mencetak dan berapa jumlah token native diterima.
2. **GoatNFT**
   * [GoatNFT](contracts/GoatNFT.sol) mewakili kambing hidup dan menyimpan beratnya di `goatValue`.
   * Pemilik dapat memperbarui berat kapan saja (event `WeightUpdated`). Berat memakai satu desimal (`WEIGHT_DECIMALS = 1`) sehingga `425` berarti **42.5 kg**.
   * Sebelum membakar NFT, berat harus diperbarui dalam tujuh hari terakhir. Fungsi `burn` memicu `GoatNFTBurnHook` yang mencetak `GOATMEAT`.
3. **Staking GOAT**
   * GOAT diperoleh dengan membungkus GoatNFT melalui `GoatNFTWrapper`.
   * Pemegang GOAT melakukan staking di `GOAT.sol` yang mencatat saldo dan timestamp. Reward terakumulasi secara linier berdasarkan `rewardRate` dan `rewardInterval`.
   * Memanggil `stake()` lagi akan mengatur ulang `lastStakedTime` dan membuang reward yang belum diambil. Klaim terlebih dahulu jika ingin menambah stake.
4. **Mengambil Reward**
   * Setelah `minClaimInterval`, staker bisa mengambil reward atau menggabungkannya kembali ke stake. Untuk keluar sepenuhnya panggil `unstake` agar pokok dan reward diterima.
5. **Barter Produk**
   * GOATMEAT dapat ditukar dengan token produk lain menggunakan `RateHandler`.
6. **Menebus MEAT**
   * Pemegang membakar MEAT mereka melalui `redeemForMeat(amount)` yang memicu `MeatRedeemed` untuk pemrosesan off-chain. Tiap token yang ditebus mewakili **satu kilogram daging** dari mitra distribusi kami. Diagram singkat jalur burn dan redemption dapat dilihat pada bagian [Burn & Redeem Flow](README.md#burn--redeem-flow) di README.

Siklus ini memastikan setiap tahap partisipasi didukung oleh fungsi kontrak yang eksplisit dan alur token yang transparan.
