// 🛠️ Exercise 2: The Double Editor Constraint

// Description: This exercise simulates a database management system trying to open multiple write handles to the same tracking counter. You can only have one mutable reference at a time.

//     In your main function, initialize a mutable integer let mut database_connections = 10;.

//     Create a mutable reference named writer_one that borrows database_connections.

//     Create a second mutable reference named writer_two that also borrows database_connections in the same scope.

//     Try to use writer_one to add 5 to the connection count, and then use writer_two to add 2 to the connection count.

//     Observe the compiler block you. Fix the program by enclosing the use of writer_one in its own distinct block using curly braces { ... } or by ensuring its usage is completely finished before writer_two is declared.

// Desired Output:

fn main() {
    let mut database_connections = 10;

    let writer_one = &mut database_connections;

    *writer_one += 5;

    let writer_two = &mut database_connections;
 
    *writer_two += 2;

    println!("Database pool scaling completed. Total connections: {}", database_connections);
}

// Curly braces case
// fn main() {
//     let mut database_connections = 10;

//     // We create an isolated inner scope block
//     {
//         let writer_one = &mut database_connections;
//         *writer_one += 5;
//     } // writer_one explicitly goes out of scope and drops RIGHT HERE!

//     // Now this is completely unburdened open space
//     let writer_two = &mut database_connections;
//     *writer_two += 2;

//     println!("Database pool scaling completed. Total connections: {}", database_connections);
// }