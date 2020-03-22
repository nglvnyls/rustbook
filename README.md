# rustbook
Rustbool is a collection of exercices provided by the book ["The Rust Programming Language"](https://doc.rust-lang.org/book/title-page.html) by Steve Klabnik and Carol Nichols.
The exercices are made using Rust 1.42.0 or later with edition="2018" in Cargo.toml of all projects to use Rust 2018 Edition idioms. 

##variables
Setting Up a New Project
Inside rustbook I made a new project using Cargo, like so:
```
$ cargo new variables
$ cd variables
```
By default variables in rust are **immutable**. This is one of many nudges Rust gives you to write your code in a way that takes advantage of the safety and easy concurrency that Rust offers. However, you still have the option to make your variables mutable.

When a variable is immutable, once a value is bound to a name, you can’t change that value.The compiler guarantees that when you state that a value won’t change, it really won’t change. That means that when you’re reading and writing code, you don’t have to keep track of how and where a value might change. Your code is thus easier to reason through.

But mutability can be very useful. You can make them mutable by adding **mut** in front of the variable name. In addition to allowing this value to change, mut conveys intent to future readers of the code by indicating that other parts of the code will be changing this variable’s value.

####Differences Between Variables and Constants

Being unable to change the value of a variable might have reminded you of another programming concept that most other languages have: constants. Like immutable variables, constants are values that are bound to a name and are not allowed to change, but there are a few differences between constants and variables.

First, you aren’t allowed to use mut with constants. Constants aren’t just immutable by default—they’re always immutable.

You declare constants using the const keyword instead of the let keyword, and the type of the value must be annotated. We’re about to cover types and type annotations in the next section, “Data Types,” so don’t worry about the details right now. Just know that you must always annotate the type.

Constants can be declared in any scope, including the global scope, which makes them useful for values that many parts of code need to know about.

The last difference is that constants may be set only to a constant expression, not the result of a function call or any other value that could only be computed at runtime.

Here’s an example of a constant declaration where the constant’s name is MAX_POINTS and its value is set to 100,000. (Rust’s naming convention for constants is to use all uppercase with underscores between words, and underscores can be inserted in numeric literals to improve readability)
