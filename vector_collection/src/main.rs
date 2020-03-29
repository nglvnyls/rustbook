fn main() {
    println!("Vector collection");
    println!("");

    let v: Vec<i32> = Vec::new(); //create a new, empty vector
                                // Note that we added a type annotation here
    println!("v : {:?}", v);
    /*
    It’s more common to create a Vec<T> that has initial values, 
    and Rust provides the vec! macro for convenience.
    The macro will create a new vector that holds the values you give it.
    */

    let v2 = vec![1, 2, 3]; //Because we’ve given initial i32 values, Rust can infer that the type of v is Vec<i32>, 
                            //and the type annotation isn’t necessary

    println!("v2 : {:?}", v2);

    let mut v = Vec::new(); //if we want to be able to change its value, 
                            //we need to make it mutable using the mut keyword

    v.push(5); //The numbers we place inside are all of type i32
    v.push(6);
    v.push(7);
    v.push(8);


} // <- v,v2 go out of scope and are freed here
