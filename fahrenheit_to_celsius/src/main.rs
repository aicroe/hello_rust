fn main() {
    let temperatures_in_fahrenheit = [0.0, 100.0, 32.0, 200.0, -100.0];

    for fahrenheit in temperatures_in_fahrenheit {
        let celsius = fahrenheit_to_celsius(fahrenheit);
        println!("{fahrenheit}°F is equal to {celsius}°C");
    }
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * (5.0 / 9.0)
}
