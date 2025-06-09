# Level of Decay (LOD)

`LOD` menentukan penurunan nilai barter komoditas setiap hari. File `lod_data.json` di root menyimpan master data awal yang dapat diperbarui melalui fungsi `setCommodityRepresentation` pada `RateHandler`.

```json
[
  {
    "commodity_id": "KAMBING",
    "lod_per_day_nft": 44.52,
    "lod_per_day_virtual": 44.52,
    "lod_per_day_product": 44.52
  },
  {
    "commodity_id": "ITIK",
    "lod_per_day_nft": 22.8,
    "lod_per_day_virtual": 22.8,
    "lod_per_day_product": 22.8
  }
  ...
]
```

Pemilik kontrak dapat memanggil `setCommodityRepresentation(bytes32 commodityId, CommodityRepresentation data)` untuk mendaftarkan komoditas beserta representasinya (NFT, token virtual, dan token produk). Struktur ini juga menyimpan `lodPerDay` untuk setiap layer.

LOD per layer dapat dibaca melalui `getLODPerDay(bytes32 commodityId, string layer)` dimana `layer` adalah `"NFT"`, `"VIRTUAL"`, atau `"PRODUCT"`.

Fungsi `computeBarterRate(fromCommodity, fromLayer, toCommodity, toLayer)` menghitung rasio barter berbasis LOD antar dua representasi.

## Formula

Rasio barter antar layer dihitung berdasarkan perbandingan LOD:

```
rate = (lodFrom * 1e18) / lodTo
```

Nilai dikalikan `1e18` agar presisi tetap terjaga pada operasi desimal.
