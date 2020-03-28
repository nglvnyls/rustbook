fn main() {
    println!("If LET Control");
    println!("");

    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"), //We want to do something with the Some(3) match 
                                      //but do nothing with any other Some<u8> value 
                                      //or the None value
        _ => (), //added any other value 
                 //to satisfy the match expression.
                 //is a lot of boilerplate code to add
    }

    //Instead, we could write this in a shorter way using if let

    
}
