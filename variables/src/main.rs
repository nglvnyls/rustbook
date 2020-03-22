fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    
    /*
    Constants
    Constants aren’t just immutable by default
    they’re always immutable.
    The type of the value must be annotated
    constants may be set only to a constant expression, 
    not the result of a function call or any other value 
    that could only be computed at runtime
    Rust’s naming convention for constants is to use all uppercase 
    with underscores between words, and underscores can be inserted 
    in numeric literals to improve readability
    */
    const MAX_POINTS: u32 = 100_000;

    /*
    Shadowing
    the first variable is shadowed by the second, 
    which means that the second variable’s value 
    is what appears when the variable is used
    Shadowing is different from marking a variable as mut, 
    because we’ll get a compile-time error 
    if we accidentally try to reassign to this variable 
    without using the let keyword. By using let, we can perform 
    a few transformations on a value but have the variable be 
    immutable after those transformations have been completed.
    We’re effectively creating a new variable when we use 
    the let keyword again, we can change the type of the value 
    but reuse the same name.
    */
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x);

    /*
    say our program asks a user to show how many spaces they want 
    between some text by inputting space characters, but we really 
    want to store that input as a number.
    This construct is allowed because the first spaces variable is 
    a string type and the second spaces variable, which is a 
    brand-new variable that happens to have the same name as 
    the first one, is a number type
    */
    let spaces = "   ";
    let spaces = spaces.len();
    
}
