#![allow(dead_code)]
use crate::note::Note;

pub struct Scale<const N: usize> {
    notes: [Note; N],
    name: ScaleType,
}

pub enum ScaleType {
    Ionian,
    Dorian,
    Phrygian,
    Lydian,
    Mixolydian,
    Aeolian,
    Locrian,
    Major,
    Minor,
    MajorPentatonic,
    MinorPentatonic,
    HarmonicMinor,
    MelodicMinor,
    HalfWhole,
    WholeHalf,
    WholeTone,
}

impl<const N: usize> Scale<N> {
    pub fn new(root_note: Note, name: ScaleType) -> Self {
        Self {
            notes: [root_note; N],
            name,
        }
    }
}
