// Exercise 1: Temperature Converter with Shadowing

// Description: Create a program that converts a temperature value from Fahrenheit to Celsius using a function.

//     Initialize an immutable variable for the initial Fahrenheit value (98.6).

//     Use variable shadowing to convert that value into Celsius using the formula: (Fahrenheit - 32) * 5 / 9.

//     Pass the final shadowed variable to a function that prints a formatted message detailing both values.

// Output : The body temperature of 98.6°F is exactly 37°C.
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