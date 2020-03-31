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

    //Another way of constructing a hash map is by using the collect method on a vector of tuples,
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
    //We aren’t able to use the variables field_name and field_value after 
    //they’ve been moved into the hash map with the call to insert.

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    println!("score of blue team is: {:?}", score);
    //The result is wrapped in Some because get returns an Option<&V>;

    let score_none = scores.get(&String::from("noteam"));
    println!("score of blue team is: {:?}", score_none);
    //if there’s no value for that key in the hash map, get will return None

    //iterate over each key/value pair in a hash map
    
    println!("");
    println!("iterate over each key/value pair in a hash map");
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    //updating a hash map
    println!("");

    //Overwriting a value

    let mut scores2 = HashMap::new();

    scores2.insert(String::from("Blue"), 10);
    scores2.insert(String::from("Blue"), 25);

    println!("{:?}", scores2);
    //if we insert a key and a value into a hash map and then insert that same key 
    //with a different value, the value associated with that key will be replaced

    //Only Inserting a Value If the Key Has No Value

    scores2.insert(String::from("Blue"), 100);

    scores2.entry(String::from("Yellow")).or_insert(500);
    //Hash maps have a special API for this called entry that takes the key you want to check as a parameter.

    scores2.entry(String::from("Blue")).or_insert(500);
    //The or_insert method on Entry is defined to return a mutable reference 
    //to the value for the corresponding Entry key if that key exists, and if not, 
    //inserts the parameter as the new value for this key and returns a mutable reference to the new value

    println!("{:?}", scores2);

    //Updating a Value Based on the Old Value

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() { //Splits a string slice by whitespace.
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
