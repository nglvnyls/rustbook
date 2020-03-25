# rustbook
Rustbool is a collection of exercices provided by the book ["The Rust Programming Language"](https://doc.rust-lang.org/book/title-page.html) by Steve Klabnik and Carol Nichols.
The exercices are made using Rust 1.42.0 or later with edition="2018" in Cargo.toml of all projects to use Rust 2018 Edition idioms. 

## References and Borrowing

**Ampersands(&)** are references.
They allow you to refer to some value without taking ownership of it. Because it does not own it, the value it points to will not be dropped when the reference goes out of scope.

When functions have references as parameters instead of the actual values, we won’t need to return the values in order to give back ownership, because we never had ownership.

**Borrowing is having references as function parameters**. 

**References are immutable by default,** as variables. We’re not allowed to modify something we have a reference to.

The opposite of referencing by using & is **dereferencing**, which is accomplished with the **dereference operator, ***

### Mutable references

References are immutable by default, but you can change to be mut.

1 - first change variable definition to be mut.
2 - Create a mutable reference in function call
3 - Function definition has to accept mutable reference as a parameter.

But mutable references have one big restriction: you can have only one mutable reference to a particular piece of data in a particular scope. 

```
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s;

println!("{}, {}", r1, r2);
```
This code will fail.

The benefit of having this restriction is that **Rust can prevent data races at compile time**. 

A data race is similar to a race condition and happens when these three behaviors occur:

- Two or more pointers access the same data at the same time.
- At least one of the pointers is being used to write to the data.
- There’s no mechanism being used to synchronize access to the data.

**Data races cause undefined behavior and can be difficult to diagnose and fix when you’re trying to track them down at runtime; Rust prevents this problem from happening because it won’t even compile code with data races!**

A similar rule exists for combining mutable and immutable references if they are simultaneos ones.

But it can be uses if immutable references occurs before the mutable reference is introduced, because they scopes don’t overlap .


### Dangling References

In Rust, **the compiler guarantees that references will never be dangling references**

Create a dangling pointer, is create a pointer that references a location in memory that may have been given to someone else, by freeing some memory while preserving a pointer to that memory.

```
fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!
```
The solution here is to return the String directly from

```
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
```
###The Rules of References

- At any given time, you can have either one mutable reference or any number of immutable references.

- References must always be valid











