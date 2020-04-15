fn main() {
    println!("Lifetimes");
    println!("");

    /*
    This code won’t compile because the value r is 
    referring to has gone out of scope before we try to 
    use it.
    The variable x doesn’t “live long enough.” 
    The reason is that x will be out of scope when 
    the inner scope ends on line 27.
    r "lives longer" because its scope is larger. 
    r is valid for the outer scope.



    let r; //The outer scope declares a variable named r 
            with no initial value
    
    {// begin the inner scope
    
        let x = 5; //the inner scope declares a variable 
                    named x with the initial value of 5
        r = &x; //it attempts to set the value of r as 
                a reference to x
        
    }//end of inner scope

    println!("r:{}", r); // here x is out of scope. 
                            Inner scope ends on line 27
    */

    let r; //The outer scope declares a variable named r 
           // with no initial value
    {
        let x = 5; //the inner scope declares a variable 
                   // named x with the initial value of 5
        r = &x; //it attempts to set the value of r as 
                //a reference to x

        println!("r is in the scope of x, r= {}", r); //r can reference x because 
                        //Rust knows that the reference 
                        //in r will always be valid while 
                        //x is valid.    
    
    } //ens of x scope. 

 
    println!("-+-+-+-+-+-+-+-+-+-+-+-+-");
    println!("");

    let string1 = String::from("abcd");
    println!("string1: {}",string1);
    let string2 = "xyz";
    println!("string2: {}",string2);

    let result = longest_with_lifetime(string1.as_str(), string2);
    println!("The longest_with_lifetime string is {}", result);

    println!("");
    println!("-+-+-+-+-+-+-+-+-+-+-+-+-");
    println!("Lifetime Annotations in Struct Definitions");
    println!("");

    let novel = String::from("Call me Ishmael. Some years ago...");
    println!("novel: {}",novel);
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    println!("first sentence: {}",first_sentence);
    let i = ImportantExcerpt { part: first_sentence };
    println!("i: {:?}",i);
    
    println!("");
    println!("-+-+-+-+-+-+-+-+-+-+-+-+-");
    println!("Lifetime Elision");
    println!("");

    let mut s = String::from("the lifetime elision rules");
    println!("s: {}",s);
    let word = first_word(&s); 
    println!("the first word of s: {}",word);


    

    



}
/*
fn longest(x: &str, y: &str) -> &str { //Rust can’t tell 
    /if the reference being returned refers to x or y
    /it won’t compile.
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
*/
/*
To fix this error, we’ll add generic lifetime parameters 
that define the relationship between the references so 
the borrow checker can perform its analysis.


&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
*/


fn longest_with_lifetime<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[derive(Debug)]
struct ImportantExcerpt<'a> { // It’s possible for structs to hold 
                            //references, but in that case we would 
                            //need to add a lifetime annotation on 
                            //every reference in the struct’s 
                            //definition
    part: &'a str, //part, that holds a string slice, which is a reference.
                    //'a is a lifetime annotation
} //This annotation means an instance of ImportantExcerpt can’t outlive 
  //the reference it holds in its part field.


fn first_word(s: &str) -> &str {//fits rules for lifetime elision , so 
                            //doesn’t need to write the lifetimes explicitly.
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}  

/*
the compiler applies the first rule, which specifies that each 
parameter gets its own lifetime. We’ll call it 'a as usual, so now 
the signature is this:

fn first_word<'a>(s: &'a str) -> &str {

The second rule applies because there is exactly one input lifetime. 
The second rule specifies that the lifetime of the one input 
parameter gets assigned to the output lifetime, so the signature 
is now this:

fn first_word<'a>(s: &'a str) -> &'a str {

Now all the references in this function signature have lifetimes, 
and the compiler can continue its analysis without needing the 
programmer to annotate the lifetimes in this function signature.    
*/
