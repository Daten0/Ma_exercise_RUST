// 🌐 Exercise 1: The API Path Splitter (Slicing Memory)

// Description: Imagine you are building a lightweight web router (like a mini Hono or Axum). You receive a full URL route path as an owned String on the heap, and you need to inspect a specific section of it without copying the text data into a new variable.

//     In your main function, initialize a heap-allocated String named full_path with the value "/api/v1/users".

//     Write a function called extract_version that takes a string slice parameter (&str) and returns a string slice (&str).

//     Inside main, use Rust's range slicing syntax [start..end] on full_path to grab just the "v1" part (hint: "v1" occupies string indices 5 to 7).

//     Pass that slice into your extract_version function, catch the return value, and print both the extracted version and the original full path.

// Desired Output:
// Extracted API Version: v1
// Original path remains untouched: /api/v1/users


fn extract_version(path : &str) -> &str {
    path
}
fn main() {
    let full_path = String::from("/api/v1/users");
    let slice = &full_path[5..=6];
    let extract_path = extract_version(slice);
    println!("Extracted API Version: {}", extract_path);
    println!("Original path remains untouched: {}", full_path);
}
