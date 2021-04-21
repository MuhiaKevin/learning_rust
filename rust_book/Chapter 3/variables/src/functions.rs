fn main(){
    let x = 5;

    let y = {
        let x = 3;
        x + 1;
    };
    println!("The value of y is: {}", y);
}
fn five() -> i32 {
    let x: i32 = 5;
    x + 2
    // return 5
}

fn main(){
    let x = five();
    println!("The value of x is {}", x)
}

// fn main() {
//     // Functions
//     let mut number: u32 = 89;
//     say_number(number);
//     number = 16;
//     say_number(number);
//     multiple_parameters(12,56,18);
// }

// fn say_number(x: u32) {
//     println!("{}", x);
// }
// fn multiple_parameters(x: u32,y: u32, z: u32) {
//     println!("x :{} y: {} z {}", x,y,z);
// }

// fn main(){
//     // let x = 5;

//     let y = {
//         let x = 3;
//         x + 1
//     };
//     println!("The value of y is: {:?}", y);
// }\

// TODO: Functions return values

// this is how you return values in rust
