mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup; //Because we made the Appetizer enum public, 
                                                 //we can use the Soup and Salad variants 
    let order2 = back_of_house::Appetizer::Salad; 

}

// Relative path with "super" to go to parents module

fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // Relative path with "super" to go to parents module
        super::serve_order();
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast { // A function defined in an impl block can be standalone, 
                    //meaning it would be called like Foo::bar()
        /*
        because back_of_house::Breakfast has a private field, 
        the struct needs to provide a public associated function that 
        constructs an instance of Breakfast (we’ve named it summer here)
        f Breakfast didn’t have such a function, we couldn’t create an instance 
        of Breakfast in eat_at_restaurant because we couldn’t set the value of 
        the private seasonal_fruit field in eat_at_restaurant.
        */
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }

}