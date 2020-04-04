use std::io;
//use std::cmp::Ordering;
use rand::Rng;
use std::net::IpAddr;
fn main() {
    println!("93 Panic or Not ro Panic");
    println!("");

    //it’s perfectly acceptable to call unwrap, 
    //because inspecting the code it would never have an Err.alloc

    

    let home: IpAddr = "127.0.0.1".parse().unwrap();

    /*
    We’re creating an IpAddr instance by parsing a hardcoded string. 
    We can see that 127.0.0.1 is a valid IP address, so it’s acceptable 
    to use unwrap here. However, having a hardcoded, valid string 
    doesn’t change the return type of the parse method: 
    we still get a Result value, and the compiler will still make us 
    handle the Result as if the Err variant is a possibility because 
    the compiler isn’t smart enough to see that this string is always 
    a valid IP address.

    If the IP address string came from a user rather than being hardcoded 
    into the program and therefore did have a possibility of failure, 
    we’d definitely want to handle the Result in a more robust way instead.
    */

    //Creating Custom Types for Validation

    println!("Guess the number!");

    //let secret_number = rand::thread_rng().gen_range(1, 101);


    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
    
        let guess: i32 = match guess.trim().parse() { //i32 only accepts positive integers
            Ok(num) => num,
            Err(_) => continue,
        };
    
        if guess < 1 || guess > 100 { //here we are testing if guess number is smaller than 100 and greater than 0
            println!("The secret number will be between 1 and 100.");
            continue;
        }    

        /*However, this is not an ideal solution: if it was absolutely 
        critical that the program only operated on values between 1 and 100, 
        and it had many functions with this requirement, having a check 
        like this in every function would be tedious 
        (and might impact performance).*/

        pub struct Guess { //we  make a new type
                        //we define a struct named Guess that has a field 
                        //named value that holds an i32
            value: i32, //This is where the number will be stored
        }

        impl Guess { //we implement an associated function named new on Guess 
                    //that creates instances of Guess values
            pub fn new(value: i32) -> Guess { //function is defined to have one parameter 
                                            //named value of type i32 and to return a Guess
                if value < 1 || value > 100 { //tests value to make sure it’s between 1 and 100
                    panic!("Guess value must be between 1 and 100, got {}.", value);
                    /*
                    If value doesn’t pass this test, we make a panic! call, 
                    which will alert the programmer who is writing the calling code that 
                    they have a bug they need to fix, because creating a Guess with a value 
                    outside this range would violate the contract that 
                    Guess::new is relying on
                    */
                }
        
                Guess { //If value does pass the test, we create a new Guess with its 
                        //value field set to the value parameter and return the Guess.
                    value
                }
            }
        
            pub fn value(&self) -> i32 {
                /* we implement a method named value that borrows self, doesn’t have any other parameters, 
                and returns an i32. This kind of method is sometimes called a getter, because its purpose 
                is to get some data from its fields and return it. 
                This public method is necessary because the value field of the Guess struct is private. 
                It’s important that the value field be private so code using the Guess struct is not allowed
                to set value directly: code outside the module must use the Guess::new function to create 
                an instance of Guess, thereby ensuring there’s no way for a Guess to have a value that 
                hasn’t been checked by the conditions in the Guess::new function.
                */
                self.value
            }
        }

        /*That way, it’s safe for functions to use the new type in their signatures 
        and confidently use the values they receive. A function that has a parameter or returns only numbers 
        between 1 and 100 could then declare in its signature that it takes or returns a Guess rather than an i32 
        and wouldn’t need to do any additional checks in its body.
        */



        
    }




}
