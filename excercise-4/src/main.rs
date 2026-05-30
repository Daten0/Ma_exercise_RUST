// Initialize two variables: an integer representing a total score (87) and another integer representing the maximum possible score (100).

// Write a function that accepts these two values, calculates the percentage score, and returns it as a 64-bit float (f64).

// Note: Ensure you cast the variables before performing the division to avoid truncation!
fn main() {
    let total_scores = 87;
    let max_scores = 100;

    println!("Final Exam Result: {:.2}%", exam_results(&total_scores, &max_scores));
}

fn exam_results(scores: &i32, max : &i32) -> f64 {
    // before
    // let results = (*num1 as i32 / *num2 as i32) as f64 * 100.00;
    // after
    let results = (*scores as f64 / *max as f64) * 100.00;
    results
}