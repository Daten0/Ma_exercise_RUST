// 🗄️ Exercise 3: Database SQL Query Assembly

// Description: Simulate assembling a SQL database query where you combine static query blueprints with dynamic incoming parameters. You will practice converting from &str to String and appending data.

//     In main, define an immutable string literal for a table name: let table = "users"; (Type: &str).

//     Define a mutable owned string for the filter clause: let mut condition = String::from("WHERE id = ");. Use a mutation method to append the string literal "42" to the end of it.

//     Now, build a brand new mutable String variable called sql_query initialized with the text "SELECT * FROM ".

//     Append the table name to sql_query, append a space string " ", and then append the final condition string to it.

//     Print out the completely assembled query string.

// Desired Output:
// Executing SQL: SELECT * FROM users WHERE id = 42

fn main() {
    let table = "users ";
    let mut condition = String::from("WHERE id = ");
    condition.push_str("42");
    let mut sql_query = String::from("SELECT * FROM ");
    sql_query.push_str(table);
    sql_query.push_str(&condition);
    println!("Executing SQL: {}", sql_query);
}
