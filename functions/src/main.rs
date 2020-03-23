fn main() {
    println!("Functions");
    println!("");

    another_function(5, 6);

    let r = 5;
    /*
    {} is a block that, in this case, evaluates to 4. 
    That value gets bound to t as part of the let statement
    */
    let t = {
        let r = 3;
        r + 1 //Note the x + 1 line without a semicolon at the end, which is unlike most of the lines you’ve seen so far. Expressions do not include ending semicolons
    };

    println!("The value of t is: {}", t);


    let x_five = five(); //Calling a function is an expression

    println!("The value of x_five is: {}", x_five);

    let x_plus_one = plus_one(5);

    println!("The value of x_plus_one is: {}", x_plus_one);

}
/*
Rust doesn’t care where you define your functions, 
only that they’re defined somewhere.
The lines execute in the order in which they appear 
in the main function.
*/
fn another_function(x: i32, y: i32) {
    println!("Another function.");
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

/*
Function that returns a value
*/
fn five() -> i32 { //return type is specified too, as -> i32
    5 //with no semicolon because it’s an expression
}

fn plus_one(x: i32) -> i32 {
    x + 1 //with no semicolon because it’s an expression
    // x + 1;  changing it from an expression to a statement, we’ll get an error
}

