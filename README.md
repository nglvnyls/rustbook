# rustbook
Rustbool is a collection of exercices provided by the book ["The Rust Programming Language"](https://doc.rust-lang.org/book/title-page.html) by Steve Klabnik and Carol Nichols.
The exercices are made using Rust 1.42.0 or later with edition="2018" in Cargo.toml of all projects to use Rust 2018 Edition idioms. 

<hr>

- [rustbook](#rustbook)
  - [rectangle area](#rectangle-area)
    - [Code](#code)
      - [Firts function: with two variables parameters](#firts-function-with-two-variables-parameters)
      - [Second function: with tuples](#second-function-with-tuples)
      - [Third function: with structs](#third-function-with-structs)
    - [Adding Useful Functionality with Derived Traits](#adding-useful-functionality-with-derived-traits)
      - [print an instance of rectangle](#print-an-instance-of-rectangle)
  
<hr>

## rectangle area

program that calculates the area of a rectangle.

We’ll start with single variables, and then refactor the program until we’re using structs instead.

Rectangle_area will take the width and height of a rectangle specified in pixels and calculate the area of the rectangle.

### Code

There is 3 diferent functions

#### Firts function: with two variables parameters

```
let width1 = 30;
let height1 = 50;

fn area(width: u32, height: u32) -> u32 {

```

The area function is supposed to calculate the area of one rectangle, but the function we wrote has two parameters. The parameters are related, but that’s not expressed anywhere in our program

#### Second function: with tuples

```
let rect1 = (30, 50);

fn area(dimensions: (u32, u32)) -> u32 {

```

Tuples let us add a bit of structure, and we’re now passing just one argument. 

But in another way, this version is less clear: 

**tuples don’t name their elements**, so our calculation has become more confusing because we have to index into the parts of the tuple.

We would have to keep in mind that width is the tuple index 0 and height is the tuple index 1. If someone else worked on this code, they would have to figure this out and keep it in mind as well. It would be easy to forget or mix up these values and cause errors, because we haven’t conveyed the meaning of our data in our code.

#### Third function: with structs

```
struct Rectangle {
    width: u32,
    height: u32,
}

let rect1 = Rectangle { width: 30, height: 50 };


fn area(rectangle: &Rectangle) -> u32 {

```

we’ve defined a struct and named it Rectangle. 

Inside the curly brackets, we defined the fields as width and height, both of which have type u32. 

Then in main, we created a particular instance of Rectangle that has a width of 30 and a height of 50.

Our area function is now defined with one parameter, which we’ve named rectangle, whose type is an immutable borrow of a struct Rectangle instance. This way, main retains its ownership and can continue using rect1, which is the reason we use the & in the function signature and where we call the function.

The area function accesses the width and height fields of the Rectangle instance. Our function signature for area now says exactly what we mean: calculate the area of Rectangle, using its width and height fields. This conveys that the width and height are related to each other, and it gives descriptive names to the values rather than using the tuple index values of 0 and 1. This is a win for clarity.

### Adding Useful Functionality with Derived Traits

#### print an instance of rectangle

```
println!("rect1 is {}", rect1); 
//error[E0277]: `Rectangle` doesn't implement 
//`std::fmt::Display`

```

The println! macro can do many kinds of formatting, and by default, the curly brackets tell println! to use formatting known as Display: output intended for direct end user consumption

But with structs, the way println! should format the output is less clear because there are more display possibilities: 

- Do you want commas or not? 
- Do you want to print the curly brackets? 
- Should all the fields be shown?

Due to this ambiguity, Rust doesn’t try to guess what we want, and structs don’t have a provided implementation of Display.

Putting the specifier :? inside the curly brackets tells println! we want to use an output format called Debug. 

```
println!("rect1 is {:?}", rect1); 
//it causes an error[E0277]: `Rectangle` doesn't implement 
//`std::fmt::Debug

```
Rust does include functionality to print out debugging information, but we have to explicitly opt in to make that functionality available for our struct. To do that, **we add the annotation #[derive(Debug)] just before the struct definition**

It’s not the prettiest output, but **it shows the values of all the fields for this instance**. 

```
rect1 is Rectangle { width: 30, height: 50 }

```

When we have larger structs, it’s useful to have output that’s a bit easier to read; in those cases, we can use **{:#?}** style with this result

```
rect1 is Rectangle {
    width: 30,
    height: 50
}

```
Rust has provided a number of traits for us to use with the derive annotation that can add useful behavior to our custom types.
