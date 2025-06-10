# Level of Decay (LOD)

`LOD` menentukan penurunan nilai barter komoditas setiap hari. Data dasar tersimpan pada `lod_data_base.json` dan diolah oleh skrip `compute_lod.py` menjadi `lod_data.json`. Nilai pada berkas keluaran tersebut dapat dimuat on-chain melalui fungsi `setCommodityRepresentation` pada `RateHandler`.

```json
[
  {
    "commodity_id": "KAMBING",
    "lod_per_day_nft": 44.38,
    "lod_per_day_virtual": 44.38,
    "lod_per_day_product": 44.38
  },
  {
    "commodity_id": "ITIK",
    "lod_per_day_nft": 17.34,
    "lod_per_day_virtual": 17.34,
    "lod_per_day_product": 17.34
  }
  ...
]
```

Pemilik kontrak dapat memanggil `setCommodityRepresentation(bytes32 commodityId, CommodityRepresentation data)` untuk mendaftarkan komoditas beserta representasinya (NFT, token virtual, dan token produk). Struktur `CommodityRepresentation` menyimpan:

- alamat NFT
- alamat token virtual
- alamat token produk
- subtype produk (bytes32) - contoh: `ethers.encodeBytes32String("GOATMEAT")`
- status aktif masing-masing layer
- nilai `lodPerDay` untuk setiap layer
- parameter transparan: `protein_g_per_kg`, `fat_g_per_kg`, `micronutrient_index_x1000`, `yield_per_cycle_kg`, `cycle_time_days`

Semua data dihasilkan pipeline `compute_lod.py` dan dirangkum dalam `lod_data.json`.

LOD per layer dapat dibaca melalui `getLODPerDay(bytes32 commodityId, string layer)` dimana `layer` adalah `"NFT"`, `"VIRTUAL"`, atau `"PRODUCT"`. Versi lama `getLODPerDay(bytes32)` dipertahankan hanya untuk audit governance.

Fungsi `computeBarterRate(fromCommodity, fromLayer, toCommodity, toLayer)` menghitung rasio barter berbasis LOD antar dua representasi. Sejak v1.1, kedua `layer` harus bernilai `"PRODUCT"` dan fungsi versi singkat telah dihapus.

## Formula

Rasio barter antar layer dihitung berdasarkan perbandingan LOD. Nilai LOD sendiri dihasilkan oleh skrip `compute_lod.py` menggunakan rumus:

```
life_support_density = (protein_g_per_kg * 4 + fat_g_per_kg * 9) * micronutrient_index
lod_per_day = life_support_density * (yield_per_cycle_kg / cycle_time_days) / 4 * bias_factor
```

Rasio barter antar layer kemudian dihitung dengan membandingkan LOD:

```
rate = (lodFrom * 1e18) / lodTo
```

Nilai dikalikan `1e18` agar presisi tetap terjaga pada operasi desimal.

## Memperbarui Data LOD

Untuk menyesuaikan LOD atau menambah komoditas baru:

1. Perbarui daftar komoditas di [lod_data_base.json](../lod_data_base.json).
   Tidak perlu mengubah skrip `compute_lod.py`.
2. Jalankan perintah berikut di root repositori untuk menghasilkan ulang `lod_data.json`:

   ```bash
   python3 compute_lod.py
   ```

File `lod_data.json` kemudian dapat dimuat ke on-chain melalui `setCommodityRepresentation`.

### Contoh Hardhat Script

Skrip `scripts/updateCommodityRepresentation.js` menyiapkan transaksi governance
untuk mendorong data dari `lod_data.json` ke `RateHandler`.

```bash
npx hardhat run scripts/updateCommodityRepresentation.js --network yourNetwork
```

Sesuaikan alamat kontrak dan parameter pada skrip dengan data hasil pipeline.

Untuk panduan end-to-end mengenai proses governance, lihat [governance-lod-engine.md](governance-lod-engine.md).
