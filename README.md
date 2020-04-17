# rustbook
Rustbool is a collection of exercices provided by the book ["The Rust Programming Language"](https://doc.rust-lang.org/book/title-page.html) by Steve Klabnik and Carol Nichols.
The exercices are made using Rust 1.42.0 or later with edition="2018" in Cargo.toml of all projects to use Rust 2018 Edition idioms. 

----
- [rustbook](#rustbook)
  - [111 - Controlling how tests are run](#111---controlling-how-tests-are-run)
    - [Running Tests in Parallel or Consecutively with](#running-tests-in-parallel-or-consecutively-with)
    - [Showing Function output](#showing-function-output)
    - [Running a Subset of Tests by name](#running-a-subset-of-tests-by-name)
    - [Filtering to run multiple tests.](#filtering-to-run-multiple-tests)
    - [Ignoring Some Tests Unless Specifically Requested](#ignoring-some-tests-unless-specifically-requested)
----

## 111 - Controlling how tests are run

Just as cargo run compiles your code and then runs the resulting binary, **cargo test** compiles your code in test mode and **runs the resulting test binary**.

it can be specified command line **options to change** the default **behavior of cargo test**

options followed by **--**  :goes to cargo test
**--** options after this   :goes to the resulting test binary

```rust
  cargo test --help //display the options you can use with cargo test
  cargo test -- --help //displays the options you can use after the separator --
```

### Running Tests in Parallel or Consecutively with

By default **test run in parallel**

Make sure your tests don’t depend on each other or on any shared state, including a shared environment, such as the current working directory or environment variables.

If you **don’t want to run the tests in parallel** or if you want more fine-grained control over the number of threads used, you can **send the --test-threads** flag and the number of threads you want to use to the test binary.

```rust
  cargo test -- --test-threads=1 //set the number of test threads to 1, telling the program not to use any parallelism
```
Running the tests using one thread will take longer than running them in parallel, but the tests won’t interfere with each other if they share state.

### Showing Function output

```rust
  cargo test -- --show-output //also show the output of successful tests
```

### Running a Subset of Tests by name

it can be run only the tests pertaining in a particular area.

You can choose which tests to run by **passing cargo test the name** or names of the test(s) you want to run **as an argument.**

```rust
  cargo test one_hundred 
```
The test output lets us know if we had more tests than what this command ran by displaying the ones filtered out at the end of the summary line.

We can’t specify the names of multiple tests in this way; only the first value given to cargo test will be used

### Filtering to run multiple tests.

It can be specified a** part of a test name**.

Any test whose **name matches that value will be run**.

### Ignoring Some Tests Unless Specifically Requested

It can be excluded some test during most runs of cargo test.

Using the ignore attribute to exclude them.

After #[test] we add the #[ignore] line to the test we want to exclude.

```rust
#[test]
#[ignore]
fn expensive_test() {
    // code that takes an hour to run
}
```
If we want to run only the ignored tests, we can use cargo test -- --ignored:

```rust
$ cargo test -- --ignored
```







- 





