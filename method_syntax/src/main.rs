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
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
