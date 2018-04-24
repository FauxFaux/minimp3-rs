extern crate cast;
#[macro_use]
extern crate more_asserts;

/// Raw generated bindings, directly from `bindgen`.
pub mod bindgen;

/// Safe wrappers for the generated bindings.
pub mod safe;
