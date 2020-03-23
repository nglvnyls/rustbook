# rustbook
Rustbool is a collection of exercices provided by the book ["The Rust Programming Language"](https://doc.rust-lang.org/book/title-page.html) by Steve Klabnik and Carol Nichols.
The exercices are made using Rust 1.42.0 or later with edition="2018" in Cargo.toml of all projects to use Rust 2018 Edition idioms. 

## Control Flow

The most common constructs that let you control the flow of execution of Rust code are **if expressions and loops**.

### if Expressions

An if expression allows you to **branch** your code **depending on conditions**

 “If this condition is met, run this block of code. If the condition is not met, do not run this block of code.”

 All if expressions start with the keyword **if**, which is **followed by a condition**

The block of code we want to execute if the **condition is true** is **placed immediately after the condition** inside curly brackets

Optionally, we can also include an **else** expression,  to give the program an alternative block of code to execute should **the condition evaluate to false**

The **condition** in this code **must be a bool**.

If the condition isn’t a bool, we’ll get an error

Rust will **not** automatically **try to convert non-Boolean** types to a Boolean

##### Handling Multiple Conditions with else if

You can have multiple conditions by combining if and else in an else if expression

```
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
```
When this program executes, it checks each if expression in turn and **executes the first body** for which the condition holds **true**

 Rust only executes the block for the first true condition, and once it finds one, it doesn’t even check the rest

 ##### Using if in a let Statement

 if is an expression








