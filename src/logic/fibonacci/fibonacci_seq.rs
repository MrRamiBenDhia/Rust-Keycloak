pub fn fibonacci_seq(n: u32) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci_seq(n - 1) + fibonacci_seq(n - 2),
    }
}

fn main() {
    let n = 25;


    println!("Welcome to Fibonacci");
    println!("Input nth number: ", );
    
    let mut line = String::new();
    let _read = std::io::stdin().read_line(&mut line).unwrap();
    
    let x: u32 = line.trim().parse::<u32>().unwrap();
    
    let result = fibonacci_seq(x);

    println!("Input in: {}", line);
    println!("Result = {:?}", result);
    println!("Click Enter to close this window...");
    std::io::stdin().read_line(&mut String::new()).unwrap();
}
