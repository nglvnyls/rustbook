# rustbook
Rustbool is a collection of exercices provided by the book ["The Rust Programming Language"](https://doc.rust-lang.org/book/title-page.html) by Steve Klabnik and Carol Nichols.
The exercices are made using Rust 1.42.0 or later with edition="2018" in Cargo.toml of all projects to use Rust 2018 Edition idioms. 

----
- [rustbook](#rustbook)
  - [modules_files](#modules_files)

----

## modules_files

All the examples until now  defined multiple modules in one file. 

When modules get large, you might want to move their definitions to a separate file to make the code easier to navigate.

The **mod keyword declares modules**, and Rust looks in a file with the same name as the module for the code that goes into that module.

Rust lets you split a package into multiple crates and a crate into modules so you can refer to items defined in one module from another module. 






