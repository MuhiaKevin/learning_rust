fn main() {
    // s is only accessbile between the brackets
    {
        let s = "Hello";
        println!("{}", s)
    }

    // String Type
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`

    // Ways variable and Data interact Mpve
    let x = 5;
    let y = x;
    // beacuse intergers are simple values whose size are known they are pushed to the stack
    println!("{} {}", x, y); // This will print `hello, world!`

    // unlike the the above example the string1 (s1) saves: pointer to memory location, length and capacity
    // The length is how much memory in bytes the contents the contents of the string is currently using
    // The capacity is the total amount of memory in bytes that the string has recieved from the allocator

    let s1 = String::from("Hello");
    let s2 = s1.clone();
    let s2 = s1;

    println!("{} {}", s1, s2); // This will print `hello, world!`

    // OWNERSHIP AND FUNCTIONS
    let s = String::from("Hello");

    takes_ownership(s);

    println!("{}",s); // this will bring an error because the data has been moved to takes_ownership
    let x = 5;
    makes_copy(x) // this is possible because intergers have a copy trait that allows their values to be copied

    // Returning values can also transfer ownership
    // Return Values and Scope
 
    let s1 = gives_ownership(); // ownership of String type is given by the function
    let s2 = String::from("hello"); // ownership belongs s2
    let s3 = takes_and_gives_back(s2); // ownership of data moves from s2 to takes_and_gives which is then given to s3
 
    println!("s1: {} s2: {} s3: {}", s1, s2, s3); // This will output an error because s2 does not anything
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_string: i32) {
    println!("{}", some_string);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello"); // some_string comes into scope

    some_string // some_string is returned and
                // moves out to the calling
                // function
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string // a_string is returned and moves out to the calling function
}
