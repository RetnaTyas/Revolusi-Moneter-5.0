# Deploy Kontrak CosmWasm

Proyek ini menyertakan implementasi CosmWasm untuk seluruh kontrak inti di direktori `wasm-contracts/`:

- `starter` – token GOAT dengan logika staking
- `meat` – token MEAT. Pencetakan dilakukan via pesan `mint_subtype` oleh kontrak terotorisasi dan penebusan menggunakan `redeem` pada `RedeemEngine`.
- `goatnft` – kontrak NFT sederhana tempat setiap token menyimpan nilai berat yang dapat ditebus menjadi GOAT
- `ratehandler` – utilitas kecil yang menyimpan rasio konversi terbaru dan memungkinkan pemilik memperbarui atau menonaktifkannya
- `goatnftwrapper` – membungkus GoatNFT untuk mencetak GOAT, NFT dikunci hingga pengguna melakukan `unwrap`
- `goatnftburnhook` – hook saat GoatNFT dibakar dan mencetak `GOATMEAT`
- `sapinft` – NFT sapi yang menyimpan berat dan dapat dibakar menjadi `BEEFMEAT`
- `sapinftwrapper` – membungkus SapiNFT untuk mencetak GOAT
- `sapinftburnhook` – hook saat SapiNFT dibakar dan mencetak `BEEFMEAT`
- `barterengine` – modul barter produk antar subtype menggunakan `RateHandler`

Paket-paket ini mencerminkan kontrak Solidity di `contracts/`. Sebagian besar fungsi memiliki pesan execute yang setara, namun terdapat beberapa perbedaan penting:

- **MEAT** tidak memiliki fungsi auto-mint. Token dicetak melalui pesan `mint_subtype` yang dipanggil kontrak lain.
- **starter** menerapkan staking GOAT dan penebusan NFT sama seperti `GOAT.sol`, hanya saja event menjadi atribut log.
- **goatnft** menyimpan metadata kambing dan berat mirip dengan `GoatNFT.sol` dengan sedikit perbedaan penamaan. Pemilik dapat memanggil `update_weight` untuk memperbarui berat. Pembakaran membutuhkan pembaruan terakhir dalam rentang `WEIGHT_UPDATE_VALIDITY` (7 hari).

## Membangun

Untuk penyiapan toolchain dan detail tambahan lihat [../wasm-contracts/README.md](../wasm-contracts/README.md).

Pasang target WASM jika belum:

```bash
rustup target add wasm32-unknown-unknown
```

Lalu jalankan skrip build:

```bash
./wasm-contracts/build.sh
```

Skrip ini mengompilasi seluruh paket dan menempatkan berkas `.wasm` di `artifacts/`. Skema JSON hasil `cargo schema` disimpan di direktori `schema/` masing-masing paket. Jika `cargo schema` belum tersedia, instal dengan `cargo install cargo-run-script` sebelum menjalankan skrip build.

## Unggah & Instansiasi

1. Unggah bytecode wasm (contoh untuk GOAT):
```bash
# contoh upload kontrak GOAT
wasmd tx wasm store artifacts/starter.wasm --from wallet \
 --gas-prices 0.025uatom --gas auto --gas-adjustment 1.3 \
  --chain-id testing-1 --node https://rpc.testnet.cosmos.network
```
Untuk mengunggah `ratehandler` cukup ganti nama berkas:
```bash
wasmd tx wasm store artifacts/ratehandler.wasm --from wallet \
  --gas-prices 0.025uatom --gas auto --gas-adjustment 1.3 \
  --chain-id testing-1 --node https://rpc.testnet.cosmos.network
```
Simpan `code_id` yang dihasilkan.
2. Instansiasi kontrak (contoh untuk GOAT):
```bash
wasmd tx wasm instantiate <code_id> '{"meat_contract":"cosmos1..."}' \
  --from wallet --label "goat" \
  --gas-prices 0.025uatom --gas auto --gas-adjustment 1.3 \
  --chain-id testing-1 --node https://rpc.testnet.cosmos.network
```
Instansiasi `meat`, `goatnft`, `sapinft`, dan `ratehandler` dengan perintah serupa. `ratehandler` digunakan oleh `BarterEngine` untuk menentukan rasio barter. Setelah itu deploy `goatnftwrapper`, `sapinftwrapper`, `goatnftburnhook`, `sapinftburnhook`, dan `barterengine` dengan parameter alamat kontrak terkait:
```bash
# contoh instansiasi goatnftwrapper
wasmd tx wasm instantiate <code_id_wrapper> '{"nft_contract":"<nft_addr>","goat_contract":"<goat_addr>"}' \
  --from wallet --label "goat-wrapper" \
  --gas-prices 0.025uatom --gas auto --gas-adjustment 1.3 \
  --chain-id testing-1 --node https://rpc.testnet.cosmos.network

# contoh instansiasi sapinftwrapper
wasmd tx wasm instantiate <code_id_sapi_wrapper> '{"nft_contract":"<sapi_nft_addr>","goat_contract":"<goat_addr>"}' \
  --from wallet --label "sapi-wrapper" \
  --gas-prices 0.025uatom --gas auto --gas-adjustment 1.3 \
  --chain-id testing-1 --node https://rpc.testnet.cosmos.network

# contoh instansiasi burn hook kambing
wasmd tx wasm instantiate <code_id_hook> '{"nft_contract":"<nft_addr>"}' \
  --from wallet --label "goat-burnhook" \
  --gas-prices 0.025uatom --gas auto --gas-adjustment 1.3 \
  --chain-id testing-1 --node https://rpc.testnet.cosmos.network

# contoh instansiasi burn hook sapi
wasmd tx wasm instantiate <code_id_sapi_hook> '{"nft_contract":"<sapi_nft_addr>"}' \
  --from wallet --label "sapi-burnhook" \
  --gas-prices 0.025uatom --gas auto --gas-adjustment 1.3 \
  --chain-id testing-1 --node https://rpc.testnet.cosmos.network

# contoh instansiasi barterengine
wasmd tx wasm instantiate <code_id_barter> '{"meat_contract":"<meat_addr>","rate_handler":"<rate_addr>"}' \
  --from wallet --label "barter" \
  --gas-prices 0.025uatom --gas auto --gas-adjustment 1.3 \
  --chain-id testing-1 --node https://rpc.testnet.cosmos.network
```


Setelah `goatnft` dideploy, pemilik dapat langsung memanggil `burn` pada kontrak NFT untuk menandai penyembelihan. Fungsi ini memicu `GoatNFTBurnHook` yang menjalankan `mint_subtype` pada kontrak MEAT dan mencetak `GOATMEAT` sesuai berat terakhir.

```bash
wasmd tx wasm execute <nft_address> '{"burn":{"token_id":1}}' \
  --from wallet --gas-prices 0.025uatom --gas auto --gas-adjustment 1.3 \
  --chain-id testing-1 --node https://rpc.testnet.cosmos.network
```
Catatan: pesan `burn_and_mint` pada versi terdahulu telah dihapus. Proses burn hanya mencetak `GOATMEAT`. GOAT diperoleh dengan membungkus NFT melalui `GoatNFTWrapper`.

## Contoh Query

```bash
# cek saldo
wasmd query wasm contract-state smart <address> '{"balance":{"address":"cosmos1..."}}'

# pending reward
wasmd query wasm contract-state smart <address> '{"pending_reward":{"address":"cosmos1..."}}'
```

Pastikan CLI `wasmd` Anda dikonfigurasi dengan key bernama `wallet` yang sesuai dengan jalur mnemonic pada `deploy-config/wasm-config.json`.
