//! # audiate
//!
//! Audiate is a library that allows you to generate notes, chords, and scales. It utilizes the math inherent to music theory to calculate notes with enharmonic correctness
//!
//!
//! ## Basic Usage
//!
//! Make a C major chord from a root C note.
//!
//! ```
//! use audiate::{Note, Chord, ChordQuality};
//!
//! let root_note = Note::try_from("C").unwrap();
//! let chord = Chord::new(root_note, ChordQuality::Major);
//!
//! assert_eq!(root_note, chord.root());
//! assert_eq!(Note::try_from("E").unwrap(), chord.third());
//! assert_eq!(Note::try_from("G").unwrap(), chord.fifth());
//!
//! ```

// #![deny(missing_docs)]
#![cfg_attr(test, deny(warnings))]
#![allow(clippy::module_inception)]

mod note;
pub use note::Note;

pub mod chord;
pub use chord::{Chord, ChordQuality};

pub mod scale;
pub use scale::Scale;
