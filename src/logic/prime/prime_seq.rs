pub fn is_prime(n: u32) -> bool {
    for i in 2..f64::sqrt(n as f64) as u32 {
        if n % i == 0 {
            return false;
        }
    }
    return true;
}

pub fn nth_prime(n: u32) -> Vec<u32> {
    let mut counter = n;

    let mut result = Vec::new();
    let mut index = 2;

    loop {
        if is_prime(index) {
            result.push(index);
            counter -= 1;
        }

        if counter == 0 {
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
