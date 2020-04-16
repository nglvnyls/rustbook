#[cfg(test)] // tests module
mod tests {
    #[test] //.This attribute indicates this is a test function
    fn it_works() {
        assert_eq!(2 + 2, 4); //The function body uses the assert_eq! macro
    }

    #[test]
    fn another() {
        panic!("Make this test fail"); //this test FAILS!!!
    }

    /*
    TEST FOR can hold method of Rectangle
    */
    use super::*; //Because the tests module is an inner module, 
                //we need to bring the code under test in the 
                //outer module into the scope of the inner module.
                //We use a glob here so anything we define in the 
                //outer module is available to this tests module.

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn larger_can_hold_smaller2() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };

        assert!(larger.can_hold_with_error(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger2() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };

        assert!(!smaller.can_hold_with_error(&larger));
    }

    /*
    TEST FOR custom message
    */

    #[test]
    fn greeting_contains_name() { //we want to test that the 
                            //name we pass into the function 
                            //appears in the output
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`", result
        );
    }

    #[test]
    fn greeting_contains_name_wrong() { //we want to test the 
                            //message printed when assert fails                   //appears in the output
        let result = greeting("another_thing");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was: `{}`", result
        );
    }

    /*
    Checking that our code returns the correct values we expect
    */
    #[test]
    #[should_panic] //attribute that makes a test pass if the code 
                    //inside the function panics
                    //We place the #[should_panic] attribute 
                    //after the #[test] attribute and before the 
                    //test function it applies to
    fn greater_than_100() { // test that ensures that attempting 
                        //to create a Guess instance with a value 
                        //outside that range panics.
        Guess::new(200);
    }

    /*
     To make should_panic tests more precise, we can add an 
     optional expected parameter to the should_panic attribute
    */

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100_2() { //This test will pass because the value 
        //we put in the should_panic attribute’s expected parameter 
        //is a substring of the message that the Guess::new function 
        //panics with
        Guess2::new(101); 
    }

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100_3() { //This test will FAIL because the value 
        //we put in the should_panic attribute’s expected parameter 
        //is NOT a substring of the message that the Guess::new function 
        //panics with
        Guess2::new(-1); 
    }

    /*
    Test rewritten to use Result<T, E> and return an Err instead of panicking:
    */
    #[test]
    fn it_works2() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(()) //rather than calling the assert_eq! macro, 
                    //we return Ok(()) when the test passes
        } else {
            Err(String::from("two plus two does not equal four")) 
            // we return an Err with a String inside when the test fails.
        }
    }

}


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[allow(dead_code)] //disable the lint that will warn about unused functions.
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool { //returns a Boolean, which means 
                                        //it’s a perfect use case for the assert! macro
        self.width > other.width && self.height > other.height
    }

    fn can_hold_with_error(&self, other: &Rectangle) -> bool { 
        self.width < other.width && self.height > other.height  //we have change < in width equality. That is wrong.
    }
}

#[allow(dead_code)]
fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

#[allow(dead_code)]
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess {
            value
        }
    }
}

#[allow(dead_code)]
pub struct Guess2 {
    value: i32,
}

impl Guess2 {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {}.",
                   value);
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {}.",
                   value);
        }

        Guess {
            value
        }
    }
}