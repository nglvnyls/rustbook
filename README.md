# rustbook
Rustbool is a collection of exercices provided by the book ["The Rust Programming Language"](https://doc.rust-lang.org/book/title-page.html) by Steve Klabnik and Carol Nichols.
The exercices are made using Rust 1.42.0 or later with edition="2018" in Cargo.toml of all projects to use Rust 2018 Edition idioms. 

## Loops

It’s often useful to **execute a block of code more than once**

 A loop runs through the code inside the loop body to the end and then starts immediately back at the beginning

 Rust has three kinds of loops: 

 - loop
 - while
 - for

 #### Repeating Code with loop

The loop keyword tells Rust to **execute** a block of code **over and over again** forever or until you explicitly tell it to stop.

```
fn main() {
    loop {
        println!("again!");
    }
}
```
this program printed over and over continuously until we stop the program manually.

Rust provides another, more reliable way to break out of a loop. You can place the **break** keyword within the loop to tell the program when to **stop executing the loop**.

####  Returning Values from Loops

One of the uses of a loop is to retry an operation you know might fail, such as checking whether a thread has completed its job. 
 
However, you might need to pass the result of that operation to the rest of your code. To do this, you **can add the value you want returned after the break expression you use to stop the loop**; that value will be returned out of the loop so you can use it.


#### Conditional Loops with while

It’s often useful for a program to evaluate a condition within a loop.While the condition is true, the loop runs.

When the condition ceases to be true, the program calls break, stopping the loop.

This loop type could be implemented using a combination of loop, if, else, and break; you could try that now in a program, if you’d like.

However, this pattern is so common that Rust has a built-in language construct for it, called a while loop

#### Looping Through a Collection with for

You could u**se the while construct to loop over the elements of a collection**, such as an array.

But this approach is error prone; we could cause the program to panic if the index length is incorrect.

As a more concise alternative, you can use a for loop and execute some code for each item in a collection.

More importantly, we increase the safety of the code and eliminated the chance of bugs that might result from going beyond the end of the array or not going far enough and missing some items.

The **safety and conciseness** of **for** loops make them the **most commonly used loop construct** in Rust.

Even in situations in which **you want to run some code a certain number** of times, you would use a for loop.The way to do that would be to use a **Range**, which is a type provided by the standard library that generates all numbers in sequence starting from one number and ending before another number.

You can use a for loop and another method  **rev**, to **reverse the range**:

















