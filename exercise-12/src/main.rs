// 🛠️ Exercise 4: The Dangling Return Trap

// Description: This exercise tests your understanding of memory safety across function boundaries. You cannot return a reference to a variable that dies inside a function.

//     Write a function called get_server_status that returns a reference to a string: -> &String.

//     Inside that function, initialize a local variable: let status = String::from("ONLINE");. Try to return a reference to it: &status.

//     Read the compiler error regarding "missing lifetime specifier" or "returns a reference to data owned by the current function".

//     Fix the function signature and the return expression so that instead of a reference, the function completely transfers ownership of the data back to main. Unpack and print it in main.

// Desired Output:
// System Gateway Check: ONLINE

fn get_server_status() -> String {
    let status = String::from("ONLINE");
    status
}
fn main() {
    println!("System Gateway Check: {}", get_server_status());
}
