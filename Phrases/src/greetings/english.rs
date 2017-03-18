//! English phrases
//!
//! # Examples
//! ```
//! let username = "John";
//! println!("{}, {}!", phrases::greetings::english::hello(), username)
//! ```

/// Applies to code that follows it.
/// So this is for the `hello()` function.
pub fn hello() -> String { "hello".to_string() }

pub fn goodbye() -> String { "goodbye".to_string() }
