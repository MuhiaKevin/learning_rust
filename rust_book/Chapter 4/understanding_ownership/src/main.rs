// TODO: The Slice Type

fn main() {
    let mut s = String::from("Muhia Kahiga");
    let word = first_word(&s);
    s.clear();
    println!{"{}", word}
}

// // reasons why the function returns a usize is because, usize depends on the size of the variable the usize is pointing to
// // on x86_x64 usize= u64
// // using usize than let's say using u32, will guarantee

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); // convert string to array

    // iter() returns each element in   collection
    // .enumerate() wraps the result of  iter and returns each element as part of a tuple instead

    // in (i, &item) i = index and &item = a reference to the element
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// TODO: STRING SLICES

fn main(){
    let s = String::from("hello world");
    first_word(&s);
    let s = String::from("hello world 🍕");
    let bytes = s.as_bytes();
    println!("{:?}", bytes);
}

fn first_word(s: &String) -> &str{
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let s = String::from("🍕💀💀💀💀");


    let slice = &s[..8];
    println!("{:?} {}", slice, s.len());
}
