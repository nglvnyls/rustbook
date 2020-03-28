fn main() {
    println!("Match Control");
    println!("");

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }
    
    fn value_in_cents(coin: Coin) -> u8 {
        /*
        values go through each pattern in a match, and at the first pattern the value “fits,” 
        the value falls into the associated code block to be used during execution
        */
        match coin { //match keyword followed by an expression, which in this case is the value coin
            
            Coin::Penny => {               //every time the method was called with a 
                println!("Lucky penny!");  //Coin::Penny but would still return the 
                1                          //last value of the block, 1:
            },
            Coin::Nickel => 5,//Each arm is separated from the next with a comma
            Coin::Dime => 10,//If that pattern doesn’t match the value, execution continues to the next arm
            Coin::Quarter => 25,
            //We can have as many arms as we need
        }
    }

    println!("100 pennies are equal to {} cents", 100*value_in_cents(Coin::Penny));
    println!(" 20 Nickels are equal to {} cents", 20*value_in_cents(Coin::Nickel));
    println!(" 10 Dime    are equal to {} cents", 10*value_in_cents(Coin::Dime));
    println!("  4 Quarter are equal to {} cents", 4*value_in_cents(Coin::Quarter));
    

    //Patterns that binds to values

    #[derive(Debug)] // so we can inspect the state in a minute
    enum UsState {
        Alabama,
        Alaska,
        // --snip--
    }

    enum Coin2 {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }  
    
    fn value_in_cents2(coin: Coin2) -> u8 {
        match coin {
            Coin2::Penny => 1,
            Coin2::Nickel => 5,
            Coin2::Dime => 10,
            Coin2::Quarter(state) => { //we add a variable called state to the pattern 
                                      //that matches values of the variant Coin::Quarter
                                      //When a Coin::Quarter matches, the state variable 
                                      //will bind to the value of that quarter’s state. 
                                      //Then we can use state in the code for that arm
                println!("State quarter from {:?}!", state); //We can then use the binding for state in the println! expression
                                                             //thus getting the inner state value out 
                                                             //of the Coin enum variant for Quarter.
                25 //value returned by this arm when matchs
            },
        }
    }

    println!("45 pennies are equal to {} cents", 45*value_in_cents(Coin2::Penny));
    println!(" 54 Nickels are equal to {} cents", 54*value_in_cents(Coin2::Nickel));
    println!(" 23 Dime    are equal to {} cents", 23*value_in_cents(Coin2::Dime));
    println!("  4 Quarter of Alaska are equal to {} cents", 4*value_in_cents(Coin2::Quarter(UsState::Alaska)));


    //Matching with Option<T>
    /*Remember enum is defined in standard library:
        enum Option<T> {
        None,
        Some(T),
}
    */

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None, //If there isn’t a value inside, the function should return 
                          //the None value and not attempt to perform any operations.
            Some(i) => Some(i + 1), //if there’s a value inside, adds 1 to that value
        }
    }
    
    let five = Some(5);
    let six = plus_one(five); //the variable x in the body of plus_one will have the value Some(5)
    let none = plus_one(None);

    /*The _Placeholder
    when we don’t want to list all possible values
    */
    let some_u8_value = 0u8;
    match some_u8_value { //If we only care about the values 1, 3, 5, and 7
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (), // we don’t want to have to list out 0, 2, 4, 6, 8, 9 all the way up to 255
                 // the _ will match all the possible cases that aren’t specified before it
                 // The () is just the unit value, so nothing will happen in the _ case
    }

}
