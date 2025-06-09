# Level of Decay (LOD)

`LOD` menentukan penurunan nilai barter komoditas setiap hari. File `lod_data.json` di root menyimpan master data yang dihasilkan oleh skrip `compute_lod.py`. Nilai pada berkas ini dapat diperbarui maupun dimuat on-chain melalui fungsi `setCommodityRepresentation` pada `RateHandler`.

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

Pemilik kontrak dapat memanggil `setCommodityRepresentation(bytes32 commodityId, CommodityRepresentation data)` untuk mendaftarkan komoditas beserta representasinya (NFT, token virtual, dan token produk). Struktur ini juga menyimpan `lodPerDay` untuk setiap layer.

LOD per layer dapat dibaca melalui `getLODPerDay(bytes32 commodityId, string layer)` dimana `layer` adalah `"NFT"`, `"VIRTUAL"`, atau `"PRODUCT"`.

Fungsi `computeBarterRate(fromCommodity, fromLayer, toCommodity, toLayer)` menghitung rasio barter berbasis LOD antar dua representasi.

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
