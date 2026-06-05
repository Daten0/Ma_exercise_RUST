// 🛠️ Exercise 1: The Active Viewer Conflict

// Description: This exercise tests your understanding of overlapping reference lifetimes. You cannot mutate data while someone else holds a read-only lens to it.

//     In your main function, create a mutable String variable named api_route initialized to "api/v1/deploy".

//     Create an immutable reference named current_view that borrows api_route.

//     On the very next line, try to modify the original api_route by appending "/v2" to it using .push_str().

//     Right after the modification, try to print current_view using a println! statement.

//     Attempt to compile the code and read the error. To fix it, rearrange the sequence of code inside main so that the read-only lens is finished before the mutation happens.

// Desired Output:

// Inspecting current route: api/v1/deploy
// Upgraded target route to: api/v1/deploy/v2

fn main() {
    let mut api_route = String::from("api/v1/deploy");

    let current_view = &api_route;

    println!("Inspecting current route: {}", current_view);

    api_route.push_str("/v2");

    println!("Upgraded target route to: {}", api_route);
    
}
