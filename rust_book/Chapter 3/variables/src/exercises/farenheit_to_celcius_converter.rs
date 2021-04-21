use std::io;

fn main() {
    temp_converter(-32.2)
}

fn temp_converter(_: f32) {
    let mut value = String::new();
    let mut choice = String::new();
    let mut answer: f32 = 0.0;
    
    println!("Welcome to temperature converter 🤒");
    println!("What do you want to 'f' for farenheit to celcius or 'c' for celcius to farenheit :");
    
    io::stdin()
    .read_line(&mut choice)
    .expect("Choose either 'f' or 'c' ");
    
    println!("Enter the value :");
    
    io::stdin()
    .read_line(&mut value)
    .expect("Choose either 'f' or 'c' ");
    
    let value: f32 = value.trim().parse().expect("Please type a number!");
    let choice = choice.trim();
    
    // Farenheit  to celcius => celcius = 5/9 * (farenheit - 32)
    if choice == "c" {
        answer = 9.0 / 5.0 * (value + 32.0);
    } else if choice == "f" {
        answer = 5.0 / 9.0 * (value - 32.0);
    }

    println!("The answer is {} choice : ", answer)
}

