# rustbook
Rustbool is a collection of exercices provided by the book ["The Rust Programming Language"](https://doc.rust-lang.org/book/title-page.html) by Steve Klabnik and Carol Nichols.
The exercices are made using Rust 1.42.0 or later with edition="2018" in Cargo.toml of all projects to use Rust 2018 Edition idioms. 

----
- [rustbook](#rustbook)
  - [90 - Error Handling](#90---error-handling)

----

## 90 - Error Handling

Errors are a fact of life in software, so Rust has a number of features for handling situations in which something goes wrong.

Rust groups errors into two major categories:

- **recoverable** such as a file not found error, it’s reasonable to report the problem to the user and retry the operation.

- **unrecoverable** errors are always symptoms of bugs, like trying to access a location beyond the end of an array.

**Rust doesn’t have exceptions**

It has the **type Result<T, E>** for **recoverable errors**

the **panic! macro** that **stops execution** when the program encounters an **unrecoverable error**.









