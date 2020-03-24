# rustbook
Rustbool is a collection of exercices provided by the book ["The Rust Programming Language"](https://doc.rust-lang.org/book/title-page.html) by Steve Klabnik and Carol Nichols.
The exercices are made using Rust 1.42.0 or later with edition="2018" in Cargo.toml of all projects to use Rust 2018 Edition idioms. 

## Ownership

Rust’s central feature is ownership. Ownership is **how memory is managed** ,with a set of rules that the compiler checks at compile time. **None** of the ownership features **slow down your program while it’s running**.

### Ownwership task

Ownership exists for **managing heap data** 

- **Keeping track of** what parts of **code** are using **what data** on the heap, 
- **minimizing** the amount of **duplicate data** on the heap, 
- **Cleaning up unused data** on the heap so you don’t run out of space 

### The Stack and the Heap

In a systems programming language like Rust, whether a value is on the stack or the heap has more of an effect on how the language behaves and why you have to make certain decisions.

Both the stack and the heap are parts of memory that are available to your code to use at runtime, but they are structured in different ways

#### Stack

The stack **stores values in the order it gets them** and removes the values in the opposite order.

Think of a stack of plates: when you add more plates, you put them on top of the pile, and when you need a plate, you take one off the top.

**Adding** data is called **pushing** onto the stack.

**Removing** data is called **popping** off the stack

All **data** stored on the stack **must have a known, fixed size**

Pushing to **the stack is faster** than allocating on the **heap**

#### Heap

The Headp stores values with an **unknown size at compile time or a size that might change**.

**Pushing** values onto the stack is not considered **allocating**

Think of being seated at a restaurant. When you enter, you state the number of people in your group, and the staff finds an empty table that fits everyone and leads you there. If someone in your group comes late, they can ask where you’ve been seated to find you.

Accessing data in **the heap is slower** than accessing data on the **stack** because you have to follow a pointer to get there.

### Ownership rules

- **Each value has** a variable that’s called **its owner**.
- There can **only be one owner at a time**.
- When the **owner goes out of scope**, the value will be **dropped**.

### Variable scope

A scope is the range within a program for which an item is valid.

there are two important points in time here:

- When variable comes into scope, it is valid.
- It remains valid until it goes out of scope.


## The String Type

There are two types 

#### String literals

String literals are convenient, but they aren’t suitable for every situation in which we may want to use text. 

- they’re immutable. 

#### String type

This type is allocated on the heap and as such is able to store an amount of text that is unknown to us at compile time. 

You can create a String from a string literal using the from function,

This kind of string **can be mutated**


### Memory and Allocation

With the String type, in order to support a mutable, growable piece of text, we need to allocate an amount of memory on the heap, unknown at compile time, to hold the contents. 

This means:

- The memory must be requested from the operating system at runtime. That first part is done by the code: when we call String::from, its implementation requests the memory it needs.


- We need a way of returning this memory to the operating system when we’re done with our String.Rust returned automatically the memory once the variable that owns it goes out of scope. When a variable goes out of scope, Rust calls a  function is called **drop**, and it’s where the author of String can put the code to return the memory. Rust calls **drop automatically at the closing curly bracket**.

#### Ways Variables and Data Interact: Move

#### Ways Variables and Data Interact: Clone

If we do want to **deeply copy the heap data** of the String, not just the stack data, we can use a common method called clone

#### Stack-Only Data: Copy

Copy trait is a **special annotation**  that we can place on types like integers that are stored on the stack . 

If a type has the Copy trait, an older variable is still usable after assignment. Rust won’t let us annotate a type with the Copy trait if the type, or any of its parts, has implemented the Drop trait

Here are some of the types that are Copy:

- All the integer types, such as u32.
- The Boolean type, bool, with values true and false.
- All the floating point types, such as f64.
- The character type, char.
- Tuples, if they only contain types that are also Copy. For example, (i32, i32) is Copy, but (i32, String) is not.

### Ownership and Functions

Passing a variable to a function will move or copy, just as assignment does

### Return Values and Scope
Returning values can also transfer ownership.

The ownership of a variable follows the same pattern every time: 

- assigning a value to another variable moves it. 
- When a variable that includes data on the heap goes out of scope, the value will be cleaned up by drop unless the data has been moved to be owned by another variable.

Taking ownership and then returning ownership with every function is a bit tedious. What if we want to let a function use a value but not take ownership? It’s quite annoying that anything we pass in also needs to be passed back if we want to use it again, in addition to any data resulting from the body of the function that we might want to return as well.

It’s possible to return multiple values using a tuple

But this is too much ceremony and a lot of work for a concept that should be common. **Luckily for us, Rust has a feature for this concept, called references**











