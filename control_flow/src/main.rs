fn main() {
    println!("Branches- Control Flow");

    let mut number = 3;
    println!("number = {} ",number);

    if number < 5 { //All if expressions start with the keyword if, which is followed by a condition.
        println!("condition was true");
    } else { //Optionally, we can also include an else expression
        println!("condition was false");
    }

    number = 7;
    println!("number = {}",number);

    if number < 5 { //All if expressions start with the keyword if, which is followed by a condition.
        println!("condition was true");
    } else { //Optionally, we can also include an else expression
        println!("condition was false");
    }

    if number != 0 {
        println!("number was something other than zero");
    }

    number = 6;
    println!("number = {}",number);

    /*
     Rust only executes the block for the first true condition, 
     and once it finds one, it doesn’t even check the rest
    */
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    


}
