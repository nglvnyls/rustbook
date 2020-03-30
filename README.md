# rustbook
Rustbool is a collection of exercices provided by the book ["The Rust Programming Language"](https://doc.rust-lang.org/book/title-page.html) by Steve Klabnik and Carol Nichols.
The exercices are made using Rust 1.42.0 or later with edition="2018" in Cargo.toml of all projects to use Rust 2018 Edition idioms. 

----
- [rustbook](#rustbook)
  - [vector collection](#vector-collection)
      - [Creating a New Vector](#creating-a-new-vector)
      - [Updating a Vector](#updating-a-vector)
      - [Dropping a Vector Drops Its Elements](#dropping-a-vector-drops-its-elements)
      - [Reading Elements of Vectors](#reading-elements-of-vectors)
      - [Iterating over the Values in a Vector](#iterating-over-the-values-in-a-vector)
      - [Using an Enum to Store Multiple Types](#using-an-enum-to-store-multiple-types)

----

## vector collection

**Vec<T>**, also known as a vector.

They allow you to **store more** than one value **in a single data structure** that puts all the values next to each other in memory.

Vectors can **only store values** of the **same type**.

They are useful when you have a list of items: ,
 - such as the lines of text in a file 
 - the prices of items in a shopping cart.

#### Creating a New Vector

To create a new, empty vector, we can call the **Vec::new** function

```rust
#![allow(unused_variables)]
fn main() {
let v: Vec<i32> = Vec::new();
}
```
Note that we added a type annotation here. Because we aren’t inserting any values into this vector, Rust doesn’t know what kind of elements we intend to store. 

**Vectors are implemented using generics**. The Vec<T> type provided by the standard library can hold any type, and when a specific vector holds a specific type, the type is specified within angle brackets. <>

It’s more common to create a Vec<T> that has initial values, and Rust provides the vec! macro for convenience.


#### Updating a Vector

It cans be used the **push method**. 

If values placed inside are all of the same type, Rust infers this form the data, do it is no necessary the Vec<i32>
annotation.

#### Dropping a Vector Drops Its Elements

*A vector is freed when it goes out of scope*.

When the vector gets dropped, **all of its contents** are also **dropped**, meaning those content it holds will be cleaned up. This may seem like a straightforward point but can get a bit more complicated when you start to introduce references to the elements of the vector.

#### Reading Elements of Vectors

There are two ways:

- the **index value**. It  will cause the program to panic because it references a nonexistent element.
  
- the **get method** with the index passed as an argument. When the get method is passed an **index that is outside** the vector, **it returns None without panicking**.The code will then have logic to handle having either Some(&element) or None, by exemple wiht "match"

```rust
#![allow(unused_variables)]
fn main() {
let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2];
println!("The third element is {}", third);

match v.get(2) {
    Some(third) => println!("The third element is {}", third),
    None => println!("There is no third element."),
}
}

```

#### Iterating over the Values in a Vector

we can iterate through all of the elements rather than use indices to access one at a time.

#### Using an Enum to Store Multiple Types

Vectors can only store values that are the **same type**. This can be inconvenient; there are definitely use cases for needing to store a **list of items of different types**.

Fortunately, the variants of an enum are defined under the same enum type, so when we need to store elements of a different type in a vector, **we can define and use an enum!**

it cans define an enum whose variants will hold the different value types, and then all the enum variants will be considered the same type: that of the enum.





















