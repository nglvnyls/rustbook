# rustbook
Rustbool is a collection of exercices provided by the book ["The Rust Programming Language"](https://doc.rust-lang.org/book/title-page.html) by Steve Klabnik and Carol Nichols.
The exercices are made using Rust 1.42.0 or later with edition="2018" in Cargo.toml of all projects to use Rust 2018 Edition idioms. 

----
- [rustbook](#rustbook)
  - [101 - Generic DATA types](#101---generic-data-types)
      - [In Function Definitions](#in-function-definitions)

----

## 101 - Generic DATA types

We can use generics to create definitions for:
-  items: 
   -  function signatures 
   -  structs

which we can then use with many **different concrete data types**.


#### In Function Definitions

Defining a function that uses generics: 
- place the generics in the signature of the function where we would usually specify the data types of the parameters and return value. Code is more flexible and provides more functionality to callers of our function while preventing code duplication.

To parameterize the types in the new function we’ll define, we need to name the type parameter, just as we do for the value parameters to a function. 

You can use any identifier as a type parameter name. But we’ll **use T** because, **by convention**, parameter names in Rust are short, often just a letter, and **Rust’s type-naming convention is CamelCase**. 

Short for “type,” T is the default choice of most Rust programmers.

To define the generic largest function, place type name declarations inside angle brackets, <>, between the name of the function and the parameter list

```rust
fn largest<T>(list: &[T]) -> T {
```
We read this definition as: the function largest is generic over some type T. This function has one parameter named list, which is a slice of values of type T. The largest function will return a value of the same type T.










