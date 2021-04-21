fn main() {
    let number = 3;
    if number < 5 {
        println!("Condition is true");
    } else {
        println!("Condition was false");
    }

    // tenary operators in rust
    let answer = if 3 > 5 { true } else { false };
    println!("It is {} 3 > 5", answer);

    // LOOPS

    // 1. Loops ==> this loop will run forever until you decide to "break"

    let mut count: u64 = 0;
    loop {
        if count == 12000 {
            break;
        } else {
            println!("Hello world again");
        }
        count = count + 1;
    }
    


    // A loop can return a value by using break inplace of return
    let mut count = 0;
    
    // While loop
    
    println!("{}", result)
   
    let mut number = 12000;
    
    while number != 0 {
        println!("{}", number);
        number -= number;
    }
    
    println!("LIFTOFF!!!");   let result = loop {
        count = count + 1;
        if count == 10 {
            // returning a value from a loop
            break count * 2;
        }
    };
 
 
    // looping in a list
 
    let numbers  = [12,45123,56167];

    for number in numbers.iter(){
        println!("Number in list is : {}", number)
    }


    for number in (1..4).rev(){
        println!("Number {} range", number)
    }
}
