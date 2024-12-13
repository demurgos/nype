//! # Nype - Newtype library for Rust
//!
//! Nype is a library of declarative macros to help you define newtype wrappers.
//! This library is transparent: it does leak into your public API and could
//! be replaced at any time with manual implementations.
//!
//! Nype uses declarative macro (as opposed to procedural macros) to reduce
//! compile times. The trade-off is a somewhat less flexible syntax: the main
//! result is that options must be defined in alphabetical order.
//!
//! By default, Nype has no dependencies and support `no-std` environment. A
//! wide range of traits can be added through the different features.
//!
//! ## Newtype pattern and benefits
//!
//! The goal of the newtype pattern is to wrap an existing type into a new type.
//! This enables a few benefits:
//! 1. The newtype wrapper is fully owned by the crate where it is defined. It
//!    allows the corresponding crate to implement any trait it wants without
//!    being restricted by the orphan rule.
//! 2. The newtype wrapper may carry extra semantics. Construction of the
//!    wrapper can be checked to enforce invariants.
//! 3. The newtype gets its own identity so this enables stronger compiler
//!    checks and better documentation for users.
//!
//! As an example, you may define the newtype `Username` wrapping a `String`:
//! ```ignore
//! pub struct Username(String);
//! ```
//!
//! In this example, the inner value is private, this means that you must
//! provide a constructor, which can also enforce extra checks:
//!
//! ```ignore
//! impl Username {
//!   pub fn new(value: String) -> Result<Self, String> {
//!     // Enforce that a username is restricted to ASCII alphanum chars
//!     let is_valid = value
//!       .chars()
//!       .all(|c| c.is_ascii_alphanumeric() || c == '_');
//!     if is_valid {
//!       Ok(Self(value))
//!     } else {
//!       Err(String::from("invalid input, alls characters must be ASCII alphanum"))
//!     }
//!   }
//! }
//! ```
//!
//! You can also define functions that take `Username` as an argument instead
//! of `String`. This stronger typing helps to avoid errors: the compiler will
//! make sure that you don't mix-up values and pass a valid username. Example:
//!
//! ```ignore
//! fn generate_greeting_email(username: &Username) -> EmailTemplate {
//!   todo!();
//! }
//!
//! fn main() {
//!   let email_address = String::from("john.doe@example.com");
//!   let username = Username::new(String::from("john_doe")).expect("constant username is valid");
//!
//!   let _ = generate_greeting_email(&username); // ok
//!   // let _ = generate_greeting_email(&email_address); // rejected by the compiler
//! }
//! ```
//!
//! You're able to define your own traits and methods directly on the newtype,
//! as opposed to being limited to the methods defined on `String`.
//!
//! ```ignore
//! impl Username {
//!   pub fn get_mock_value(some_rng: ...) -> Self {
//!     match some_rng.rand(0..5) {
//!       0 => Self::new(String::from("Alice")).expect("mock value is valid"),
//!       1 => Self::new(String::from("Bob")).expect("mock value is valid"),
//!       2 => Self::new(String::from("Charlie")).expect("mock value is valid"),
//!       3 => Self::new(String::from("Dan")).expect("mock value is valid"),
//!       _ => Self::new(String::from("Eve")).expect("mock value is valid"),
//!     }
//!   }
//! }
//! ```
//!
//! The newtype pattern can be applied to any value. It is most commonly used
//! to define domain types. In particular it can enforce checks on strings,
//! integers, enums, ids, etc.
//!
//! ## Drawbacks of the newtype pattern
//!
//! There are two main drawbacks to the newtype pattern.
//! 1. Compatibility
//! 2. Boilerplate
//!
//! The fact that newtypes get their own identity is a benefit as it enables
//! stronger type checks, but also a drawback as now your type is distinct.
//! If we follow with the `Username` example from the previous section, it can
//! no longer be passed directly to functions from the standard library or
//! ecosystem that expect a `String`: you need a conversion step first (either
//! explicit or through some extra traits/methods). In practice the benefit of
//! stronger identity are usually worth it, as long as conversion to more common
//! types are implemented.
//!
//! The second drawback is boilerplate. Implementing a reliable and easy-to-use
//! newtype can invovle a fair amount of boilerplate. For example, you should
//! implement conversions as discussed just before. But, you also need custom
//! deserializers to enforce that the checks are performed during deserialization
//! too. You should also have dedicated errors for failures. There could also
//! be some performance concerns: for example the `Username` example requires
//! ownership and allocation of a string, which could cause extra cloning.
//!
//! This extra boilerplate is a major reason why the newtype pattern may not
//! be used enough. This is where Nype steps in: it provides a set of macros
//! to define high quality newtype wrappers while keeping the boilerplate to
//! a minimum.
//!
//! ## Nype macros
//!
//! Nype defines the following macros:
//! - [`define_new_string`]: Define a string-like newtype wrapper.
#![cfg_attr(not(feature = "std"), no_std)]

#[macro_use]
pub mod string;
