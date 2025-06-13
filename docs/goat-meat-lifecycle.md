# Siklus Hidup Token GOAT & MEAT

Kedua token membentuk loop tertutup yang memungkinkan nilai masuk melalui MEAT dan diberikan imbalan lewat staking GOAT.

- Setiap **GoatNFT** mencatat *ras*, *NFC tag*, *tahun lahir*, dan *berat* terkini.
- Pemilik bebas memindahkan NFT dan memperbarui berat seiring pertumbuhan hewan.
- Sebelum kambing disembelih, token dapat **dibakar** dengan berat terbaru yang mencetak `GOATMEAT` sebesar 60% dari berat hidup melalui `GoatNFTBurnHook`.
- Alamat `GoatNFT` dan `MEAT` ditetapkan di konstruktor. Fungsi `setNFTAddress` maupun `setMEATAddress` hanya dipakai pemilik bila perlu mengganti kontrak.
- GOAT diperoleh dengan mengunci NFT pada `GoatNFTWrapper` dan digunakan untuk staking.
- MEAT pada akhirnya dibakar melalui `RedeemEngine.redeem` untuk menebus daging fisik.

1. **Mencetak MEAT**
   * Token hanya dicetak melalui `mintSubtype` oleh kontrak terotorisasi seperti `GoatNFTBurnHook`.
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
   * GOATMEAT dapat ditukar dengan token produk lain menggunakan `RateHandler` yang diakses oleh `BarterEngine` (port CosmWasm belum tersedia).
   * Sebelum barter, pengguna harus memberikan *approval* kepada `BarterEngine` untuk jumlah subtype yang akan dibakar.
   * Subtype seperti `GOATMEAT` harus di-encode ke `bytes32` misal `ethers.encodeBytes32String("GOATMEAT")`.
   * MEAT selalu mengharapkan parameter subtype berupa `bytes32`; ubah ID numerik ke string sebelum di-encode. contoh: `bytes32 subtypeFromId = ethers.encodeBytes32String("99");`
6. **Menebus MEAT**
   * Panggil `RedeemEngine.redeem(subtype, amount)` untuk membakar MEAT setelah verifikasi lineage.
     Berat daging dihitung memakai `RedeemConfig.gramsPerTokenUnit` dan hasilnya
     dicatat dalam event `RedeemExecuted`.
     Konfigurasi default menyetarakan **1 MEAT (1e18 unit) dengan 1 kg**, tetapi
     nilainya dapat diubah. Lihat diagram [Burn & Redeem Flow](README.md#burn--redeem-flow).
   * Pengguna harus men-*approve* `RedeemEngine` sebelum memanggil `redeem`.

Siklus ini memastikan setiap tahap partisipasi didukung oleh fungsi kontrak yang eksplisit dan alur token yang transparan.

## Architecture Diagram

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

Diagram di atas menunjukkan pemisahan lapisan token dan jalur pertukaran yang diizinkan.

## Alur SapiNFT → BEEFMEAT

Sapi juga tercatat menggunakan [SapiNFT](contracts/SapiNFT.sol). NFT ini dapat
dibungkus lewat [SapiNFTWrapper.sol](contracts/SapiNFTWrapper.sol) untuk
mendapatkan GOAT sesuai beratnya. Ketika sapi disembelih, fungsi `burn` pada
`SapiNFT` memicu
[SapiNFTBurnHook.sol](contracts/SapiNFTBurnHook.sol) yang otomatis mencetak
`BEEFMEAT`.

Kontrak hook memiliki konstanta `SLAUGHTER_YIELD_BPS` sebesar `6500`, artinya
**65 %** dari berat hidup dicetak sebagai BEEFMEAT. Sebelum burn, berat harus
dipastikan terkini sebagaimana alur kambing. Token BEEFMEAT selanjutnya dapat
dibarter atau ditebus melalui `RedeemEngine`.
