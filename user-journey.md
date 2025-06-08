# Perjalanan Pengguna

Dokumen ini menggambarkan pengalaman umum bagi peserta baru di ekosistem GOAT/MEAT.

1. **Mendapatkan MEAT**
   * Pengguna membuka aplikasi web, menghubungkan dompet, lalu mengirim sejumlah kecil token native ke kontrak MEAT. Kontrak akan mencetak MEAT sesuai `DepositRate` saat ini.
2. **Mengonversi ke GOAT**
   * Melalui antarmuka swap, pengguna menyetujui MEAT dan memanggil `swapMEATForGOAT` yang mentransfer MEAT dan mengembalikan GOAT berdasarkan rate terkini dari `RateHandler`.
3. **Staking untuk Reward**
   * Dengan token GOAT di dompet, pengguna melakukan staking untuk mulai memperoleh reward. UI menampilkan hitungan mundur hingga klaim diperbolehkan (`minClaimInterval`).
4. **Panen**
   * Setelah interval terpenuhi, pengguna dapat mengklaim reward secara langsung atau menggabungkannya kembali ke staking guna menambah pokok.
5. **Keluar**
   * Ketika selesai, pengguna melakukan unstake untuk menarik GOAT asli beserta reward lalu dapat segera menukar kembali ke MEAT. MEAT tersebut bisa diperdagangkan atau disimpan.

Sepanjang siklus ini pengguna berinteraksi dengan kedua kontrak hanya melalui fungsi yang diizinkan sehingga dana tetap aman namun fleksibel.
