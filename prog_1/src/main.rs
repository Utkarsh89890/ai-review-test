fn main() {
    println!("Starting basic calculations...");

    let a = 10;
    let b = 5;

    println!("a = {}", a);
    println!("b = {}", b);

    // Basic arithmetic
    let sum = a + b;
    let diff = a - b;
    let product = a * b;
    let quotient = a / b;

    println!("Sum: {}", sum);
    println!("Difference: {}", diff);
    println!("Product: {}", product);
    println!("Quotient: {}", quotient);

    // Loop example
    println!("Looping 1 to 5:");
    for i in 1..=5 {
        println!("i = {}", i);
    }

    // Simple function call
    let result = square(7);
    println!("Square of 7 is {}", result);
    //comment
}

fn square(n: i32) -> i32 {
    n * n
}
