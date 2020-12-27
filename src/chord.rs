use crate::note::{Note, NoteLetter};

pub struct Chord {
  pub name: String,
  quality: ChordQuality,
  root: Note,
  third: Note,
  fifth: Note,
  seventh: Option<Note>,
  ninth: Option<Note>,
  eleventh: Option<Note>,
  thirteenth: Option<Note>,
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

impl Chord {
  pub fn new(root: Note, quality: ChordQuality, extension_kind: Option<ChordExtensionKind>) -> Chord {
    let name = get_chord_name(&root);

    // TODO: add fn for grabbing triad + seventh notes
    let third = find_interval(&root, 4);
    let fifth = find_interval(&root, 7);
    let seventh = match extension_kind {
      None => None,
      _ => Some(find_interval(&root, 10)),
    };

    // TODO: add fn for grabbing extension notes
    let ninth = match extension_kind {
      None => None,
      // semitones for interval need to be dynamic
      _ => Some(find_interval(&root, 2)),
    };
    let eleventh = match extension_kind {
      None | Some(ChordExtensionKind::Ninth) => None,
      // semitones for interval need to be dynamic
      _ => Some(find_interval(&root, 5)),
    };
    let thirteenth = match extension_kind {
      None | Some(ChordExtensionKind::Ninth) | Some(ChordExtensionKind::Eleventh) => None,
      // semitones for interval need to be dynamic
      _ => Some(find_interval(&root, 7)),
    };

    Chord {
      name,
      quality,
      root,
      third,
      fifth,
      seventh,
      ninth,
      eleventh,
      thirteenth,
    }
  }
}

// get the name of a chord. for now it just returns the name of the root note without any additional description. 
fn get_chord_name(root: &Note) -> String {
  String::from(&root.name)
}

fn find_interval(root: &Note, semitones: i8) -> Note {
  let name = String::from(&root.name);
  let letter = root.letter;
  // let pitch = root.pitch;
  let variant = root.variant;
  Note {
    name,
    letter,
    variant,
  }
}