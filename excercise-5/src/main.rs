// Initialize a string variable containing a port number "8080".

// Shadow the variable by parsing it into a numerical integer.

// Write a separate function that evaluates if this numerical port is within a safe, non-privileged range (greater than 1024) and returns a boolean.

// Print the verdict based on the function's return value.


fn main() {
    let port = "1023";
    let port: i32 = port.parse().expect("No Data types");
    println!("Checking network configuration...");
    println!("Is port 8080 safely accessible? {}", check_port(port));
}

fn check_port(num: i32) -> bool {
    if num > 1024 {
        false
    } else {
        true
    }
}