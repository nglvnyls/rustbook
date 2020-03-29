mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting; //bring a path into a scope once 
                                    //and then call the items in that 
                                    //path as if they’re local items 
                                    //This is the idiomatic way (whit parent module)
                                    //calling the function makes it cleas that is not
                                    //locally defined while still minimizing repetition of
                                    //the full path.

/*
use crate::front_of_house::hosting::add_to_waitlist //is not the idiomatic way, 
use front_of_house::hosting  -it can be used a relative paht instead of abosolute
*/

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();//because of use it's not necessary to write crate::front_of_house::
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

use std::collections::HashMap; //it’s idiomatic to specify the full path

fn main() {

    println!("Main function");
    let mut map = HashMap::new();
    map.insert(1, 2);

    /*
    The exception to this idiom is if we’re bringing two items 
    with the same name into scope with use statements, 
    because Rust doesn’t allow that.
    */
    use std::fmt;
    use std::io;

    fn function1() -> fmt::Result { //using the parent modules distinguishes the two types
        Ok(()) /*An empty block as an expression gives the unit type (), 
                but your functions need to return their respective Result types
                */
        // --snip--
    }

    fn function2() -> io::Result<()> {
        unimplemented!("");
        // --snip--
    }

    //Providing New Names with the as Keyword


    use std::fmt::Result;
    use std::io::Result as IoResult;

    fn function3() -> Result {
        // --snip--
        Ok(())
    }

    fn function4() -> IoResult<()> {
        // --snip--
        Ok(())
    }


}






mod front_of_house2 {
    pub mod hosting2 {
        pub fn add_to_waitlist() {}
    }
}

/*re-exporting*/

pub use crate::front_of_house2::hosting2;

pub fn eat_at_restaurant2() {
    hosting2::add_to_waitlist();
    hosting2::add_to_waitlist();
    hosting2::add_to_waitlist();
}






