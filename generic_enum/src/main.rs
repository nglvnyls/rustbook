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








 
}
