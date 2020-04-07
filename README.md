# rustbook
Rustbool is a collection of exercices provided by the book ["The Rust Programming Language"](https://doc.rust-lang.org/book/title-page.html) by Steve Klabnik and Carol Nichols.
The exercices are made using Rust 1.42.0 or later with edition="2018" in Cargo.toml of all projects to use Rust 2018 Edition idioms. 

----
- [rustbook](#rustbook)
  - [- 103 - Validating References with Lifetimes](#ulli103---validating-references-with-lifetimesliul)
  - [103 - Validating References with Lifetimes](#103---validating-references-with-lifetimes)
    - [Preventing Dangling References with Lifetimes](#preventing-dangling-references-with-lifetimes)
----

## 103 - Validating References with Lifetimes

Every reference has a lifetime.

Lifetime is the scope for which that reference is valid.

Most of the time, lifetimes are implicit and inferred, just like most of the time, types are inferred.

We must **annotate types when multiple types are possible**.

we **must annotate lifetimes when the lifetimes** of references could be related in a few **different ways**.

Rust requires us to annotate the relationships using generic lifetime parameters to ensure the actual references used at runtime will definitely be valid.

### Preventing Dangling References with Lifetimes



