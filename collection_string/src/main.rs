fn main() {
    println!("Collection String");
    println!("");

    let s = String::new(); //with the new function
    println!("s = {}, created with function String::new()", s);

    let data = "initial contents";  
    let s =               data.to_string(); //with the to_string method
    println!("s = {}, created with data.to_string() method", s);
    let s = "initial contents".to_string(); //with the to_string method on a literal directly
    println!("s = {}, created with data.to_string() method on a literal directly", s);

    let s = String::from("initial contents"); // with function String::from
    println!("s = {}, created with with function String::from()", s);

    //Remember that strings are UTF-8 encoded

    let hello = String::from("السلام عليكم");
    println!("{}", hello);
    let hello = String::from("Dobrý den");
    println!("{}", hello);
    let hello = String::from("Alo");
    println!("{}", hello);
    let hello = String::from("שָׁלוֹם");
    println!("{}", hello);
    let hello = String::from("नमस्ते");
    println!("{}", hello);
    let hello = String::from("こんにちは");
    println!("{}", hello);
    let hello = String::from("안녕하세요");
    println!("{}", hello);
    let hello = String::from("你好");
    println!("{}", hello);
    let hello = String::from("Olá");
    println!("{}", hello);
    let hello = String::from("Здравствуйте");
    println!("{}", hello);
    let hello = String::from("Hola");
    println!("{}", hello);

    //Appending to a String with push_str and push

    let mut s = String::from("foo");
    s.push_str("bar"); //takes a string slice because don't necessarily 
                    //want to take ownership of the parameter
    
    let s1 = String::from("foo");
    let s2 = "bar";
    s.push_str(s2); 
    println!("s1 is {}",s1);
    println!("s2 is {}",s2);//If the push_str method took ownership of s2, 
                            //we wouldn’t be able to print its value on the last line

    //Concatenation with the + Operator                        

    let s1b = String::from("Hello, ");
    let s2b = String::from("world!");
    let s3b = s1b + &s2b; // note s1b has been moved here and can no longer be used
                        //the reason we used a reference to s2 has to do with the 
                        //signature of the method that gets called when we use the + operator
    println!("s3b is {}",s3b);

    //concatenate multiple strings
 
    let s1c = String::from("tic");
    let s2c = String::from("tac");
    let s3c = String::from("toe");

    let s4 = format!("{}-{}-{}", s1c, s2c, s3c);
    //The format! macro works in the same way as println!, 
    //but instead of printing the output to the screen, 
    //it returns a String with the contents
    //format! is much easier to read and doesn’t take ownership of any of its parameters.

    let sc = s1c + "-" + &s2c + "-" + &s3c;
    
    println!("sc is {}",sc); 
    println!("s4 is {}",s4);

    //Slicing Strings

    let hello = "Здравствуйте";
    println!("{}",hello);

    let s5 = &hello[0..4];
    println!("the first 4 bytes of the last string is {}",s5);
    
    /*
    What would happen if we used &hello[0..1]? 
    The answer: Rust would panic at runtime in the same way 
    as if an invalid index were accessed in a vector:
    */

    //Methods for Iterating Over Strings

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    //The bytes method returns each raw byte, which might be appropriate for your domain:

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }

    
    


   

}
