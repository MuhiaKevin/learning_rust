use std::io;

fn main() {
    // let mut x = 5; // adding mut behind the variable tells the complier that that value of  variable can be changed in the future
    // println!("The value of x is: {}", x);
    // x = 6;
    // println!("The value of x is: {}", x);

    // // constants
    // const MAXPOINTS: u64 = 4899;

    // println!("Maximum points is: {}", MAXPOINTS);

    // // shadowing
    // let x = 5;

    // let x = x + 1;

    // let x = x * 2;

    // println!("The value of x is: {}", x);

    // let mut spaces = "     💀 ";
    // let spaces = spaces.len();

    // println!("There are  {} spaces", spaces);

    // DATA TYPES
    // 1. INTERGERS
    //TODO: read om Two's complement
    // let guess: u32 = "42".parse().expect("Not a Number");
    // println!("The value of x is: {}", guess);

    // let x = 567885678;
    // println!("{}", x);

    // // 2. CHARACTER TYPE

    // println!("{}", x % guess);

    // let c = 'z';
    // let z = 'ℤ';
    // let heart_eyed_cat = '🦀'; // Characters

    // println!("{},{},{}", c, z, heart_eyed_cat);

    // // 3. TUPPLE TYPE
    // //=> tupples have a fixed length and once declared they cannot grow or shrink in size and can accept various types
    // let tup = (500, 6.4, 1);
    // println!("This is tupple type {:?}", tup); // this is the way you display a tupple in rust
    // let (x, y, z) = tup; // this is like destructuring an object in javascript`

    // println!("{} {} {}", x, z, y);

    // // accessing a tuple element
    // println!("{}",tup.0); // accessing the first element

    // // 2. ARRAY TYPE
    // // array must have values that must be of the same type
    // // arrays are applicable when you know that you wont change anyting
    // let a = [12,23,35,46,56,7676,3434,343];
    // let months = ["January", "February", "March", "April", "May", "June", "July","August", "September", "October", "November", "December"];
    // // declaring an array with its type
    // let array1: [i32; 5] = [1,2,3,4,5] ;

    // println!("{:?} {:?}",months, array1);// accessing the first element
    // let first = a[0];
    // let second = a[1];

    // println!("This is an array type{:?}",a); // accessing the first element
    // println!("first {} second {}", first, second);

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {} is: {}",index, element)
}
