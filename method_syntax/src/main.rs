fn main() {
    println!("Method syntax");
    println!("");

    let rect1 = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {}",
        rect1.area() //we called the area function and passed rect1 as an argument
                    //we can instead use method syntax to call the area method on our Rectangle instance
                    //rect1.area()
    );


    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    //associated Functions

    let square1 = Rectangle::square(50); //This function is namespaced by the struct: 
                                        //the :: syntax is used for both associated functions 
                                        //and namespaces created by modules
    println!( 
        "Square1 is a {:?}",
        square1 //
    );

}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
/*
To define the function within the context of Rectangle, 
we start an impl (implementation) block.
*/
impl Rectangle {
    fn area(&self) -> u32 { //we use &self instead of rectangle: &Rectangle 
                            //because Rust knows the type of self is Rectangle 
                            //due to this method’s being inside the impl Rectangle context
        /*
        We’ve chosen &self here for the same reason we used &Rectangle in the function version: 
        we don’t want to take ownership, and we just want to read the data in the struct, not write to it
        */                    
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool { //just want to read the data in the struct, not write to it
                                                     // for second parameter we take an immutable borrow of another Rectangle
                                                     // This makes sense because we only need to read
                                                     // and we want main to retain ownership of rect2 so 
                                                     // we can use it again after calling the can_hold method.

        self.width > other.width && self.height > other.height
    }

    //Associated Functions

    /*
    Associated functions are often used for constructors that will return a new instance of the struct
    */

    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }



}
