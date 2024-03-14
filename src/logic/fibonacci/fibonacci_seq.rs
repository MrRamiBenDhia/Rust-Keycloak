pub fn fibonacci_seq(n: u32) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci_seq(n - 1) + fibonacci_seq(n - 2),
    }
}

fn main() {
    let n = 25;

    let result = fibonacci_seq(n);

    println!("Welcome to Fibonacci");
    println!("Input: {}", n);
    println!("Result = {}", result);
}
