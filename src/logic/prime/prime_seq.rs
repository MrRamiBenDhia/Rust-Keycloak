pub fn is_prime(n: u32) -> bool {
    for i in 2..n {
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
    let max = 1000;

    loop {
        if is_prime(index) {
            result.push(index);
            counter -= 1;
        }

        if counter == 0 || index > max {
            break;
        }
        index += 1;
    }
    return result;
}

fn main() {
    let n = 10;

    let result = nth_prime(n);

    println!("Welcome to Prime Numbers");
    println!("Input: {}", n);
    println!("Result = {:?}", result);
}
