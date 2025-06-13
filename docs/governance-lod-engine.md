# Revolusi Moneter 5.0 → LOD Engine Governance Pipeline v1.0

Panduan ini menjelaskan alur yang harus dijalankan tim governance ketika memperbarui data LOD dan CommodityRepresentation pada kontrak `RateHandler`.

## 📌 Overview

Revolusi Moneter 5.0 menggunakan **Life Output Density (LOD)** sebagai penggerak utama paritas agar semua swap komoditas berbasis **nilai hidup yang berkelanjutan** → bukan ilusi fiat.

Setiap komoditas direpresentasikan sebagai:

- **NFT** → Aset fisik
- **Token virtual** → Representasi keseluruhan aset
- **Token produk** → Produk olahan (subtype MEAT, JMC, dll)

LOD per hari untuk tiap layer disimpan on-chain.
Semua nilai tersebut kini hanya dimuat melalui fungsi `setCommodityRepresentation` pada `RateHandler`.
Tidak ada fallback atau referensi ke `SwapConfig` maupun mapping lama lainnya.
Panggilan fungsi tersebut memunculkan event `CommodityRepresentationUpdated(commodityId)` untuk menandai pembaruan data di blockchain.

Versi CosmWasm dari `RateHandler`, `BarterEngine`, dan `RedeemEngine` kini menerapkan pembatasan subtype serta perhitungan LOD yang sama persis dengan implementasi Solidity.

Lihat diagram *Canonical Reasoning Path* pada [architecture.md](../architecture.md#token-layer-separation--lod-engine-enforcement) bagian 4 untuk pemahaman menyeluruh mengenai pemisahan layer dan aturan swap.

---

## 🛠️ Governance Workflow

### 1⃣ Perbarui `lod_data_base.json`

File: `lod_data_base.json`

- Tambahkan atau ubah entri komoditas.
- **Jangan mengedit `compute_lod.py`;** semua perubahan data dilakukan di file ini.
- Setiap komoditas menyimpan parameter berikut:
  - `protein_g_per_kg`
  - `fat_g_per_kg`
  - `micronutrient_index`
  - `yield_per_cycle_kg`
  - `cycle_time_days`
  - `market_price_usd_per_kg`
  - `bias_factor`

### 2⃣ Jalankan compute_lod.py

```bash
python3 compute_lod.py
```

Output berupa `lod_data.json`.

### 3⃣ Tinjau lod_data.json

Pastikan setiap komoditas memiliki:

- `lod_per_day_nft` benar.
- `lod_per_day_virtual` benar.
- `lod_per_day_product` benar.
- Seluruh parameter lengkap.

### 4⃣ Push CommodityRepresentation on-chain

Gunakan skrip Hardhat `updateCommodityRepresentation.js`:

```bash
npx hardhat run scripts/updateCommodityRepresentation.js --network yourNetwork
```

Jalankan untuk tiap komoditas dengan data dari `lod_data.json`, meliputi:

- Alamat NFT
- Alamat token virtual
- Alamat token produk
- Subtype produk (bila ada) - gunakan `ethers.encodeBytes32String("GOATMEAT")` untuk nilainya
- Seluruh parameter transparan
- Nilai LOD per layer

### 5⃣ Jalankan Unit Test QA

```bash
npx hardhat test test/RateHandler.test.js
```

Semua test harus lulus:

- `getLODPerDay` bekerja untuk tiap layer.
  Fungsi versi lama dengan satu argumen dipertahankan hanya untuk audit.
- `computeBarterRate` benar sesuai rasio LOD (hanya `PRODUCT↔PRODUCT`, versi singkat dihapus).
- Layer tidak valid harus revert.

### 6⃣ Commit `lod_data.json`

Versikan file LOD:

```
LOD v1.0 → lod_data_v1_20250609.json
LOD v1.1 → lod_data_v1_1_202507XX.json
...
```

---

## 🚀 Prinsip Utama

✅ LOD Engine = sumber kebenaran paritas.
✅ Governance mengontrol pembaruan parameter secara transparan.
✅ Riwayat LOD harus versi.
✅ Unit Test **WAJIB** lulus sebelum mengirim pembaruan on-chain.
✅ `RateHandler` hanya mengambil data melalui `setCommodityRepresentation` tanpa fallback ke `SwapConfig`.

---

## Checklist Governance

- [ ] Update `lod_data_base.json` lalu jalankan `compute_lod.py` untuk menghasilkan `lod_data.json`.
- [ ] Review dan approve `lod_data.json` secara internal.
- [ ] Push `CommodityRepresentation` on-chain.
- [ ] Jalankan seluruh unit test → harus lulus.
- [ ] Commit `lod_data.json` dengan versi terbaru.
- [ ] Umumkan pembaruan LOD secara publik.

---

## Catatan Akhir

Nilai harus tetap hidup. LOD Engine adalah logika yang menjaga keberlanjutan sistem. Governance bertanggung jawab menjaga paritas yang adil dan dapat diaudit di semua komoditas.

