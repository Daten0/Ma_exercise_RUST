// Exercise 5: The Config Shadow-Mesh

// Description: Simulate a configuration loading pipeline where input types change, requiring strict scope management and shadowing.

// Initialize a string variable containing a port number "8080".

// Shadow the variable by parsing it into a numerical integer.

// Write a separate function that evaluates if this numerical port is within a safe, non-privileged range (greater than 1024) and returns a boolean.

// Print the verdict based on the function's return value.

// Output:
// Checking network configuration...
// Is port 8080 safely accessible? true

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