# rustbook
Rustbool is a collection of exercices provided by the book ["The Rust Programming Language"](https://doc.rust-lang.org/book/title-page.html) by Steve Klabnik and Carol Nichols.
The exercices are made using Rust 1.42.0 or later with edition="2018" in Cargo.toml of all projects to use Rust 2018 Edition idioms. 

## guessing_game

#### Setting Up a New Project
Inside rustbook I made a new project using Cargo, like so:
```
$ cargo new guessing_game
$ cd guessing_game
```
I needed to generate a secret number that the user will try to guess. The secret number should be different every time so the game is fun to play more than once. I used a random number between 1 and 100 so the game isn’t too difficult. Rust doesn’t yet include random number functionality in its standard library. However, the Rust team does provide a ```rand``` crate.

Remember that a crate is a collection of Rust source code files. The project we’ve been building is a binary crate, which is an executable. The rand crate is a library crate, which contains code intended to be used in other programs.

Cargo’s use of external crates is where it really shines. Before I could write code that uses rand, I needed to modify the Cargo.toml file to include the rand crate as a dependency

I add
```
[dependencies]
rand = "0.6.0"
```
Then, that I’ve added the rand crate to Cargo.toml, I started using rand. The next step was to update ```src/main.rs```. Now that I had user input and a random number, I could compare them.
I used a ```match``` expression to decide what to do next based on which variant of Ordering was returned from the call to cmp with the values in guess and secret_number.

Then, I had most of the game working, but the user can make only one guess. I changed that by adding a loop!
Adding a loop, give users more chances at guessing the number.

Typing quit actually quits the game, but so will any other non-number input. However, this is suboptimal to say the least. I wanted the game to automatically stop when the correct number is guessed.

I added a ```break``` statement so when when the user wins, the app quits.

To further refine the game’s behavior, rather than crashing the program when the user inputs a non-number, I made the game ignore a non-number so the user could continue guessing






Run this project in your terminal. 
```
$ cargo run
```
You get this answer
```
Hello, cargo!
```
