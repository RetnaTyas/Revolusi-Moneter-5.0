# Alur Frontend

Antarmuka web menyediakan langkah sederhana untuk berinteraksi dengan GOAT dan MEAT. Pengguna diharapkan menghubungkan dompet yang kompatibel dengan jaringan tempat kontrak dideploy.

1. **Mint MEAT** – kirim mata uang native dari dompet langsung ke kontrak MEAT melalui UI. Frontend memantau event `MintedWithNative` dan memperbarui saldo MEAT.
2. **Tukar MEAT ke GOAT** – lakukan approval MEAT ke kontrak lalu panggil `swapMEATForGOAT`. GOAT yang diperoleh ditampilkan setelah transaksi diproses.
3. **Stake GOAT** – setelah memiliki GOAT, pengguna dapat melakukan staking berapa pun. UI menampilkan reward yang menunggu lewat `pendingReward` serta waktu berikutnya klaim tersedia.
4. **Claim atau Compound** – setelah interval klaim terlewati, UI mengaktifkan tombol `claimReward` dan `compoundReward`. Opsi compound menambahkan reward kembali ke saldo staking.
5. **Unstake dan Tukar Balik** – pengguna dapat melakukan unstake untuk menarik pokok beserta reward, kemudian menukar GOAT kembali ke MEAT dengan `swapGOATForMEAT`.

Seluruh pembaruan status berasal dari panggilan kontrak atau event sehingga saldo selalu konsisten dengan data on-chain.
