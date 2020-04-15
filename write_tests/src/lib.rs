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
}


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool { //returns a Boolean, which means 
                                        //it’s a perfect use case for the assert! macro
        self.width > other.width && self.height > other.height
    }

    fn can_hold_with_error(&self, other: &Rectangle) -> bool { 
        self.width < other.width && self.height > other.height  //we have change < in width equality. That is wrong.
    }
}

fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}