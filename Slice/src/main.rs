fn main() {
    println!("Slice");

    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    //let word = first_word_rewrited(&s); // Using the slice version of 
                                          //  first_word_rewrited will throw a compile-time error:

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. 
    // word is now totally invalid!
    // Managing these indices is even more brittle 
    // if we write a second_word function

    println!("the first word is: {}", word);
    
    //Using String Slices as parameters, instead of a reference 
    //to a String makes our API more general and useful without 
    //losing any functionality

    let my_string = String::from("hello world my_string");

    // first_word_rewrited works on slices of `String`s
    let word = first_word_rewrited(&my_string[..]); //If we have a String, 
                                           //we can pass a slice of the entire String

    let my_string_literal = "hello world my_string_literal"; //The type  here is &str: 
                                            //it’s a slice pointing to 
                                            //that specific point of the binary 

    // first_word works on slices of string literals
    let word = first_word_rewrited(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let another_word = first_word_rewrited(my_string_literal);
    
}

/*
write a function that takes a string and returns 
the first word it finds in that string. If the 
function doesn’t find a space in the string, 
the whole string must be one word, so the entire string 
should be returned.
*/

fn first_word(s: &String) -> usize { //has a reference to String as a parameter. 
                                     //We don't want ownership, so this is fine.   
                                     //But what should we return? 
                                     //We don’t really have a way to talk about part of a string. 
                                     //However, we could return the index of the end of the word                    
    
    println!("s of first word: {}",s);

    let bytes = s.as_bytes(); //Because we need to go through the String element by element 
                              //and check whether a value is a space, we’ll convert our String 
                              //to an array of bytes using the as_bytes method
    println!("bytes of s: {:?}",bytes);

    for (i, &item) in bytes.iter().enumerate() { //we create an iterator over the array 
                                                 //of bytes using the iter method:
                                                 //iter is a method that returns each element in a collection 
                                                 //and that enumerate wraps the result of iter and 
                                                 //returns each element as part of a tuple instead
        
        println!("tuples create by bytes.iter().enumerate(): ({}, {})",i,item);

        if item == b' ' { //we search for the byte that represents the space 
                          //by using the byte literal syntax

            println!("It finds a space (item={}) and return the position= {}",item,i);

            return i; //If it finds a space, it returns the position.
        }
        
    }

    s.len() //if it doesn't find a space it returns the length ot the string

    /*
    We’re returning a usize on its own, but it’s only a meaningful 
    number in the context of the &String.    
    In other words, because it’s a separate value from the String, 
    there’s no guarantee that it will still be valid in the future.
    Consider the following program
    */
}

/*
    let’s rewrite first_word to return a slice. 
    we called first_word_rewrited
*/

fn first_word_rewrited(s: &str) -> &str { //&str signifies “string slice” 
                                          //it allows us to use the same 
                                          //function on both &String values and &str values.   
    println!("s of first word rewrite with slices: {}",s);
    
    let bytes = s.as_bytes();

    println!("bytes of s: {:?}",bytes);

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {

            println!("It finds a space (item={}) and return the slice= {}",item,&s[0..i]);
            
            return &s[0..i]; //When we find a space, we return a string slice 
                             
        }
    }

    &s[..]

    /*We now have a straightforward API that’s much harder to mess up, 
    because the compiler will ensure the references into the String remain valid.
    */

}