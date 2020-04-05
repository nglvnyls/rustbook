
fn main() {
    println!("Generic in enum");
    println!("");

    /*Standard library provides  the Option<T> enum 
    By using the Option<T> enum, we can express the 
    abstract concept of having an optional value, 
    and because Option<T> is generic, we can use this 
    abstraction no matter what the type of the optional 
    value is.
    */
    #[derive(Debug)]
    enum Option<T> {//enum that is generic over type T
        Some(T), //variants: Some, which holds one value of type T,
        None, //variant: None, that doesn’t hold any value
    }


    //Enums can use multiple generic types as well.
    #[derive(Debug)]
    enum Result<T, E> { //enum is generic over two types, T and E, 
        Ok(T), //variants: Ok, which holds a value of type T
        Err(E), //variant Err, which holds a value of type E
    }

    /* This definition makes it convenient to use the Result 
    enum anywhere we have an operation that might succeed 
    (return a value of some type T) or fail (return an error 
    of some type E) */


    let optional = Some(7);// Make `optional` of type `Option<i32>`
    println!("Optional: {:?} ", optional);

    let optional = Some("abracadabra");// Make `optional` of type `Option<i32>`
    println!("Optional: {:?} ", optional);

    /*Monomorphization is the process of turning generic code 
    into specific code by filling in the concrete types that are 
    used when compiled.
    */
    let integer = Some(5); //the compiler reads the values that have been 
                            //used in Option<T> instances and identifies 
                            //two kinds of Option<T>: one is i32
    let float = Some(5.0); //the other is f64

    //it expands the generic definition of Option<T> into 
    //Option_i32 and Option_f64, thereby replacing the generic definition 
    //with the specific ones

    /*
    enum Option_i32 {
        Some(i32),
        None,
    }

    enum Option_f64 {
        Some(f64),
        None,
    }
    */

    





}