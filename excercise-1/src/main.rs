fn main() {
    let temp = 100.67;
    println!("The body temperature of {}°F is exactly {}°C.", temp, temp_converter(temp));
}

fn temp_converter(_num : f64) -> f64 {
    let fahrenheit = 98.6;
    let fahrenheit = (fahrenheit - 32 as f64) * 5 as f64 / 9 as f64;

    let celcius: f64 = fahrenheit;
    return celcius;

}