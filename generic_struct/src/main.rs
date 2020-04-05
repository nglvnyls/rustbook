/*because we’ve used only one generic type to define Point<T>, 
this definition says that the Point<T> struct is generic over 
some type T, and the fields x and y are both that same type, 
whatever that type may be.*/
#[derive(Debug)]
struct Point<T> { //First, we declare the name of the type 
                //parameter inside angle brackets just after 
                //the name of the struct
    x: T,//Then we can use the generic type in the struct definition 
        //where we would otherwise specify concrete data types.
    y: T,
}

//To define a Point struct where x and y are both generics but could 
//have different types, we can use multiple generic type parameters.
#[derive(Debug)]
struct Point2<T, U> { //let two different types
    x: T,
    y: U,
}

//When you need lots of generic types in your code, 
//it could indicate that your code needs restructuring 
//into smaller pieces.


fn main() {
    println!("Generic in struct");
    println!("");

    let integer = Point { x: 5, y: 10 };//type is i32. Both types has to be the same.
    println!("integer point: {:?} ", integer);
    let float = Point { x: 1.0, y: 4.0 };//type is f64. Both types has to be the same.
    //let wont_work = Point { x: 5, y: 4.0 }; because one type is different from the other.alloc
    println!("float point: {:?} ", float);
    println!("");

    let both_integer = Point2 { x: 5, y: 10 }; //could be the same or different type 
                                            // by Point2 definition
    println!("both_integer point: {:?} ", both_integer);
    let both_float = Point2 { x: 1.0, y: 4.0 };
    println!("both_float point: {:?} ", both_float);
    let integer_and_float = Point2 { x: 5, y: 4.0 };
    println!("integer_and_float point: {:?} ", integer_and_float);
    println!("");
}
