// #![deny(missing_docs)]
// #![cfg_attr(test, deny(warnings))]
#![allow(clippy::module_inception)]

pub mod note;
pub use note::{Note, NotePitchInterval};

pub mod scale;
pub use scale::{Scale, ScaleDirection, ScaleKind};

pub mod chord;
