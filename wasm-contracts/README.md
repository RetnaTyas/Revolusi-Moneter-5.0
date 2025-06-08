# Membangun Kontrak CosmWasm

Folder ini memuat implementasi Rust untuk kontrak GOAT, MEAT, dan GoatNFT. Untuk mengompilasinya Anda memerlukan toolchain Rust standar serta target WebAssembly.

## Prasyarat

1. Instal [rustup](https://rustup.rs/) jika belum tersedia.
2. Tambahkan target build `wasm32-unknown-unknown`:
   ```bash
   rustup target add wasm32-unknown-unknown
   ```

## Membangun

Jalankan skrip build dari root repositori:
```bash
./wasm-contracts/build.sh
```
Skrip akan mengompilasi seluruh paket di direktori ini. Berkas `.wasm` yang dihasilkan ditempatkan pada folder `artifacts/` sementara schema JSON ditulis ke direktori `schema/` masing-masing paket.
