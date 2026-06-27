// 🧠 Latihan 3: Token Session Manager (Level: Pro)

// Fokus: #[derive], Struct Update Syntax (..), dan Pembatasan Lifetime dengan {}

// Di dunia backend, kita sering mengelola sesi login user menggunakan token. Di latihan ini, Anda akan memanipulasi token tersebut secara aman.

//     Tugas Anda:

//         Buat struct Session dengan field: id (u64), token (String), dan is_expired (bool).
#[derive(Debug)]

struct Session {
    id: u64,
    token: String,
    is_expired: bool
}
//         Tambahkan #[derive(Debug)] di atas struct agar ia bisa di-print secara utuh.
fn main() {
//         Di dalam fungsi main:

//             Buat variabel session_aktif (harus mut).
    let mut session_active = Session {
        id: 1,
        token: "Nj382^&*1".to_string(),
        is_expired: false
    };

    {
        // session_active.token = "BroSKY".to_string();
        println!("Sesi aktif: {:?}", session_active);
    }
//             Buat variabel baru bernama session_Expired menggunakan Struct Update Syntax (..) yang menyalin data dari session_aktif, tetapi ubah nilai is_expired-nya menjadi true. (Hati-hati dengan efek Move pada String token! Bagaimana cara mengatasinya? Clue: Anda bisa menggunakan .clone() pada token sebelum update syntax jika diperlukan).
    
    let session_exp = &mut Session {
        ..session_active
    };

    session_exp.is_expired = true;
    session_active.token = "qwem".to_string();
//             Gunakan blok scope {} terpisah untuk meminjam session_aktif secara mutable (&mut) untuk mensimulasikan pembaruan token (misal mengubah teks tokennya). Setelah blok {} itu selesai, print variabel session_aktif untuk membuktikan bahwa Borrow Checker sudah mengizinkan pembacaan kembali.
    println!("Sesi aktif: {:?}", session_active);
    println!("Sesi Expired: {:?}", session_exp);
}
