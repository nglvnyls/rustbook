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
    - [Submodules in Integration Tests](#submodules-in-integration-tests)
      - [Integration tests for Binary crates](#integration-tests-for-binary-crates)
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

When run cargo test the output include:

- 1- the unit tests 
- 2- the integration tests
- 3- the doc Tests


To run all the tests in a particular integration test file, use the --test argument of cargo test followed by the name of the file:

```rust
$ cargo test --test integration_test
```

### Submodules in Integration Tests

As you add more integration tests, you might want to make more than one file in the tests directory to help organize them; for example, you can group the test functions by the functionality they’re testing. As mentioned earlier, each file in the tests directory is compiled as its own separate crate.

Treating each integration test file as its own crate is useful to create separate scopes that are more like the way end users will be using your crate. However, this means files in the tests directory don’t share the same behavior as files in src do, as you learned in Chapter 7 regarding how to separate code into modules and files.

The different behavior of files in the tests directory is most noticeable when you have a set of helper functions that would be useful in multiple integration test files and you try to follow the steps in the “Separating Modules into Different Files” section of Chapter 7 to extract them into a common module. For example, if we create tests/common.rs and place a function named setup in it, we can add some code to setup that we want to call from multiple test functions in multiple test files

Filename: tests/common.rs
```rust
#![allow(unused_variables)]
fn main() {
  pub fn setup() {
      // setup code specific to your library's tests would go here
  }
}

```
When we run the tests again, we’ll see a new section in the test output for the common.rs file, even though this file doesn’t contain any test functions nor did we call the setup function from anywhere:

```rust
running 1 test
test tests::internal ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target/debug/deps/common-b8b07b6f1be2db70

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target/debug/deps/integration_test-d993c68b431d39df

running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

Having common appear in the test results with running 0 tests displayed for it is not what we wanted. We just wanted to share some code with the other integration test files.

To avoid having common appear in the test output, **instead of creating tests/common.rs, we’ll create tests/common/mod.rs**. 

**This is an alternate naming convention that Rust also understands**. 

Naming the file this way tells Rust not to treat the common module as an integration test file. 

When we move the setup function code into tests/common/mod.rs and delete the tests/common.rs file, the section in the test output will no longer appear. 

Files in subdirectories of the tests directory don’t get compiled as separate crates or have sections in the test output.

After we’ve created tests/common/mod.rs, **we can use it from any of the integration test files as a module**. Here’s an example of calling the setup function from the it_adds_two test in tests/integration_test.rs:

Filename: tests/integration_test.rs

```rust
use test_organization;

mod common; //calling the setup function from the it_adds_two test

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, test_organization::add_two(2));
}

```
Note that the mod common; declaration is the same as the module declaration we demonstrated in Listing 7-21. Then in the test function, we can call the common::setup() function.


#### Integration tests for Binary crates

If our project is a binary crate that only contains a src/main.rs file and doesn’t have a src/lib.rs file, **we can’t create integration tests in the tests directory** and bring functions defined in the src/main.rs file into scope with a use statement.

Only **library crates expose functions that other crates** can use; 

**Binary crates are meant to be run on their own**.







