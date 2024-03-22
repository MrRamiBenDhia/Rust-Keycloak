pub fn is_prime(n: u32) -> bool {
    let limit = (n as f64).sqrt() as u32 + 1;
    for i in 2..limit {
        if n % i == 0 {
            return false;
        }
    }
    true
}

pub fn nth_prime(n: u32) -> Vec<u32> {

    let mut result = Vec::new();
    let mut index = 2;

    loop {
        if is_prime(index) {
            result.push(index);
        }

        if result.len() >= n.try_into().unwrap() {
            break;
        }
        index += 1;
    }
    return result;
}

fn main() {
    let start_time = std::time::Instant::now(); // Start measuring time

    println!("Welcome to Prime Numbers");
    println!("Input nth number: ",);

    let mut line = String::new();
    let _read = std::io::stdin().read_line(&mut line).unwrap();

    let x: u32 = line.trim().parse::<u32>().unwrap();

    let result = nth_prime(x);

    // println!("Input in: {}", line);
    println!("Result = {:?}", result);

    let elapsed_time = start_time.elapsed();
    let elapsed_millis = elapsed_time.as_millis();
    println!("Function executed in {} milliseconds", elapsed_millis);

    println!("Click Enter to close this window...");
    std::io::stdin().read_line(&mut String::new()).unwrap();
}
