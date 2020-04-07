use traits::NewsArticle;
use traits::Tweet;
use traits::Summary;
use traits::notify;
use traits::returns_summarizable;

fn main() {
    println!("Traits");
    println!("");

    /*
    we have multiple structs that hold various 
    kinds and amounts of text.

    We want to make a media aggregator library 
    that can display summaries of data that might be 
    stored in a NewsArticle or Tweet instance.
    To do this, we need 
        - a summary from each type, 
        - to request that summary by calling a summarize 
            method on an instance.
    */

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    
    println!("1 new tweet: With summarize: {}", tweet.summarize());
    println!("");
    println!("1 new tweet: With summarize2: {}", tweet.summarize2());
    println!("");

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best
        hockey team in the NHL."),
    };
    
    

    println!("New article available!.With summarize: {}", article.summarize()); 
    println!("");

    println!("New article available!.With summarize2: {}", article.summarize2()); 
    println!("");

    //We can call notify and pass in any instance of NewsArticle or Tweet
    //Code that calls the function with any other type, such as a String or an i32, 
    //won’t compile because those types don’t implement Summary.

    notify(article);

    //call fn notify on lib.rs that prints "Breaking news! {}", item.summarize()
    //as you can see in lib.rs/notify 
    notify(tweet);


    println!("------------------");

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best
        hockey team in the NHL."),
    };

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    
    println!("New article available!.With summarize3: {}", article.summarize3()); 
    println!("");
    println!("1 new tweet.With summarize3: {}", tweet.summarize3()); 
    println!("");

    //We can also use the impl Trait syntax in the return position to return a value 
    //of some type that implements a trait.
    
    println!("--------impl Trait syntax in the return position---------");
    let tw2= returns_summarizable();
    println!("summarize of tweet that returns_summarizable returns {:?}", tw2.summarize());
    println!("summarize2 of tweet that returns_summarizable returns {:?}", tw2.summarize2());
    println!("summarize3 of tweet that returns_summarizable returns {:?}", tw2.summarize3());
    println!("summarize_author of tweet that returns_summarizable returns {:?}", tw2.summarize_author());
    println!("");

    /*Fixing the largest Function with Trait Bounds.*/
    println!("-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-");
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);
    println!("");
    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
    println!("");

    /*Using Trait Bounds to Conditionally Implement Methods*/
    println!("-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-");

    use std::fmt::Display;

    #[derive(Debug)]
    struct Pair<T> {
        x: T,
        y: T,
    }

    // Instantiate a `Pair` generic type of i32
    let p:Pair<i32> = Pair { x:350, y: 98};
    println!("p = {:?}", p );
    println!("p.x = {:?}", p.x );
    println!("p.y = {:?}", p.y );

    // Instantiate a `Pair` generic type of String
    let c:Pair<&str> = Pair { x:"hhha", y: "ahhh"};
    println!("c = {:?}", c );
    println!("c.x = {:?}", c.x );
    println!("c.y = {:?}", c.y );

    impl<T> Pair<T> { //implements the new function for all T types
        fn new(x: T, y: T) -> Self { 
            Self {
                x,
                y,
            }
        }
    }

    impl<T: Display + PartialOrd> Pair<T> { //only implements the cmp_display 
        //method if its inner type T implements the PartialOrd trait that enables 
        //comparison and the Display trait that enables printing.
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {}", self.x);
            } else {
                println!("The largest member is y = {}", self.y);
            }
        }
    }

    println!("p.cmp_display:");
    p.cmp_display();

    println!("c.cmp_display:");
    c.cmp_display();

    println!("");
    
    /*implement a trait for any type that implements another trait*/
    println!("-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-");




    
}

/*
Fixing the largest Function with Trait Bounds.
In the body of largest we wanted to compare two values of type T using the greater than
 (>) operator. Because that operator is defined as a default method on the standard library 
 trait std::cmp::PartialOrd, we need to specify PartialOrd in the trait bounds for T so 
 the largest function can work on slices of any type that we can compare. We don’t need 
 to bring PartialOrd into scope because it’s in the prelude.
*/
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T { //To call this code with only those types
    // that implement the Copy trait, we can add Copy to the trait bounds of T! 
    
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}