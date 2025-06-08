# Deploy Kontrak CosmWasm

Proyek ini menyertakan implementasi CosmWasm untuk seluruh kontrak inti di direktori `wasm-contracts/`:

- `starter` – token GOAT dengan logika staking
- `meat` – token MEAT yang mendukung swap dan pencetakan menggunakan native token. Menyediakan `redeem_for_meat` dan dapat menautkan kontrak `ratehandler` untuk rasio dinamis
- `goatnft` – kontrak NFT sederhana tempat setiap token menyimpan nilai berat yang dapat ditebus menjadi GOAT
- `ratehandler` – utilitas kecil yang menyimpan rasio swap terbaru dan memungkinkan pemilik memperbarui atau menonaktifkannya

Paket-paket ini mencerminkan kontrak Solidity di `contracts/`. Sebagian besar fungsi memiliki pesan execute yang setara, namun terdapat beberapa perbedaan penting:

- **MEAT** tidak dapat mencetak otomatis ketika menerima token native tanpa pesan. Pengguna **harus** memanggil `mint_with_native` dan menyertakan dana.
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
Instansiasi `meat`, `goatnft` dan `ratehandler` dengan perintah serupa. `ratehandler` tidak memerlukan parameter dan alamatnya dapat ditautkan ke MEAT menggunakan pesan `set_rate_handler` setelah deployment.

### Mencetak MEAT

Panggil entri `mint_with_native` sambil mengirim koin native untuk mencetak MEAT. Contoh:

```bash
wasmd tx wasm execute <meat_address> '{"mint_with_native":{}}' \
  --amount 1000000uatom --from wallet \
  --gas-prices 0.025uatom --gas auto --gas-adjustment 1.3 \
  --chain-id testing-1 --node https://rpc.testnet.cosmos.network
```

Mengirim koin tanpa pesan ini **tidak** akan mencetak token; koin hanya tersimpan di kontrak sampai ditarik pemilik.

Setelah `goatnft` dideploy, setiap pemilik NFT harus memberi approval pada kontrak GOAT sebelum token dapat dibakar. Contoh approval:

```bash
wasmd tx wasm execute <nft_address> '{"approve":{"spender":"<goat_addr>","token_id":"1"}}' \
  --from wallet --gas-prices 0.025uatom --gas auto --gas-adjustment 1.3 \
  --chain-id testing-1 --node https://rpc.testnet.cosmos.network
```

Jika sudah di-approve, tebus nilai NFT dengan memanggil `burn_and_mint` pada kontrak GOAT:

```bash
wasmd tx wasm execute <goat_address> '{"burn_and_mint":{"token_id":1}}' \
  --from wallet --gas-prices 0.025uatom --gas auto --gas-adjustment 1.3 \
  --chain-id testing-1 --node https://rpc.testnet.cosmos.network
```

## Contoh Query

```bash
# cek saldo
wasmd query wasm contract-state smart <address> '{"balance":{"address":"cosmos1..."}}'

# pending reward
wasmd query wasm contract-state smart <address> '{"pending_reward":{"address":"cosmos1..."}}'
```

Pastikan CLI `wasmd` Anda dikonfigurasi dengan key bernama `wallet` yang sesuai dengan jalur mnemonic pada `deploy-config/wasm-config.json`.
