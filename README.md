# rustbook
Rustbool is a collection of exercices provided by the book ["The Rust Programming Language"](https://doc.rust-lang.org/book/title-page.html) by Steve Klabnik and Carol Nichols.
The exercices are made using Rust 1.42.0 or later with edition="2018" in Cargo.toml of all projects to use Rust 2018 Edition idioms. 

## Fibonacci Generator

Generate up to 186nth Fibonacci number. 

App use unsigned 128 bits integer type (**u128**). Unsigned variants can store numbers from 0 to 2^n - 1, so a u128 can store numbers from 0 to 28 - 1, which equals 0 to 255.