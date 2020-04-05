# rustbook
Rustbool is a collection of exercices provided by the book ["The Rust Programming Language"](https://doc.rust-lang.org/book/title-page.html) by Steve Klabnik and Carol Nichols.
The exercices are made using Rust 1.42.0 or later with edition="2018" in Cargo.toml of all projects to use Rust 2018 Edition idioms. 

----
- [rustbook](#rustbook)
  - [101.1 - Generic DATA types in Enum](#1011---generic-data-types-in-enum)
    - [Performance of Code Using Generics](#performance-of-code-using-generics)

----

## 101.1 - Generic DATA types in Enum

we can define enums to hold generic data types in their variants

When you recognize situations in your code with multiple struct or enum definitions that differ only in the types of the values they hold, you can avoid duplication by using generic types instead.


### Performance of Code Using Generics

Monomorphization is the process of turning generic code into specific code by filling in the concrete types that are used when compiled.

In this process, the compiler does the opposite of the steps we used to create the generic function: the compiler looks at all the places where generic code is called and generates code for the concrete types the generic code is called with.

Because Rust compiles generic code into code that specifies the type in each instance, **we pay no runtime cost for using generics**. When the code runs, it performs just as it would if we had duplicated each definition by hand. The process of monomorphization makes Rust’s generics extremely efficient at runtime.
