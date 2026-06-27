// 🛠️ Latihan 1: Sistem Inventaris Gudang IoT (Level: Hangat)

// Fokus: Named-Field Struct, Constructor (Self), Mutasi Data (&mut self), dan Unit Type ()

// Anda diminta untuk membuat sistem pencatatan stok komponen elektronik untuk proyek IoT Anda.

//     Tugas Anda:

use std::cmp::Ordering;

//         Buat struct Komponen dengan field: nama (String), kode (String), dan stok (u32).
struct Component {
    nama: String,
    kode: String,
    stok: u32
}
//         Buat blok impl Komponen dan ciptakan Associated Function bernama new(nama: &str, kode: &str, stok: u32) -> Self sebagai pabrik pembuatnya.
impl Component {
    fn new(nama: &str, kode: &str, stok: u32) -> Self {
        Self {
            nama: nama.to_string(),
            kode: kode.to_string(),
            stok
        }
    }
//         Buat Method bernama tambah_stok(&mut self, jumlah: u32) yang bertugas menambah angka stok yang ada. (Ingat aturan fungsi tindakan/prosedur yang mengembalikan unit type).
    fn tambah_stok(&mut self, jumlah: u32) {
        self.stok += jumlah;
    }
//         Buat Method bernama ambil_stok(&mut self, jumlah: u32) -> bool. Jika stok cukup, kurangi stok dan kembalikan true. Jika stok kurang, jangan kurangi stok dan kembalikan false.
    fn ambil_stok(&mut self, jumlah: u32) -> bool {
        // match self.stok.cmp(&jumlah) {
        match self.stok {
            current_stok if current_stok >= jumlah => {
                self.stok -= jumlah;
                true
            },
            _ => false,
            // Ordering::Equal | Ordering::Greater => {
            //     self.stok -= jumlah;
            //     true
            // },
            // Ordering::Less => false
        }
    }
}




fn main() {
    let mut komponen = Component::new("ESP32", "XZZ", 5);

    komponen.tambah_stok(10);
    println!("Stok : {}", komponen.ambil_stok(20));
}
