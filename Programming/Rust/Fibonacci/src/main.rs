use std::io;

fn main() {
    println!("Please input a number.");
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");

    println!("You entered: {n}");
    
    let fib = fibonacci(n.trim().parse().unwrap());

    println!("The Fibonacci number for {n} is {fib}");
}

fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
}