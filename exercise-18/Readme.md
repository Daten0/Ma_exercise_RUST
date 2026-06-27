# My Code
```Rust
struct Component {
    nama: String,
    kode: String,
    stok: u32
}

impl Component {
    fn new(nama: &str, kode: &str, stok: u32) -> Self {
        Self {
            nama: nama.to_string(),
            kode: kode.to_string(),
            stok
        }
    }

    fn tambah_stok(&mut self, jumlah: u32) {
        self.stok += jumlah;
    }

    fn ambil_stok(&mut self, jumlah: u32) -> bool {
        match self.stok {
            current_stok if current_stok >= jumlah => {
                self.stok -= jumlah;
                true
            },
            _ => false,
        }
    }
}


fn main() {
    let mut komponen = Component::new("ESP32", "XZZ", 5);

    komponen.tambah_stok(10);
    println!("Stok : {}", komponen.ambil_stok(20));
}
```

# AI Code
```Rust
use std::cmp::Ordering;

struct Component {
    nama: String,
    kode: String,
    stok: u32
}

impl Component {
    fn new(nama: &str, kode: &str, stok: u32) -> Self {
        Self {
            nama: nama.to_string(),
            kode: kode.to_string(),
            stok
        }
    }

    fn tambah_stok(&mut self, jumlah: u32) {
        self.stok += jumlah;
    }

    fn ambil_stok(&mut self, jumlah: u32) -> bool {
        match self.stok.cmp(&jumlah) {
             Ordering::Equal | Ordering::Greater => {
                 self.stok -= jumlah;
                 true
             },
             Ordering::Less => false
        }
    }
}

fn main() {
    let mut komponen = Component::new("ESP32", "XZZ", 5);

    komponen.tambah_stok(10);
    println!("Stok : {}", komponen.ambil_stok(20));
}

```