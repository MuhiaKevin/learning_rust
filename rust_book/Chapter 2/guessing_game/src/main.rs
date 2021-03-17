use rand::Rng; // will be used to get a random number
use std::cmp::Ordering;
use std::io; // import 'io' library for obtaining user input and print the result

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101); // create a random number
    // this is an infinite loop much like a while(true), that will break once the guessed number is equal to the secret number
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // shadowing of a varible allows the variable to overwritten as a conversion from string to u32
        // this allows us to reuse the variable rather than forcing us to create two unique variables
        // the variable is now u32 type
        let guess: u32 = match guess.trim().parse() {
            // guess.trim().parse() returns a Result Type Enum that has variants Ok and Err. 
            // Adding a match will ensure that if an non-number is entered the program will not crash
            Ok(num) => num,
            Err(_) => continue,
        };
 
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break; // program will quit once the guess is correct
            }
        }
    }
}
