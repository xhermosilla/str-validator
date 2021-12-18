//! A library of string validators and sanitizers.
//! 
//! This project tries to migrate the [validator.js](https://github.com/validatorjs/validator.js) library
//! written in javascript to the rust programming language.
//! 
//! The main goal is to help me with the rust language. Anyone who wishes to participate in any way is welcome :), 
//! either helping with the migration or providing comments, suggestions, corrections, etc.
mod contains;
mod equals;

pub use contains::{contains, contains_with_options, ContainsOptions};
pub use equals::equals;