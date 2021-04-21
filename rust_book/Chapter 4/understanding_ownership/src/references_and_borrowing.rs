
// passing references to data types like String type does not change ownership of the data and also allows function to get access to data for the variables

fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1); // send a reference of the variable s1

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// TODO: Mutable references => references that can change
// You can only have only one mutable reference to a particular piece of data in a particular scope
fn main() {
    // set the variable as mutable so that it can be changed
    let mut s1 = String::from("Hello");
    {
        let r1 = &mut s1;
        r1.push_str(", this is a scope that has been added");
    }

    println!("s1: {}", s1);
    {
        let r1 = &mut s1;
        r1.push_str(", and second scope added");
    }
    println!("s1: {}", s1);
    // send a mutable reference of the variable
    change(&mut s1);

    println!("{}", s1) // will show that the variable has changed
}

// fn change(s: &mut String) {
//     s.push_str(", String has been change by the function"); // change the value of the variable
// }

// TODO: Dangling references
// Dangling reference is a pointer that references a  location in memory that may have been given to someone else

fn main() {
    // let reference_to_nothing = dangle(); // the compiler will complain
    let reference_to_nothing = dangle_solution();

    println!("{}", reference_to_nothing)
}

fn dangle() -> &String {
    // returns a reference to a String
    let s = String::from("hello"); // create a new String

    &s // return a reference to the String, s
} // after this closing bracket all the data are out of scope so all of the are dropped
  Solution is to change ownership of the String rather than send a reference  to the string, saving it from it being dropped

fn dangle_solution() -> String {
    // returns a reference to a String
    let s = String::from("hello i am a dangling reference"); // create a new String

    s // return a reference to the String, s
} // after this closing bracket all the data are out of scope so all of the are dropped