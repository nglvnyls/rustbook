# rustbook
Rustbool is a collection of exercices provided by the book ["The Rust Programming Language"](https://doc.rust-lang.org/book/title-page.html) by Steve Klabnik and Carol Nichols.
The exercices are made using Rust 1.42.0 or later with edition="2018" in Cargo.toml of all projects to use Rust 2018 Edition idioms. 

<hr>

- [rustbook](#rustbook)
  - [Method Syntax](#method-syntax)
    - [Defining Methods](#defining-methods)
      - [automatic referencing and dereferencing](#automatic-referencing-and-dereferencing)
    - [Methods with More Parameters](#methods-with-more-parameters)
  - [Associated Functions](#associated-functions)
    - [Multiple *impl* Blocks](#multiple-impl-blocks)

<hr>

## Method Syntax

Methods are similar to functions.

thet are like functions in:

- declared with the fn keyword and their name 
- can have parameters 
- can return values
- contain some code that is run when they’re called from somewhere else

they are different for functions in:

- are defined within the context of a struct, enum or trait Object
- first parameter is always self, which represents the instance of the struct the method is being called on.

Methods let you specify the behavior that instances of your structs have.
  
### Defining Methods

To define the function within the context of Rectangle, we start an **impl (implementation)** block

Methods can take ownership of self, borrow self immutably as we’ve done here, or borrow self mutably, just as they can any other parameter.

We choose **&self** if we don’t want to take ownership, and we just want to read the data in the struct, not write to it.

We use **&mut self** if we want  to change the instance that we’ve called the method on as part of what the method does.

**self** as a first parameter is rare, but this technique is usually used when the method transforms self into something else and you want to prevent the caller from using the original instance after the transformation.

The main benefit of using methods instead of functions, in addition to using method syntax and not having to repeat the type of self in every method’s signature, is for organization

We’ve put all the things we can do with an instance of a type in one impl block rather than making future users of our code search for capabilities of Rectangle in various places in the library we provide.

#### automatic referencing and dereferencing

Calling methods is one of the few places in Rust that has this behavior

Here’s how it works: when you call a method with object.something(), Rust automatically adds in &, &mut, or * so object matches the signature of the method. In other words, the following are the same:

```
p1.distance(&p2);
(&p1).distance(&p2);
```

The first one looks much cleaner. 

This automatic referencing behavior works because methods have a clear receiver—the type of self. 

Given the receiver and name of a method, Rust can figure out definitively whether the method is reading (&self), mutating (&mut self), or consuming (self). 

### Methods with More Parameters

Methods can take multiple parameters that we add to the signature after the self parameter, and those parameters work just like parameters in functions.

## Associated Functions

functions within impl blocks that **don’t take self as a parameter**.

These are called associated functions because **they’re associated with the struct**.

They’re still functions, not methods, because they don’t have an instance of the struct to work with.

They let you namespace functionality that is particular to your struct without having an instance available

```
String::from
```

### Multiple *impl* Blocks

Each struct **is allowed to have multiple** impl blocks.


