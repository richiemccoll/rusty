use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number");

    // rand::thread_rng gives us a random number generator
    // that is local to the current thread of execution
    // and is seeded by the operating system.
    let secret_number = rand::thread_rng().gen_range(1, 101);
    
    // The loop keyword creates an infinite loop.
    loop {
        println!("Please enter your guess");

        // Note the :: syntax indicates that this is an associated function.
        // It is implemented on the type, rather than on a particular instance.
        // In other language this is called a static method.
        // We have to explictly tell Rust that this is a mutable variable,
        // All variables in Rust are immutable by default.
        let mut guess = String::new();

        // Note: the & syntax indicates that the argument is a reference,
        // This gives you a way to allow multiple parts of your code
        // to access one piece of data without needing to copy that data into
        // memory multiple times.

        // read_line returns a Result type, which is an enum,
        // the enum variants are 'Ok' or 'Err'.
        // Rust will throw a warning if expect is not called,
        // indicating that you have not handled a possible error.
        io::stdin().read_line(&mut guess)
                .expect("Failed to read line");

        // A few number types can have a value between 1 and 100:
        // i32 - a 32-bit number
        // u32 - an unsigned 32-bit number
        // i64 - a 64 bit number
        // There are others. Rust defaults to i32.
        
        // Rust allows to "Shadow" the previous value of the guess variable.
        // This is often used in situations where you want to convert a value
        // from one type to another type.

        // Using a match expression instead of .expect is generally
        // how to move from crashing on an error to handling the error.
        // The _ in the Err indicates a catch all value.
        // match all Err values, no matter what information they have inside them.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        // The {} syntax indicates that this is a placeholder.
        print!("You guessed: {}", guess);

        // cmp compares two values and can be called on anything that can be compared.
        // It returns a variant of the Ordering enum.
        // The match expression is made up of arms. This is how pattern matching is done in Rust.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!(" - Too small!"),
            Ordering::Greater => println!(" - Too big!"),
            Ordering::Equal => {
                println!(" - You win!");
                break;
            }
        }
    
    }
}
