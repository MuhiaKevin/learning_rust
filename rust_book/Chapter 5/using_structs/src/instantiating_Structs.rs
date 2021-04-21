// fn main() {
//     let width1 = 30;
//     let height1 = 50;
    
//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(width1, height1)
//     );
// }

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// TODO: Refactor 1 using tupples

// fn main () {
//     let rect1 = (30, 50);
//     println!("The area of the rectangle is {}", area(rect1));
// }

// fn area (dimensions: (u32, u32)) -> u32 {
//    dimensions.0 * dimensions.1
// }


// TODO: Refactor 2 using structs to group releated data and reuse them the data

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main(){
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("The area of the rectangle is {}", area(&rect1));
    println!("The area of the rectangle is {:#?}",rect1 );
}

fn area(rectangle: &Rectangle) -> u32{
    rectangle.width * rectangle.height
}