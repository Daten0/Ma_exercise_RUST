// 📦 Part 1: String Transmission & Ownership Transference

// Description: Simulate an API gateway receiving a transaction token payload and passing it downstream to an auth validator function.

//     In your main function, initialize a heap-allocated String variable named payload_token containing the text "SECURE_JWT_TOKEN".

//     Initialize an integer variable named bytes_processed on the stack containing the value 16.

//     Create a function named validate_token that accepts a String parameter and an i32 parameter. It shouldn't return anything, but it should print both values.

//     Call validate_token inside your main function and pass your two variables into it.

//     Right after the function call in main, write two separate println! statements: one trying to print bytes_processed and another trying to print payload_token.

//     Observe what the compiler does. To make the program compile successfully and match the desired output below, comment out the single line that causes an ownership compile error.

// Desired Output:
// [Validator] Processing payload token: SECURE_JWT_TOKEN (16 bytes)
// [Main] Post-validation check: Integer bytes_processed is still valid and holds: 16


fn main() {
    let payload_token = String::from("SECURE_JWT_TOKEN");
    let bytes_processed : u32 = 16;
    validate_token(&payload_token, bytes_processed);
    println!("Payload Token : {}", payload_token);
    println!("bytes_processed : {}", bytes_processed);
}

fn validate_token(token: &String, bytes: u32) {
    println!("Processing payload token: {} (16 bytes)", token);
    println!("Post-validation check: Integer bytes_processed is still valid and holds: {}", bytes);
}