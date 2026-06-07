// Initial code by me;
fn main() {
    let logs = [
    "GET /api/v1/users HTTP/1.1 | STATUS: 200 | AUTH: Bearer xyz123",
    "POST /api/v1/deploy HTTP/1.1 | STATUS: 201 | AUTH: Bearer abc456",
    "GET /api/v1/admin HTTP/1.1 | STATUS: 403 | AUTH: Bearer ttt789",
    "POST /api/v1/users HTTP/1.1 | STATUS: 500 | AUTH: Bearer qqq000",
    ];

    let mut successful_requests = 0;

    for log in logs {
        if is_successful(log) {
            successful_requests += 1
        }

        let auth = mask_auth(log);

        println!("[SECURE LOG]: {}", auth);
    }
    println!("Analysis Complete. Total Successful Requests (2xx): {}", successful_requests);

}


fn mask_auth(raw_log: &str) -> String {
    let token_value = &raw_log[0..56];

    let mut secret_token = token_value.to_string();
    
    secret_token.push_str(" CLASSIFIED");

    secret_token
}

fn is_successful(raw_log: &str) -> bool {
    if raw_log.contains("STATUS: 200") || raw_log.contains("STATUS: 201") {
        true
    } else {
        false
    }
}

// AI
// fn main() {
//     let logs = [
//         "GET /api/v1/users HTTP/1.1 | STATUS: 200 | AUTH: Bearer xyz123",
//         "POST /api/v1/deploy HTTP/1.1 | STATUS: 201 | AUTH: Bearer abc456",
//         "GET /api/v1/admin HTTP/1.1 | STATUS: 403 | AUTH: Bearer ttt789",
//         "POST /api/v1/users HTTP/1.1 | STATUS: 500 | AUTH: Bearer qqq000",
//     ];

//     let mut successful_requests = 0;

//     for log in logs {
//         // Fix 1: Pass the log slice to check success conditions
//         if is_successful(log) {
//             successful_requests += 1;
//         }

//         // Fix 2: Let mask_auth return the COMPLETE sanitized line
//         let secure_line = mask_auth(log);
//         println!("[SECURE LOG]: {}", secure_line);
//     }
    
//     // Clean formatting: dropped debug wrapper `{:?}` for display `{}`
//     println!("\nAnalysis Complete. Total Successful Requests (2xx): {}", successful_requests);
// }

// fn mask_auth(raw_log: &str) -> String {
//     // Look at how simple string slicing can be instead of looping split elements!
//     // The string "Bearer " ends right before the token value starts.
//     // In all of our log sequences, the token value starts at index 55.
    
//     // We slice everything from index 0 up to 55 to keep the log info intact!
//     let clean_log_prefix = &raw_log[0..55];
    
//     // We construct a brand new owned string by converting our slice view 
//     // and pushing our classified label to the end of it.
//     let mut masked_string = clean_log_prefix.to_string();
//     masked_string.push_str("CLASSIFIED");
    
//     masked_string
// }

// fn is_successful(raw_log: &str) -> bool {
//     // Instead of splitting whitespaces, you can leverage native `contains` 
//     // checks directly on your log slice to keep your code readable and robust!
    
//     // Fix 3: Changed the && to || so either successful state triggers true
//     if raw_log.contains("STATUS: 200") || raw_log.contains("STATUS: 201") {
//         true
//     } else {
//         false
//     }
// }