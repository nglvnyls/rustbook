fn main() {
    println!("Loops");
    println!("");

    /*
    this code executes  over and over continuously 
    until we stop the program manually.i32

    loop {
        println!("again!");
    }

    */

    //loop

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; //After the loop, we use a semicolon to end the statement that assigns the value to result
        }
    };

    println!("The result is {}", result);

    //loop whit a conditional break
    counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break 22 //After the loop, we don't use a semicolon to end the expression that assigns the value to result
        }
    };

    println!("The result is {}", result);

    //while 
    /*
    While a condition holds true, the code runs; 
    otherwise, it exits the loop
    */
    let mut number = 3;

    while number != 0 {
        println!("while loop. number minus 1 = {}", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");

    //while construct to loop over the elements of a collection
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("another while loop. the value is: {}", a[index]);

        index += 1;
    }

    //for loop as a more concise alternative
 
    for element in a.iter() {
        println!("for loop. the value is: {}", element);
    }

    //using range instead
    
    /*
    Here’s what the countdown would look like 
    using a for loop and another method we’ve 
    not yet talked about, rev, to reverse the range:
    */


    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");

    
}
