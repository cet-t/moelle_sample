#![no_std]

#[cfg(feature = "sampler")]
extern crate alloc;

#[cfg(any(feature = "sampler", feature = "shuffle"))]
mod internal;
#[cfg(any(feature = "sampler", feature = "shuffle"))]
pub(crate) use internal::*;

#[cfg(feature = "choice")]
mod choice;
#[cfg(feature = "choice")]
pub use crate::choice::Choice;

#[cfg(feature = "sampler")]
mod sampler;
#[cfg(feature = "sampler")]
pub use crate::sampler::{Alias, Bst, Sampler, SamplerError, SamplerResult};

#[cfg(feature = "shuffle")]
mod shuffle;
#[cfg(feature = "shuffle")]
pub use crate::shuffle::{FisherYates, Shuffle, ShuffleError, ShuffleResult};
