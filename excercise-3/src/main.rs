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
