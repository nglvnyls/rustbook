# rustbook
Rustbool is a collection of exercices provided by the book ["The Rust Programming Language"](https://doc.rust-lang.org/book/title-page.html) by Steve Klabnik and Carol Nichols.
The exercices are made using Rust 1.42.0 or later with edition="2018" in Cargo.toml of all projects to use Rust 2018 Edition idioms. 

----
- [rustbook](#rustbook)
  - [Modules](#modules)

----

## Modules 

let us **organize code** within a crate into groups for readability and easy reuse.

Modules also **control the privacy of items**, which is whether an item can be used by outside code (**public**) or is an internal implementation detail and not available for outside use (**private**).

We define a module by starting with the **mod** keyword and then specify the name of the module  and place curly brackets around the body of the module.

```rust
mod front_of_house {
```

Inside modules, we can have other modules

can also hold definitions for other items, such as structs, enums, constants, traits, or functions.

By using modules, we can group related definitions together and name why they’re related.

Programmers using this code would have an easier time finding the definitions they wanted to use because they could navigate the code based on the groups rather than having to read through all the definitions

Programmers adding new functionality to this code would know where to place the code to keep the program organized.

**src/main.rs and src/lib.rs** are called **crate roots** because the contents of either of these two files **form a module named crate** at the root of the crate’s module structure, known as the **module tree**

if module A is contained inside module B, we say that module A is the child of module B and that module B is the parent of module A

the entire module tree is rooted under the implicit module named crate. So crate is the first parent of every module.

Modules are just like directories in a filesystem,  adn you use it to organize your code. 
