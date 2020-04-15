# rustbook
Rustbool is a collection of exercices provided by the book ["The Rust Programming Language"](https://doc.rust-lang.org/book/title-page.html) by Steve Klabnik and Carol Nichols.
The exercices are made using Rust 1.42.0 or later with edition="2018" in Cargo.toml of all projects to use Rust 2018 Edition idioms. 

----
- [rustbook](#rustbook)
  - [103 - Validating References with Lifetimes](#103---validating-references-with-lifetimes)
    - [Preventing Dangling References with Lifetimes](#preventing-dangling-references-with-lifetimes)
    - [The Borrow Checker](#the-borrow-checker)
    - [Generic Lifetimes in Functions](#generic-lifetimes-in-functions)
      - [Lifetime Annotation Syntax](#lifetime-annotation-syntax)
    - [Lifetime Annotations in Function Signatures](#lifetime-annotations-in-function-signatures)
    - [Lifetime Elision](#lifetime-elision)
      - [Rules for elision](#rules-for-elision)
----

## 110 - Writing Automated Test

Rust is designed with a high degree of concern about the correctness of programs, but correctness is complex and not easy to prove.

Rust’s type system shoulders a huge part of this burden, but the type system cannot catch every kind of incorrectness.

When we implement and compile some function, Rust does all the type checking and borrow checking, but Rust can’t check that this function will do precisely what we intend.

That’s where tests come in.





