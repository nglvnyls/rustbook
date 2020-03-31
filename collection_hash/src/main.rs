use std::collections::HashMap;
//we need to first use the HashMap from the 
//collections portion of the standard library

fn main() {
    println!("Collections maps hash");
    println!("");

    //create an empty hash map with new
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("scores = {:?}", scores);

    let teams  = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    /*The type annotation HashMap<_, _> is needed here because it’s possible 
    to collect into many different data structures and 
    Rust doesn’t know which you want unless you specify
    For the parameters for the key and value types, 
    however, we use underscores, and Rust can infer the types 
    that the hash map contains based on the types of the data in the vectors.*/

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!



}
