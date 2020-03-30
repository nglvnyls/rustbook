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
    println!("v mutable : {:?}", v);
    v.push(6);
    println!("v mutable : {:?}", v);
    v.push(7);
    println!("v mutable : {:?}", v);
    v.push(8);
    println!("v mutable : {:?}", v);

    //read vectors elements


    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2]; // vectors are indexed by number, starting at zero
    println!("The third element is {}", third);

    match v.get(2) { //using the get method with the index passed as an argument, which gives us an Option<&T>
        Some(third) => println!("The third element is {}", third),
        None => println!("There is NO third element."),
    }

    //tries to access an element at index 100 that is NOT in the vector
    
    //let does_not_exist = &v[100]; //will cause the program to panic because it references a nonexistent element. This method is best used when you want your program 
                                    //to crash if there’s an attempt to access an element past the end of the vector
    
    match v.get(100) { //When the get method is passed an index that is outside the vector, it returns None without panicking
        
        //it uses "match" to handle either Some(&element) or None result of get method
        Some(elem) => println!("The 100th element is {}", elem),
        None => println!("There is NO 100ht element."),
    }

    //ownership and borrowing rules
    //Recall the rule that states you can’t 
    //have mutable and immutable references in the same scope.
    /*
    we hold an immutable reference to the first element in a vector 
    and try to add an element to the end, which won’t work.

    let first = &v[0];

    v.push(6);

    This error is due to the way vectors work: adding a new element onto 
    the end of the vector might require allocating new memory and copying 
    the old elements to the new space, if there isn’t enough room to put 
    all the elements next to each other where the vector currently is.
    */

    // Iterating over the Values 

    let v = vec![100, 32, 57];
    for i in &v {
        println!("The element is: {}",i);
    }

    for (i,a) in v.iter().enumerate() {//with an index
        println!("The {} element has a value of: {}",i,a);
    }

    /*get values from a row in a spreadsheet in which some of the columns 
    in the row contain integers, some floating-point numbers, and some strings*/
    #[derive(Debug)]
    enum SpreadsheetCell { //define an enum whose variants will hold the different value types
        Int(i32),
        Float(f64),
        Text(String),
    }
    
    let row = vec![ // The `vec!` macro can be used to initialize a vector
        SpreadsheetCell::Int(3), //all the enum variants will be considered the same type: that of the enu
        SpreadsheetCell::Text(String::from("blue")), // type enum 
        SpreadsheetCell::Float(10.12), // type enum
    ];

    println!("row is {:?}",row);
    
    // Iterators can be collected into vectors. Immutables and mutables ones
    let mut collected_iterator: Vec<i32> = (0..10).collect();
    println!("Collected (0..10) into: {:?}", collected_iterator);

    // The `len` method yields the number of elements currently stored in a vector
    println!("Collected iterator length: {}", collected_iterator.len());

    // `pop` removes the last element from the vector and returns it
    println!("Pop last element: {:?}", collected_iterator.pop());   
    
    //`push` add an element at then end of vector vector_collection
    collected_iterator.push(2000);

    // `Vector`s can be easily iterated over
    println!("Contents of collected_iterator:");
    for x in collected_iterator.iter() {
        println!("> {}", x);
    }

    

    // Thanks to `iter_mut`, mutable `Vector`s can also be iterated
    // over in a way that allows modifying each value
    for x in collected_iterator.iter_mut() {
        *x *= 3;
    }
    println!("Updated vector: {:?}", collected_iterator);



} // <- v,v2 go out of scope and are freed here
