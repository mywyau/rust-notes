fn main() {
    let x = 2.0;   // two primitive types
    let x: f32 = 3.0;

    // Rust supports the basic mathematical operations you’d expect for all the number types:
    // addition, subtraction, multiplication, division, and remainder.

    // Integer division truncates toward zero to the nearest integer.

    let sum = 5 + 10;
    let difference = 95.5 + 4.3;
    let product = 4 * 30;
    let quotient = 56.7 * 32.2;
    let truncated = -5 / 3;

    let remainder = 43 % 5;

    println!("sum: {}", sum);
    println!("difference: {}", difference);
    println!("product: {}", product);
    println!("quotient: {}", quotient);
    println!("truncated: {}", truncated);
    println!("remainder: {}", remainder);
}


