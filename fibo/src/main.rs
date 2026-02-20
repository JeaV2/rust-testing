use std::io;
use num_bigint::BigInt;

fn main() {
    println!("Enter the number of Fibonacci numbers to generate:");
    println!("Type \"q\" to exit.");
    
    'input_loop: loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if input.trim() == "q" {
            break;
        }
        
        let n: u64 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type in a number");
                continue;
            }
        };

        for i in 0..=n {
            println!("Fibonacci number {}: {}", i, fib(i));
            if i == n {
                println!("Done!");
                break 'input_loop;
            }
        }
    }
}

fn fib(n: u64) -> BigInt {
    if n == 0 {
        return BigInt::from(0);
    }
    if n == 1 {
        return BigInt::from(1);
    }
    
    let mut a = BigInt::from(0);
    let mut b = BigInt::from(1);
    
    for _ in 2..=n {
        let temp = &a + &b;
        a = b;
        b = temp;
    }
    
    return b;
}