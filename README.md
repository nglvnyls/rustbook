# rustbook
Rustbool is a collection of exercices provided by the book ["The Rust Programming Language"](https://doc.rust-lang.org/book/title-page.html) by Steve Klabnik and Carol Nichols.
The exercices are made using Rust 1.42.0 or later with edition="2018" in Cargo.toml of all projects to use Rust 2018 Edition idioms. 

----
- [rustbook](#rustbook)
  - [101.1 - Generic DATA types in Struct](#1011---generic-data-types-in-struct)

----

## 101.1 - Generic DATA types in Struct

We can also define structs to use a generic type parameter in one or more fields using the <> syntax. Listing 10-6 shows how to define a Point<T> struct to hold x and y coordinate values of any type.

