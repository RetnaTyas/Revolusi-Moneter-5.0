# Perjalanan Pengguna

Dokumen ini menggambarkan pengalaman umum bagi peserta baru di ekosistem GOAT/MEAT.

1. **Mendapatkan MEAT**
   * Pengguna membuka aplikasi web, menghubungkan dompet, lalu mengirim sejumlah kecil token native ke kontrak MEAT. Kontrak akan mencetak MEAT sesuai `DepositRate` saat ini.
2. **Staking untuk Reward**
   * Pengguna membungkus GoatNFT melalui `GoatNFTWrapper` untuk memperoleh GOAT.
   * GOAT tersebut kemudian di-stake dan UI menampilkan hitungan mundur hingga klaim diperbolehkan (`minClaimInterval`).
3. **Panen**
   * Setelah interval terpenuhi, pengguna dapat mengklaim reward secara langsung atau menggabungkannya kembali ke staking guna menambah pokok.
4. **Keluar**
   * Ketika selesai, pengguna melakukan unstake untuk menarik GOAT asli beserta reward. Membakar NFT setelah penyembelihan akan menghasilkan `GOATMEAT` yang dapat dibarter dengan token produk lain melalui `RateHandler`.

Sepanjang siklus ini pengguna berinteraksi dengan kedua kontrak hanya melalui fungsi yang diizinkan sehingga dana tetap aman namun fleksibel.
