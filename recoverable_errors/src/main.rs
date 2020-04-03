use std::fs::File;
use std::io::ErrorKind;

#![allow(unused_variables)]
fn main() {
    println!("92 Recoverable Errors");
    println!("");

    enum Result<T, E> { //The T and E are generic type parameters:
        Ok(T), //T represents the type of the value that 
                //will be returned in a success case within the Ok variant
        Err(E), //E represents the type of the error that 
                //will be returned in a failure case within the Err variant
    }

    //try to open a file

    let f = File::open("hello.txt");

    //We need to add  code to take different actions depending 
    //on the value File::open returns.

    // one way to handle the Result using a basic tool, 
    //the match expression.alloc

    let f = match f {
        Ok(file) => file, //return the inner file value out of the Ok variant, 
                        //and we then assign that file handle value to the variable f
        Err(error) => { // the value in f will be an instance of Err that contains 
                        //more information about the kind of error that happened.
            panic!("Problem opening the file: {:?}", error)
        },

        /*Note that, like the Option enum, the Result enum and its variants 
        have been brought into scope by the prelude, so we don’t need to specify 
        Result:: before the Ok and Err variants in the match arms.*/

    };

    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() { //The type of the value that File::open 
                                            //returns inside the Err variant is io::Error
                                            //This struct has a method kind that we can call 
                                            //to get an io::ErrorKind value
                                            //The enum io::ErrorKind is provided by the standard 
                                            //library and has variants representing the different 
                                            //kinds of errors that might result from an io operation. 
                                            //The variant we want to use is ErrorKind::NotFound, 
                                            //which indicates the file we’re trying to open doesn’t 
                                            //exist yet. So we match on f, but we also have an 
                                            //inner match on error.kind()
            ErrorKind::NotFound => match File::create("hello.txt") { //The condition we want to check 
                                            //in the inner match is whether the value returned by 
                                            //error.kind() is the NotFound variant of the ErrorKind enum
                                            //If it is, we try to create the file with File::create
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e), //However, because File::create 
                                            //could also fail, we need a second arm in the inner match expression
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };

    //That’s a lot of match! The match expression is very useful but also very much a primitive.
    //In Chapter 13, you’ll learn about closures; the Result<T, E> type has many methods that accept a closure 
    //and are implemented using match expressions. Using those methods will make your code more concise


    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });


    //Although this code has the same behavior as Listing 9-5, it doesn’t contain 
    //any match expressions and is cleaner to read


  
}
