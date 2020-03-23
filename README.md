# rustbook
Rustbool is a collection of exercices provided by the book ["The Rust Programming Language"](https://doc.rust-lang.org/book/title-page.html) by Steve Klabnik and Carol Nichols.
The exercices are made using Rust 1.42.0 or later with edition="2018" in Cargo.toml of all projects to use Rust 2018 Edition idioms. 

## Commnents

comments are notes that programmers can leave in their soruce code that the compiler will ignore but people reading the source code may find useful.

##### Simple comment

Comments that start with two slashes and continue until the end of the line.

```
// hello, world
```
also can be placed at the end of lines containing code

```
fn main() {
    let lucky_number = 7; // this is a comment
}
```

##### Multiline comment

Comments that start with  an slash followed by an asterisk, have 1 or more lines of text and finish wiht an asterisk followed by an slash.

```
/* this is the first line comment 
this is the second line comment...
... 
... 
this is the n line
*/
```
