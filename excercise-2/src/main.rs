// Exercise 2: Inventory Tracker (Mutability & Tuples)

// Description: Write a program that tracks an item's stock and price using a tuple.

//     Create a mutable tuple containing an item's ID (integer), its stock count (integer), and its unit price (float).

//     Write a function that calculates the total inventory value (stock multiplied by price) and returns it.

//     Back in the main function, simulate selling 3 units of the item by modifying the tuple's stock count directly.

//     Recalculate and print the new total inventory value.

// Output : 
// Initial total value: $249.5
// After selling 3 units, the new total value is: $124.75

fn main() {

    let mut tuples = (1, 5, 49.90);

    println!("Initial total value: ${:?}", invent_track(&tuples));

    tuples.1 -= 3;

    println!("After selling 3 units, the new total value is: ${:?}", invent_track(&tuples));
}

fn invent_track(_tup : &(u8, i32, f32)) -> f32 {

    let (_id, stock, prices) = _tup;

    (*stock as f32) * prices
}