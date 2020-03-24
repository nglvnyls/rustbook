# rustbook
Rustbool is a collection of exercices provided by the book ["The Rust Programming Language"](https://doc.rust-lang.org/book/title-page.html) by Steve Klabnik and Carol Nichols.
The exercices are made using Rust 1.42.0 or later with edition="2018" in Cargo.toml of all projects to use Rust 2018 Edition idioms. 

## Fibonacci Generator

Generate up to 186nth Fibonacci number. 

App use unsigned 128 bits integer type (**u128**). Unsigned variants can store numbers from 0 to 2^n - 1, so a u128 can store numbers from 0 to 28 - 1, which equals 0 to 170_141_183_460_469_231_731_687_303_715_884_105_727.
This values can take untill 186th number of Fibonacci series.

## How to run 

```
$ cd Fibonacci_generator
$ cargo run
```
Console displays,

```
$FIBONACCI NUMERS!
$
$Input how many numbers of FIBONACCI SERIE do you want to output?
$Select(from 1 to 186)
$
```
input one number from 1 to 186 and consolo prints results in
```
$32
$0,1,1,2,3,5,8,13,21,34,55,89,144,233,377,610,987,1597,2584,4181,6765,10946,17711,28657,46368,75025,121393,$196418,317811,514229,832040,1346269,
$------------
$
$Select(from 1 to 186)
$
```
you can input any other number, but remember, it has to be from 1 to 186.
You can stop whit ^C

