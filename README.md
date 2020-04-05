# rustbook
Rustbool is a collection of exercices provided by the book ["The Rust Programming Language"](https://doc.rust-lang.org/book/title-page.html) by Steve Klabnik and Carol Nichols.
The exercices are made using Rust 1.42.0 or later with edition="2018" in Cargo.toml of all projects to use Rust 2018 Edition idioms. 

----
- [rustbook](#rustbook)
  - [101.1 - Generic DATA types in Method](#1011---generic-data-types-in-method)
    - [Monomorphization](#monomorphization)

----

## 101.1 - Generic DATA types in Method

We can implement methods on structs and enums  and use generic types in their definitions, too.

we have to declare T just after impl so we can use it to specify that we’re implementing methods.

By declaring T as a generic type after impl, Rust can identify that the type in the angle brackets in a struct is a generic type rather than a concrete type.

In some situations some generic parameters are declared with impl and some are declared with the method definition



### Monomorphization

Is the process of **turning generic code into specific code** by filling in the concrete types that are used when compiled. Is the way that your code doesn’t run any slower using generic types than it would with concrete types.

In this process, the compiler does the opposite of the steps we used to create the generic function: the compiler looks at all the places where generic code is called and generates code for the concrete types the generic code is called with.







