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

    if let Some(3) = some_u8_value {
        println!("three");
    }

    /*
    Using if let means less typing, less indentation, and less 
    boilerplate code. However, you lose the exhaustive checking that match enforces
    */

    //We can include an else with an if let.

    #[derive(Debug)] // so we can inspect the state in a minute
    enum UsState {
        Alabama,
        Alaska,
        // --snip--
    }
    
    enum Coin {
        Penny,
        Quarter(UsState),
    }

    let mut count = 0;
    println!("count: {}", count);

    let coin = Coin::Quarter(UsState::Alaska);
    
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    };
    println!("count: {}", count);

    let mut count = 0;
    println!("count: {}", count);

    let coin = Coin::Quarter(UsState::Alabama);
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    };
    println!("count: {}", count);

    /*
    If you have a situation in which your program has logic that is 
    too verbose to express using a match, remember that if let is in 
    your Rust toolbox as well.
    */

    enum Foo {
        Bar,
        Baz,
        Qux(u32)
    }

    // Create example variables
    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);
    
    // Variable a matches Foo::Bar
    if let Foo::Bar = a {
        println!("a is foobar");
    }
    
    // Variable b does not match Foo::Bar
    // So this will print nothing
    if let Foo::Bar = b {
        println!("b is foobar");
    } else {
        println!("b is NOT a foobar");
    };
    
    // Variable c matches Foo::Qux which has a value
    // Similar to Some() in the previous example
    if let Foo::Qux(value) = c {
        println!("c is {}", value);
    }

    // Binding also works with `if let`
    if let Foo::Qux(value @ 100) = c {
        println!("c is one hundred");
    }





}
