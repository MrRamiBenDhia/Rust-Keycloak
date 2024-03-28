pub fn fibonacci_seq(n: u32) -> u128 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci_seq(n - 1) + fibonacci_seq(n - 2),
    }
}

pub fn fibonacci_seq_none_recursive(n: u32) -> Vec<u128> {
    let mut result:Vec<u128> = vec![1, 1];
    let mut count = 1;

    while result.len() < n as usize {
        result.push(result[count - 1] + result[count]);
        count += 1;
    }

    result
}

fn main() {
    let _n = 25;

    println!("Welcome to Fibonacci");
    println!("Input nth number: ",);

    let mut line = String::new();
    let _read = std::io::stdin().read_line(&mut line).unwrap();

    let x: u32 = line.trim().parse::<u32>().unwrap();

    let result = fibonacci_seq(x);

    println!("Input in: {}", line);
    println!("Result = {:?}", result);
    println!("Click Enter to close this window...");
    std::io::stdin().read_line(&mut String::new()).unwrap();
}
