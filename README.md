# rustbook
Rustbool is a collection of exercices provided by the book ["The Rust Programming Language"](https://doc.rust-lang.org/book/title-page.html) by Steve Klabnik and Carol Nichols.
The exercices are made using Rust 1.42.0 or later with edition="2018" in Cargo.toml of all projects to use Rust 2018 Edition idioms. 

----
- [rustbook](#rustbook)
  - [An I/O Project: Building a Command Line Program](#an-io-project-building-a-command-line-program)

----

## An I/O Project: Building a Command Line Program

We’ll build a command line tool that interacts with file and command line input/output to practice some of the Rust concepts you now have under your belt.

we’ll make our **own version of the classic command line tool grep** (globally search a regular expression and print). In the simplest use case, grep searches a specified file for a specified string. To do so, grep takes as its arguments a filename and a string. Then it reads the file, finds lines in that file that contain the string argument, and prints those lines.

Along the way, we’ll show **how to make our command line tool use features of the terminal** that many command line tools use.

1- We’ll read the value of an environment variable to allow the user to configure the behavior of our tool

2-We’ll also print error messages to the standard error console stream (stderr) instead of standard output (stdout).

Our grep project will combine a number of concepts you’ve learned so far:

 -  Organizing code
 -  Using vectors and strings
 -  Handling errors
 -  Using traits and lifetimes where appropriate 
 -  Writing tests
 -  introduce closures 
 -  introduce iterators
 -  trait objects










  




