use crate::note::Note;

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

// how to denote dominant / major seventh ?
// how to denote suspended chords?
//

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
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn quality(&self) -> ChordQuality {
        self.quality
    }
    pub fn root(&self) -> &str {
        &self.root.name()
    }
}
