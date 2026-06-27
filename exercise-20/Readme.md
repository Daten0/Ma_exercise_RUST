# My Code
```Rust
#[derive(Debug)]

struct Session {
    id: u64,
    token: String,
    is_expired: bool
}

fn main() {
    let mut session_active = Session {
        id: 1,
        token: "Nj382^&*1".to_string(),
        is_expired: false
    };

    {
        // session_active.token = "BroSKY".to_string();
        println!("Sesi aktif: {:?}", session_active);
    }
    
    let session_exp = &mut Session {
        ..session_active
    };

    session_exp.is_expired = true;
    session_active.token = "qwem".to_string();

    println!("Sesi aktif: {:?}", session_active);
    println!("Sesi Expired: {:?}", session_exp);
}
```
# AI Code
```Rust
#[derive(Debug)]
struct Session {
    id: u64,
    token: String,
    is_expired: bool,
}

fn main() {
    // 1. Membuat session aktif awal
    let mut session_active = Session {
        id: 1,
        token: "Nj382^&*1".to_string(),
        is_expired: false,
    };

    // ==========================================
    // TANTANGAN: Buat Session Expired Tanpa Merusak Session Aktif
    // ==========================================
    let session_exp = Session {
        is_expired: true, // Kita ubah is_expired jadi true
        token: session_active.token.clone(), // KITA KLONING TOKENNYA! 
        ..session_active // Sisa field (id) disalin menggunakan Copy karena u64 otomatis bertipe Copy
    };
    
    // Hasilnya: session_active MASIH UTUH dan tidak hancur di memori!

    // ==========================================
    // TANTANGAN: Gunakan Scope {} untuk Update Token Secara Mutable
    // ==========================================
    {
        // Kita pinjam session_active secara mutable di dalam scope ini
        let session_updater = &mut session_active;
        session_updater.token = "BroSKY".to_string(); 
        
    } // <-- PENTING: Di baris ini, variabel 'session_updater' MATI.
      // Pinjaman mutable selesai, memori session_active dibebaskan kembali!

    // 2. Sekarang kita bisa mencetak keduanya dengan aman tanpa amukan Borrow Checker
    println!("Sesi Aktif Baru : {:#?}", session_active);
    println!("Sesi Expired    : {:#?}", session_exp);
}
```