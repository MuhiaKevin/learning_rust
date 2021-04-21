#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}


// Rectangle Struct
impl Rectangle {
    fn area(&self) -> u32{
        self.width * self.height
    }

    fn can_hold(&self, other_rectangle: &Rectangle) -> bool{
        self.width > other_rectangle.width && self.height > other_rectangle.height
    }

    // associated function
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

}




fn main(){
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("The area of the rectangle  with a of wdith {}  and height of {} is {}", rect1.width, rect1.height, rect1.area());
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // create  a square using an associated function
    let square = Rectangle::square(64);
    println!("Square dimensions are {:#?} and the area is {}",square, square.area() )
}



/* 
TODO: ASSOCIATED FUNCTIONS
    - these functions do not have the self parameter 
    - They are assciated with the struct as opposed to an instance of the struct when self parameter is used
    - are often used for constructors that will return a new instance of the struct
    
    -Syntax is follows Struct::associated_function_name()

*/

