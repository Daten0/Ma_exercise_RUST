// Exercise 3: Array Data Processing & Bounds Checking

// Description: Create a fixed-size array of 5 integers representing daily steps.

//     Write a function that takes this array as a reference and returns both the total steps and the average daily steps as a tuple (u32, f32).

//     In your main function, unpack this returned tuple and print the results cleanly.

// Output:
// Total steps tracked: 34200
// Average daily step count: 6840.0
fn main() {
    let steps: [u32;100] = [1;100];
    let (step, averages) = step(&steps);

    println!("Total steps tracked: {}", step);
    println!("Average daily step count: {}", averages);
}

fn step(arr : &[u32; 100]) -> (u32, f32) {
    let total_steps = arr.iter().sum();
    let average_steps = (arr.len() / 5) as f32;
    (total_steps, average_steps)
}
