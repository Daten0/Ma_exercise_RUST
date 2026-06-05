// 📋 Part 2: Working with Vectors (The Heap Move Engine)

// Description: A Vec<T> (Vector) in Rust is a dynamic array that stores its elements on the Heap, exactly like a String. This exercise tests how moves occur across complex types.

//     In your main function, create a mutable Vector of strings containing a couple of server names: let mut server_cluster = vec![String::from("Server_A"), String::from("Server_B")];.

//     Create a function named retire_cluster that takes ownership of a Vec<String>. Inside the function, print out the cluster list.

//     In your main function, assign your cluster to a brand new variable: let backup_cluster = server_cluster;.

//     Try to add a new server to the original cluster using server_cluster.push(String::from("Server_C"));.

//     Pass backup_cluster into your retire_cluster function.

//     Observe the compilation behavior. Remove or comment out the line that violates ownership rules so that the program runs cleanly.

// Desired Output:
// [Retirement Unit] Safely shutting down the following nodes: ["Server_A", "Server_B"]

fn retire_cluster(vec : &Vec<String>) {
    println!("[Retirement Unit] Safely shutting down the following nodes: {:?}", vec);
}

fn main() {
    let mut server_cluster = vec![String::from("Server_A"), String::from("Server_B")];

    server_cluster.push(String::from("Server_C"));

    let backup_cluster = &server_cluster;

    retire_cluster(backup_cluster);

    // println!("Hello, world!");
}
