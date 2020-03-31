# rustbook
Rustbool is a collection of exercices provided by the book ["The Rust Programming Language"](https://doc.rust-lang.org/book/title-page.html) by Steve Klabnik and Carol Nichols.
The exercices are made using Rust 1.42.0 or later with edition="2018" in Cargo.toml of all projects to use Rust 2018 Edition idioms. 

----
- [rustbook](#rustbook)
  - [collection_hash (objects in javascript)](#collection_hash-objects-in-javascript)
    - [Creating a New Hash Map](#creating-a-new-hash-map)
    - [Hash Maps and Ownership](#hash-maps-and-ownership)
    - [Accessing Values in a Hash Map](#accessing-values-in-a-hash-map)
    - [Updating a Hash Map](#updating-a-hash-map)
    - [Hashing Functions](#hashing-functions)

----

## collection_hash (objects in javascript)

The type HashMap<K, V> stores a mapping of keys of type K to values of type V.

Hash maps are useful when you want to look up data using vectors, but by using a key that can be of any type.

### Creating a New Hash Map

It cans create an empty hash map with **new**.

Of our three common collections, this one is the least often used, so it’s not included in the features brought into scope automatically in the prelude. 

Hash maps also have less support from the standard library.

Data is **stored on the heap**. 

Hash maps are homogeneous: 
 - **all of the keys** must have the same type
 - **all of the values** must have the same type.

Another way of constructing a hash map is by **using the collect method** on a vector of tuples, where each tuple consists of a key and its value.

### Hash Maps and Ownership

For types that implement the Copy trait, like i32, the values are copied into the hash map.

For owned values like String, the values will be moved and the hash map will be the owner of those values

### Accessing Values in a Hash Map

We can get a value out of the hash map by providing its key to the get method.

### Updating a Hash Map

Although the number of keys and values is growable, each key can only have one value associated with it at a time.

For changing data in a hash map it can:

- Replace the old value with the new value
  
- keep the old value and ignore the new one
  
- if key does not have value, add new value. Hash maps have a special API for this called entry that takes the key you want to check as a parameter
  
- combine the old value whith the new value.

### Hashing Functions

By default, HashMap uses a “cryptographically strong”1 hashing function that can provide resistance to Denial of Service (DoS) attacks. This is not the fastest hashing algorithm available, but the trade-off for better security that comes with the drop in performance is worth it. 

If you profile your code and find that the default hash function is too slow for your purposes, you can switch to another function by specifying a different hasher. A hasher is a type that implements the BuildHasher trait. 

We’ll talk about traits and how to implement them in Chapter 10. You don’t necessarily have to implement your own hasher from scratch; crates.io has libraries shared by other Rust users that provide hashers implementing many common hashing algorithms.










