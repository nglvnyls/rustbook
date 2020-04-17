# rustbook
Rustbool is a collection of exercices provided by the book ["The Rust Programming Language"](https://doc.rust-lang.org/book/title-page.html) by Steve Klabnik and Carol Nichols.
The exercices are made using Rust 1.42.0 or later with edition="2018" in Cargo.toml of all projects to use Rust 2018 Edition idioms. 

----
- [rustbook](#rustbook)
  - [113 - Test organization](#113---test-organization)
    - [unit tests](#unit-tests)
      - [The Tests Module and #[cfg(test)]](#the-tests-module-and-cfgtest)
      - [Testing Private functions](#testing-private-functions)
    - [integration tests](#integration-tests)
      - [The tests Directory](#the-tests-directory)
----

## 113 - Test organization

The Rust community thinks about tests in terms of two main categories:

- unit tests
- integration tests

### unit tests

Unit tests are small and more focused, **testing one module in isolation at a time**, and can test private interfaces.

You’ll put **unit tests in the src directory in each file with the code** that they’re testing.

The **convention is to create** a **module named tests** in each file to contain the test functions and to annotate the module with cfg(test).

#### The Tests Module and #[cfg(test)]

The #[cfg(test)] annotation on the tests module tells Rust to compile and run the test code only when you run cargo test, not when you run cargo build

if unit tests go in the same files as the code, you’ll use #[cfg(test)] to specify that they shouldn’t be included in the compiled result.


#### Testing Private functions

Rust’s privacy rules do allow you to test private functions.

### integration tests

There are **entirely external to your library** and use your code in the same way any other external code would, using only the public interface and potentially exercising multiple modules per test.

**Units of code** that work correctly on their own **could have problems when integrated**, so test coverage of the integrated code is important as well. 

To create integration tests, you first **need a tests directory.**

#### The tests Directory

Tests directory would be **created** at the top level of our project directory, **next to src**. 

It can be made **as many test files** as we want to in this directory, and Cargo **will compile each** of the files as **an individual crate**.






- 





