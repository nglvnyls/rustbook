/*
#![allow(unused_variables)]
fn main() {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

To continue with our example and extract the hosting module 
to its own file as well, we change src/front_of_house.rs 
to contain only the declaration of the hosting module:

*/
pub mod hosting;
