# Level of Decay (LOD)

`LOD` menentukan penurunan nilai barter komoditas setiap hari. File `lod_data.json` di root menyimpan master data awal yang dapat diperbarui melalui fungsi `setCommodityLOD` pada `RateHandler`.

```json
{
  "GOAT_MEAT": { "lodPerDay": 5 },
  "RAW_MILK": { "lodPerDay": 10 },
  "WHEAT": { "lodPerDay": 1 }
}
```

Pemilik kontrak dapat memanggil `setCommodityLOD(bytes32 commodity, uint256 lodPerDay)` untuk memperbarui nilai tersebut secara on-chain.

## Formula

Barter rate dihitung menggunakan dynamic rate terbaru yang dikelola `RateHandler` kemudian ditambah kenaikan sesuai LOD per hari sejak pembaruan rate:

```
finalRate = currentRate + (lodPerDay * daysSinceUpdate)
```

`daysSinceUpdate` dihitung berdasarkan `lastUpdateTimestamp` pada kontrak. Jika LOD belum diset, nilai final sama dengan `currentRate`.
