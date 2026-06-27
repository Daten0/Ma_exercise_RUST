// 🛑 Latihan 2: Detektif Bug "Move" pada User Validator (Level: Menantang)

// Fokus: Mengatasi Error Borrow Checker, Kepemilikan String, dan Penggunaan &self

// Teman kerja Anda menulis kode di bawah ini untuk sistem backend validasi user, tetapi kodenya error saat di-compile karena masalah Ownership (data String yang pindah tangan/Move).

//     Tugas Anda: Perbaiki fungsi di dalam impl atau cara pemanggilannya di main agar kode ini bisa berjalan dengan sukses tanpa error!

struct User {
    username: String,
    role: String,
}

impl User {
    // BUG 1: Periksa parameter di bawah ini, apakah aman bagi data internal struct?
    fn dapatkan_role(&self) -> &str {
        &self.role
    }
}

fn main() {
    let user_api = User {
        username: String::from("siddiq_dev"),
        role: String::from("admin"),
    };

    let role_sekarang = user_api.dapatkan_role();
    
    // BUG 2: Baris di bawah ini ERROR karena user_api dianggap sudah "cacat/mati"
    println!("User {} adalah seorang {}", user_api.username, role_sekarang);
}

// Alasan kenapa Bug 2 terjadi karena role_sekarang memperoleh nilai dari properti User yaitu role