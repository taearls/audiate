//! # audiate
//!
//! Audiate is a library that allows you to generate notes, chords, and scales. It utilizes the math inherent to music theory to calculate notes with enharmonic correctness
//!
//! ## Basic Usage
//!
//! ### Chords
//! Generate C major, minor, augmented, and diminished chords from a root C note.
//!
//! ```
//! use audiate::{
//!     chord::{Chord, ChordQuality},
//!     Note,
//! };
//!
//! let root_note = Note::try_from("C").unwrap();
//!
//! let major_chord = Chord::new(root_note, ChordQuality::Major);
//! assert_eq!(root_note, major_chord.root());
//! assert_eq!(Note::try_from("E").unwrap(), major_chord.third());
//! assert_eq!(Note::try_from("G").unwrap(), major_chord.fifth());
//!
//! let minor_chord = Chord::new(root_note, ChordQuality::Minor);
//! assert_eq!(root_note, minor_chord.root());
//! assert_eq!(Note::try_from("Eb").unwrap(), minor_chord.third());
//! assert_eq!(Note::try_from("G").unwrap(), minor_chord.fifth());
//!
//! let diminished_chord = Chord::new(root_note, ChordQuality::Diminished);
//! assert_eq!(root_note, diminished_chord.root());
//! assert_eq!(Note::try_from("Eb").unwrap(), diminished_chord.third());
//! assert_eq!(Note::try_from("Gb").unwrap(), diminished_chord.fifth());
//!
//! let augmented_chord = Chord::new(root_note, ChordQuality::Augmented);
//! assert_eq!(root_note, augmented_chord.root());
//! assert_eq!(Note::try_from("E").unwrap(), augmented_chord.third());
//! assert_eq!(Note::try_from("G#").unwrap(), augmented_chord.fifth());
//!
//! ```
//!
//! ### Scales
//! Generate scales from a root C note.
//!
//! ```
//! use audiate::{
//!     scale::{Scale, ScaleDirection, ScaleKind},
//!     Note,
//! };
//!
//! let root_note = Note::try_from("C").unwrap();
//!
//! let major_scale = Scale::new(root_note, ScaleKind::Major, ScaleDirection::Ascending);
//! assert_eq!(String::from("C D E F G A B C"), major_scale.print());
//!
//! let minor_scale = Scale::new(root_note, ScaleKind::Minor, ScaleDirection::Ascending);
//! assert_eq!(String::from("C D Eb F G Ab Bb C"), minor_scale.print());
//!
//! let harmonic_minor_scale = Scale::new(
//!     root_note,
//!     ScaleKind::HarmonicMinor,
//!     ScaleDirection::Ascending,
//! );
//! assert_eq!(
//!     String::from("C D Eb F G Ab B C"),
//!     harmonic_minor_scale.print()
//! );
//!
//! let major_pentatonic_scale = Scale::new(
//!     root_note,
//!     ScaleKind::MajorPentatonic,
//!     ScaleDirection::Ascending,
//! );
//! assert_eq!(String::from("C D E G A C"), major_pentatonic_scale.print());
//! ```

// #![deny(missing_docs)]
#![cfg_attr(test, deny(warnings))]
#![allow(clippy::module_inception)]

#[doc = include_str!("../README.md")]
mod note;
pub use note::Note;

pub mod chord;
pub use chord::{Chord, ChordQuality};

pub mod scale;
pub use scale::Scale;
