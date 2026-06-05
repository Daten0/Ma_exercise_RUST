// 🛠️ Exercise 3: The Read-While-Writing Paradox (Aliasing)

// Description: This exercise demonstrates what happens when a function tries to read from and write to the same variable simultaneously.

//     Create a function named merge_logs with this exact signature: fn merge_logs(destination: &mut String, source: &String). Inside the function, append the source string to the destination string.

//     In your main function, create a mutable string: let mut log_buffer = String::from("WARN: Timeout; ");.

//     Try to call the function by passing log_buffer as both arguments: merge_logs(&mut log_buffer, &log_buffer);.

//     Notice how the compiler prevents this because it violates the aliasing rule (reading while writing). Fix this inside main by creating a separate, independent snapshot of the string data to pass as the read-only parameter.

// Desired Output:
// Aggregated Log Buffer: WARN: Timeout; WARN: Timeout;

fn main() {
    let mut log_buffer = String::from("WARN: Timeout; ");
    let snapshot_buffer = log_buffer.clone();
    
    merge_logs(&mut log_buffer, &snapshot_buffer);
    
    println!(" Aggregated Log Buffer: {}", log_buffer);
}

fn merge_logs(destination: &mut String, source: &String) {
    destination.push_str(source);
}