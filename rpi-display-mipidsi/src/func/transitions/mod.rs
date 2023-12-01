//! Functions to transition between two images, or states of the same image, by the
//! given steps and duration.

mod base;
pub use base::*;

#[cfg(feature = "simd")]
mod dissolve;
#[cfg(feature = "simd")]
pub use dissolve::*;

mod transverse;
pub use transverse::*;
