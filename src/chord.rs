use crate::note::Note;

pub struct Chord<'a> {
    pub name: &'a str,
    quality: ChordQuality,
    root: Note<'a>,
}

// the chord quality a chord can be.
// it describes the root triad in a chord.
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

impl<'a> Chord<'a> {
    // pub fn new(root: &str, quality: ChordQuality) -> Result<Chord, &str> {
      // Chord {
      //   name,
      //   quality,
      //   root
      // }
    // }
    fn get_chord_name(root: &'a Note) -> &'a str {
        &root.name
    }
}
