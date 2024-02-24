# Rust Basics

## Variables

- Variables are immutable by default
    - This is for Safety, Concurrency, Speed.

## Scope

- Variables are limited to scopes. They are block scoped by default.

## Memory Safety 

- Rust provides memory safety in compile time.


## Control Flow 

- `If` is an expression not a statement in Rust. Which means that `if` can return a value. 

## String

- `&str` is a borrowed string and the value cannot be modified.
- `String` is also a string type where the values can be modified.

## Ownership 

- Rules
    - Each value has an owner
    - Only one owner 
    - Value gets dropped if its owner goes out of scope

