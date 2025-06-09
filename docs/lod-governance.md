# Level of Decay (LOD)

`LOD` menentukan penurunan nilai barter komoditas setiap hari. File `lod_data.json` di root menyimpan master data awal yang dapat diperbarui melalui fungsi `setCommodityLOD` pada `RateHandler`.

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

Pemilik kontrak dapat memanggil `setCommodityLOD(bytes32 commodity, uint256 lodPerDay)` untuk memperbarui nilai tersebut secara on-chain.

## Formula

Barter rate dihitung menggunakan dynamic rate terbaru yang dikelola `RateHandler` kemudian ditambah kenaikan sesuai LOD per hari sejak pembaruan rate:

```
finalRate = currentRate + (lodPerDay * daysSinceUpdate)
```

`daysSinceUpdate` dihitung berdasarkan `lastUpdateTimestamp` pada kontrak. Jika LOD belum diset, nilai final sama dengan `currentRate`.
