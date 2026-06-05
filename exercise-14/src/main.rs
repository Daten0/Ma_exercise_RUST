// 🛡️ Exercise 2: Header Aggregator (Deref Coercion)

// Description: This exercise tests Deref Coercion—the feature that allows a function expecting a read-only &str to automatically accept a reference to an owned String (&String) without any manual type conversions or cloning.

//     Create a utility function named print_header that takes a single string slice parameter (header: &str) and prints it out.

//     In your main function, create a static string literal named static_token (which is naturally a &str) initialized to "Bearer xyz123".

//     Next, create a mutable owned String named dynamic_token initialized to "Token ". Use .push_str() to append a dynamic ID "abc456" to it.

//     Call your print_header function twice in a row: once passing the static_token directly, and once passing a reference to dynamic_token.

// Desired Output:
// [System Header] Value: Bearer xyz123
// [System Header] Value: Token abc456

fn print_header(header: &str) {
    println!("[System Header] Value: {header}");
}
fn main() {
    let static_token = "Bearer xyz123";
    let mut dynamic_token = String::from("Token ");

    dynamic_token.push_str("abc456");

    print_header(static_token);
    print_header(&dynamic_token);
}
