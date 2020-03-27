# rustbook
Rustbool is a collection of exercices provided by the book ["The Rust Programming Language"](https://doc.rust-lang.org/book/title-page.html) by Steve Klabnik and Carol Nichols.
The exercices are made using Rust 1.42.0 or later with edition="2018" in Cargo.toml of all projects to use Rust 2018 Edition idioms. 

<hr>

- [rustbook](#rustbook)
  - [Method Syntax](#method-syntax)
    - [Defining Methods](#defining-methods)

<hr>

## Method Syntax

Methods are similar to functions.

Are like functions in:

- declared with the fn keyword and their name 
- can have parameters 
- can return values
- contain some code that is run when they’re called from somewhere else

Are different for functions in:

- are defined within the context of a struct, enum or trait Object
- first parameter is always self, which represents the instance of the struct the method is being called on.
  
### Defining Methods


