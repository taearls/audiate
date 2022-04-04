// #![deny(missing_docs)]
// #![cfg_attr(test, deny(warnings))]
#![allow(clippy::module_inception)]

mod note;
pub use note::Note;

pub mod chord;

pub mod scale;
pub use scale::Scale;
