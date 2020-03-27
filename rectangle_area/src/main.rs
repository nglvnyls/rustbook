
fn main() {
    println!("Area of a Rectangle");

    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {}",
        rectangle_area(width1, height1)
    );

    /*
    The parameters are related, but that’s not expressed 
    anywhere in our program. 
    It would be more readable and more manageable to group
    width and height together.
    */

    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {}",
        rectangle_area2(rect1)
    );
    println!("rect1 is {:?}", rect1);
    println!("rect1 is {:#?}", rect1);

    /*
    We would have to keep in mind that 
    width is the tuple index 0 
    and height is the tuple index 1. 
    If someone else worked on this code, they would have to figure this out 
    and keep it in mind as well. It would be easy to forget or mix up 
    these values and cause errors, because we haven’t conveyed the meaning 
    of our data in our code.
    */

    

    let rect2 = Rectangle { width: 300, height: 500 }; // created a particular instance of Rectangle 
                                                    //that has a width of 30 and a height of 50

    println!(
        "The area of the rectangle is {}",
        rectangle_area3(&rect2)
    );
    println!("rect2 is {:?}", rect2);
    println!("rect2 is {:#?}", rect2);


}

fn rectangle_area(width: u32, height: u32) -> u32 {
    width * height 
    //function is supposed to calculate the area of one rectangle, 
    //but the function we wrote has two parameters
    
}

fn rectangle_area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
    //Tuples let us add a bit of structure, 
    //and we’re now passing just one argument
    //But is less clear: tuples don’t name their elements
}


/*
we’ve defined a struct and named it Rectangle. 
Inside the curly brackets, we defined the fields 
as width and height, both of which have type u32
*/
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn rectangle_area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
    /*
    function is now defined with one parameter, 
    which we’ve named rectangle, whose type is 
    an immutable borrow of a struct Rectangle instance.

    we want to borrow the struct rather than take ownership of it. 
    This way, main retains its ownership and can continue using rect1, 
    which is the reason we use the & in the function signature 
    and where we call the function
    */
}