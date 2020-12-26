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

// TODO: be able to construct a note with just its name.
#[derive(Debug, Clone)]
pub struct Note {
  pub name: String,
  pub letter: NoteLetter,
  pub variant: NotePitchVariant,
  // pitch: NotePitch
}

// use this value to calculate the pitch from a letter
#[derive(Debug, Copy, Clone)]
pub enum NotePitchVariant {
  // Flatdbl = -2,
  Flat = -1,
  Natural = 0,
  Sharp = 1,
  // Sharpdbl = 2,
}

//  Because most songs are in the key of C,
//  that is the lowest value in this enum that everything else is based around.
//  The pitch values are relative to C being the root.
//  only use natural letter values to measure pitch. 
//  variants can describe changes from the root.
// 
//  to get Db, subtract 1 from D.
//  to get D#, add 1 to D.
#[derive(Debug, Copy, Clone)]
pub enum NoteLetter {
  C = 1,
  D = 3,
  E = 5,
  F = 6,
  G = 8,
  A = 10,
  B = 12,
}

// Note should have
// letter -> A - G with numeric values
// variant -> 
// flat - natural - sharp - double flat - double sharp

// pitch value -> calculated field -> 1-12
// letter + variant -> pitch value

// name -> String representation derived from other info

// cache intervals


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

impl Note {
  // TODO: make pitchvariant optional in initializer? 
  pub fn new(note_name: &str, variant: NotePitchVariant) -> Note {
    let name = match note_name {
      // todo: refactor this to use a constant
      "A" | "B" | "C" | "D" | "E" | "F" | "G"  => String::from(note_name),
      _ => panic!("{} is not a valid note name", note_name),
    };

    let letter = match name.as_str() {
      "A" => NoteLetter::A,
      "B" => NoteLetter::B,
      "C" => NoteLetter::C,
      "D" => NoteLetter::D,
      "E" => NoteLetter::E,
      "F" => NoteLetter::F,
      "G" => NoteLetter::G,
      _ => panic!("{} is not a valid note letter", name),
    };

    Note {
      name,
      letter,
      variant,
    }
  }
  // fn set_pitch(note: &self) -> () {
  //   note.pitch = note.letter + note.variant 
  // }
}

// get the name of a chord. for now it just returns the name of the root note without any additional description. 
fn get_chord_name(root: &Note) -> String {
  String::from(&root.name)
}

// find a note in relation to another by using semitones.
// find_interval(C, 4) -> E
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