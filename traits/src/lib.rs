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

pub trait Summary { //we declare a trait using the trait 
                    //keyword and then the trait’s name, 
                    //which is Summary in this case
    //A trait can have multiple methods in its body: the method 
    //signatures are listed one per line and each line ends 
    //in a semicolon.

    fn summarize(&self) -> String; //After the method signature, 
                    //instead of providing an implementation within 
                    //curly brackets, we use a semicolon

    fn summarize2(&self) -> String { //Sometimes it’s useful to have 
                    //default behavior for some or all of the methods 
                    //in a trait instead of requiring implementations 
                    //for all methods on every type.
        String::from("(Read more...)")
    }   

    //Default implementations can call other methods in the same trait, 
    //even if those other methods don’t have a default implementation
    fn summarize_author(&self) -> String;

    fn summarize3(&self) -> String { //to use summarize3, it needs to define
        //summarize_author when we implement the trait on a type
        format!("(Read more from {}...)", self.summarize_author())
    }
}
/*
After implementing the trait, we can call the methods on instances 
of NewsArticle and Tweet in the same way we call regular methods,
*/

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle { //Implementing a trait on a type is similar 
                                //to implementing regular methods
    fn summarize(&self) -> String { //implementation of the Summary trait 
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
    //To use summarize3, it is needed to define summarize_author when we implement the trait on a type

    fn summarize_author(&self) -> String { //After we define summarize_author, 
        //we can call summarize on instances of the Tweet struct, and the default implementation of 
        //summarize3 will call the definition of summarize_author that we’ve provided
        format!("author: {}", self.author)
    }
}

#[derive(Debug)]
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String { //we define summarize as the username followed by 
                                //the entire text of the tweet, assuming that tweet 
                                //content is already limited to 280 characters.
        format!("{}: {}", self.username, self.content)
    }
    fn summarize_author(&self) -> String {//definition here (implement the trait on type) 
        //override default implementation, ang it's no possible call default implementation
        format!("@{}", self.username)
    }
}

//how to use traits to define functions that accept many different types.

pub fn notify(item: impl Summary) { //Instead of a concrete type for the 
    //item parameter, we specify the impl keyword and the trait name
    //This parameter accepts any type that implements the specified trait.

    println!("Breaking news! {}", item.summarize()); //we can call any methods 
    //on item that come from the Summary trait, such as summarize.
}

/*
trait bound:  This longer form is equivalent to the example 
in the previous section but is more verbose.

    pub fn notify<T: Summary>(item: T) {
        println!("Breaking news! {}", item.summarize());
    }
*/

/*
We can also use the impl Trait syntax in the return position to return 
a value of some type that implements a trait, as shown here:
*/

pub fn returns_summarizable() -> impl Summary { //returns_summarizable returns a Tweet, 
    //but the code calling this function doesn’t know that.
    Tweet {
        username: String::from("user1 inside Tweet returned by returns_summarizable()"),
        content: String::from("contents inside Tweet returned by returns_summarizable()"),
        reply: false,
        retweet: false,
    }
}






    



