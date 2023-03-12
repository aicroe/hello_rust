fn main() {
    for n in 0..51 {
        let fibonacci = fibonacci(n);
        println!("Fibonacci n={n} => {fibonacci}");
    }
}

fn fibonacci(n: u32) -> u64 {
    if n == 0 {
        return 0;
    }

    if n == 1 {
        return 1;
    }

    let mut fibonacci_prev_2: u64 = 0;
    let mut fibonacci_prev_1: u64 = 1;

    for _ in 2..n {
        let current_fibonacci = fibonacci_prev_1 + fibonacci_prev_2;

        // Update previous fibonacci state
        fibonacci_prev_2 = fibonacci_prev_1;
        fibonacci_prev_1 = current_fibonacci;
    }

    return fibonacci_prev_1 + fibonacci_prev_2;
}