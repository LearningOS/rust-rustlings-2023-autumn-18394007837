use std::f64;

fn main() {
    let pi = 3.14f64;  // Change the type to f64
    let radius = 5.00f64;  // Change the type to f64

    let area = pi * f64::powi(radius, 2);  // Change the type to f64

    println!(
        "The area of a circle with radius {:.2} is {:.5}!",
        radius, area
    )
}
