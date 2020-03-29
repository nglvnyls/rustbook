# rustbook
Rustbool is a collection of exercices provided by the book ["The Rust Programming Language"](https://doc.rust-lang.org/book/title-page.html) by Steve Klabnik and Carol Nichols.
The exercices are made using Rust 1.42.0 or later with edition="2018" in Cargo.toml of all projects to use Rust 2018 Edition idioms. 

----
- [rustbook](#rustbook)
  - [vector collection](#vector-collection)
      - [Creating a New Vector](#creating-a-new-vector)

----

## vector collection

**Vec<T>**, also known as a vector.

They allow you to **store more** than one value **in a single data structure** that puts all the values next to each other in memory.

Vectors can **only store values** of the **same type**.

They are useful when you have a list of items, such as the lines of text in a file or the prices of items in a shopping cart.

#### Creating a New Vector

To create a new, empty vector, we can call the Vec::new function

```rust
#![allow(unused_variables)]
fn main() {
let v: Vec<i32> = Vec::new();
}
```















