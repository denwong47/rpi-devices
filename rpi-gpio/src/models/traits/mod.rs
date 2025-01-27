//! Traits for use with GPIO related structs.

mod from_tuple_rgb;
pub use from_tuple_rgb::*;

mod rgb_between;
pub use rgb_between::*;

mod rgb_transition;
pub use rgb_transition::*;

mod marker;
pub use marker::*;
