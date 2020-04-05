# rustbook
Rustbool is a collection of exercices provided by the book ["The Rust Programming Language"](https://doc.rust-lang.org/book/title-page.html) by Steve Klabnik and Carol Nichols.
The exercices are made using Rust 1.42.0 or later with edition="2018" in Cargo.toml of all projects to use Rust 2018 Edition idioms. 

----
- [rustbook](#rustbook)
  - [100 - Generic_types_traits_lifetimes](#100---generic_types_traits_lifetimes)
    - [Removing Duplication by Extracting a Function](#removing-duplication-by-extracting-a-function)

----

## 100 - Generic_types_traits_lifetimes

Tools for effectively handling the duplication of concepts.

**Generics** are abstract stand-ins for concrete types or other properties

Similar to the way a function takes parameters with unknown values to run the same code on multiple concrete values, functions can take parameters of some generic type instead of a concrete type, like i32 or String.

### Removing Duplication by Extracting a Function

In the same way that you recognize duplicated code to extract into a function, you’ll start to recognize duplicated code that can use generics.







