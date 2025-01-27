//! Functions to transition between two images, or states of the same image, by the
//! given steps and duration.

mod base;
pub use base::*;

mod sweep;
pub use sweep::*;

mod transverse;
pub use transverse::*;

pub use crate::traits::DrawTransition;
