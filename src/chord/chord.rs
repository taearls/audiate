#![allow(dead_code)]
use crate::note::{Note, NotePitchInterval};

use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub struct Chord {
    name: String,
    quality: ChordQuality,
    root: Note,
}

// the chord quality a chord can be.
// it describes the root triad in a chord.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ChordQuality {
    Major,
    Minor,
    Diminished,
    Augmented,
}

impl Display for ChordQuality {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ChordQuality::Major => write!(f, "Major"),
            ChordQuality::Minor => write!(f, "Minor"),
            ChordQuality::Diminished => write!(f, "Diminished"),
            ChordQuality::Augmented => write!(f, "Augmented"),
        }
    }
}

// how to denote dominant / major seventh ?
// how to denote suspended chords?

// describes the extensions of the chord.
// each value includes the previous one.
// e.g., Ninth includes Seventh and Triad.
// TODO: more semantic name to describe this enum
pub enum ChordExtensionKind {
    Triad,
    Seventh,
    Ninth,
    Eleventh,
    Thirteenth,
}

impl Chord {
    pub fn new(root: Note, quality: ChordQuality) -> Self {
        Chord {
            name: format!("{} {}", root.name(), quality),
            quality,
            root,
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn quality(&self) -> ChordQuality {
        self.quality
    }
    pub fn root(&self) -> Note {
        self.root
    }
    pub fn third(&self) -> Note {
        let interval = match self.quality {
            ChordQuality::Major | ChordQuality::Augmented => NotePitchInterval::MajorThird,
            ChordQuality::Minor | ChordQuality::Diminished => NotePitchInterval::MinorThird,
        };
        self.root.by_interval_ascending(interval)
    }
    pub fn fifth(&self) -> Note {
        let interval = match self.quality {
            ChordQuality::Major | ChordQuality::Minor => NotePitchInterval::PerfectFifth,
            ChordQuality::Diminished => NotePitchInterval::DiminishedFifth,
            ChordQuality::Augmented => NotePitchInterval::AugmentedFifth,
        };
        self.root.by_interval_ascending(interval)
    }
}
