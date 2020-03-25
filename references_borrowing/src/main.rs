fn main() {
    println!("REFERENCES & BORROWING");
    println!("");

    let s1 = String::from("hello");

    /*you would define and use a calculate_length function 
    that has a reference to an object as a parameter 
    instead of taking ownership of the value*/

    let len = calculate_length(&s1); //we pass &s1 into calculate_length

    println!("The length of '{}' is {}.", s1, len);

    //mutable references
    let mut s = String::from("hello"); //First, we had to change s to be mut
    
    change(&mut s); //Then we had to create a mutable reference with &mut s 

    //combining mutable and immutable references

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2); // r1 and r2 are no longer used after this point
    //The scopes of the immutable references r1 and r2 end after the println! 

    let r3 = &mut s; // s is introduced as a mutable. 
                     // It's no problem, because these scopes don’t overlap
    println!("{}", r3);



}

fn calculate_length(s: &String) -> usize { //we take &String rather than String.
                                           //s is a reference to a String 
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
// it refers to, nothing happens.


fn change(some_string: &mut String) { //accept a mutable reference
    /*
    mutable references have one big restriction: 
    you can have only one mutable reference to a 
    particular piece of data in a particular scope
    */
    some_string.push_str(", world");
}