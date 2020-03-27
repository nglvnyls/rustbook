# rustbook
Rustbool is a collection of exercices provided by the book ["The Rust Programming Language"](https://doc.rust-lang.org/book/title-page.html) by Steve Klabnik and Carol Nichols.
The exercices are made using Rust 1.42.0 or later with edition="2018" in Cargo.toml of all projects to use Rust 2018 Edition idioms. 


<div style={{ padding: '20px', backgroundColor: 'green' }}>
  <h3>This is JSX</h3>
</div>
## structs

Structs are similar to tuples.

- the pieces of a struct **can be different types**
- **you’ll name each piece of data** so it’s clear what the values mean

we use **struct** a name  and curly brackets. 
Inside curly brackets we define **fields** 
Fields are **the name and the type** of the pieces of data.

```
struct User {
    username: String, 
    email: String,
    sign_in_count: u64,
    active: bool,
}
```
To use a struct after we’ve defined it, we create an **instance** of that struct by specifying **concrete values for each of the fields**

the **struct** definition is like a general **template for the type**, and **instances** fill in that template with **particular data** to create values of the type

To get a specific value from a struct, **we can use dot notation**. 

Instance is immutable, but with mut entire instance can be mutable.

Rust doesn’t allow us to mark only certain fields as mutable, entire instance must be mutable.

#### Using the Field Init Shorthand when Variables and Fields Have the Same Name

If parameter names and the struct field names are exactly the same, **we can use the field init shorthand syntax **

#### Creating Instances From Other Instances With Struct Update Syntax

**The syntax ..** specifies that the **remaining fields** not explicitly set should have the **same value as** the fields in the **given instance**.

#### Using Tuple Structs without Named Fields to Create Different Types







