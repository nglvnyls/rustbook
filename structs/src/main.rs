fn main() {
    println!("STRUCTS");

    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    };

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

}


fn build_user(email: String, username: String) -> User {
    User {
        email, //field init shorthand syntax
        username,
        active: true,
        sign_in_count: 1,
    }
}
