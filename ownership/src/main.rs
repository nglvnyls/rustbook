fn main() {
    println!("Ownership");

    // s is not valid here, it’s not yet declared
    let _s = "hello"; // s is valid from this point forward

    //can create a String 
    //from a string literal using the from function
    let _t = String::from("hello"); // this is String type not String literal

    let mut u = String::from("hello"); // can be mutated

    u.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", u); // This will print `hello, world!`


    let x = 5; //bind the value 5 to x
    let y = x; //make a copy of the value in x and bind it to y

    let s1 = String::from("hello");
    /*A String is made up of three parts: 
       -a pointer to the memory that holds the contents of the string, 
       -a length, 
       -and a capacity. 
    This group of data is stored on the stack. 
    the heap holds the contents.
    */
    let s2 = s1; //s1 it's invalidated
    /* When we assign s1 to s2, the String data is copied, 
    meaning we copy the pointer, the length, and the capacity 
    that are on the stack. 
    We do not copy the data on the heap that the pointer refers to
    Rust invalidates the first variable, an instead of 
    being called a shallow copy, it’s known as a move
    */
    println!("{}, world!", s2);

    let s1 = String::from("hello");
    let s2 = s1.clone();
    /* clone makes a deeply copy the heap data of the String,
    not just the stack data. 
    */

    println!("s1 = {}, s2 = {}", s1, s2);

    let x = 5;
    let y = x;
    /*Types such as integers that have a known size at 
    compile time are stored entirely on the stack, 
    so copies of the actual values are quick to make. 
    That means there’s no reason we would want to prevent x 
    from being valid after we create the variable y. 
    In other words, there’s no difference between deep 
    and shallow copying here, so calling clone wouldn’t do 
    anything different from the usual shallow copying 
    and we can leave it out.    
    */
    println!("x = {}, y = {}", x, y);

    let sm = String::from("hello");  // sm comes into scope

    takes_ownership(sm);             // sm's value moves into the function...
                                    // ... and so is no longer valid here
    //as a string variable, it cannot be used after takes_ownership function
    let xm = 5;                      // xm comes into scope

    makes_copy(xm);                  // xm would move into the function,
                                    // but i32 is Copy, so it’s okay to still
                                    // use x afterward
    println!("after makes_copy xm untill exist xm = {}", xm);
  
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);
                                    
    println!("The length of '{}' is {}.", s2, len);




} // this scope is now over, and s,t,u,x,y,s2 
  // is no longer valid.
  // Remember that s1 was moved to s2

  fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("inside takes_ownership, prints: {}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("inside makes_copy, prints:  {}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.  

fn gives_ownership() -> String {             // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("hello"); // some_string comes into scope

    some_string // some_string is returned and
                // moves out to the calling function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope

    a_string  // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}