fn main() {
    println!("STRUCTS");

    struct User {
        username: String, //we used the owned String type rather than the &str string slice type
                          //because we want instances of this struct to own all of its data and 
                          //for that data to be valid for as long as the entire struct is valid
        email: String,
        sign_in_count: u64,
        active: bool,
    };

    /*
    struct User {
        username: &str,
        email: &str,
        sign_in_count: u64,
        active: bool,
    }
    It’s possible for structs to store references to data (&str )owned by something else, 
    but to do so requires the use of lifetimes.
    The compiler will complain that it needs lifetime specifiers
    
    */


    let mut user1 = User { //assigning mut converts immutable user1 in mutable user1
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1 //struct update syntax(.l)
        /*
         instance user2 has a different value for email and username 
         but has the same values for the 
         active and sign_in_count fields from user1
        */
    };

    println! ("user1.email = {}",user1.email);

    //if user1 is mutable we can use dot notation
    user1.email = String::from("anotheremail@example.com");

    println! ("user1.email = {}",user1.email);

    //tuple struct 
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0); 
    //black and origin are diferent types
    //because they’re instances of different tuple structs



}


fn build_user(email: String, username: String) -> User {
    User {
        email, //field init shorthand syntax
        username,
        active: true,
        sign_in_count: 1,
    }
}
