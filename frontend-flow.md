# Alur Frontend

Antarmuka web menyediakan langkah sederhana untuk berinteraksi dengan GOAT dan MEAT. Pengguna diharapkan menghubungkan dompet yang kompatibel dengan jaringan tempat kontrak dideploy.

1. **Mint MEAT** – Hanya kontrak terotorisasi yang memanggil `mintSubtype` pada MEAT. Frontend memantau event `SubtypeMinted` dan memperbarui saldo.
2. **Stake GOAT** – setelah memiliki GOAT, pengguna dapat melakukan staking berapa pun. UI menampilkan reward yang menunggu lewat `pendingReward` serta waktu berikutnya klaim tersedia.
3. **Claim atau Compound** – setelah interval klaim terlewati, UI mengaktifkan tombol `claimReward` dan `compoundReward`. Opsi compound menambahkan reward kembali ke saldo staking.
4. **Unstake** – pengguna dapat melakukan unstake untuk menarik pokok beserta reward.

Seluruh pembaruan status berasal dari panggilan kontrak atau event sehingga saldo selalu konsisten dengan data on-chain.
