pub struct Chord {
  pub name: String,
  quality: ChordQuality,
  root: Note,
  third: Note,
  fifth: Note,
  seventh: Note,
}

pub enum ChordQuality {
  Major,
  Minor,
  Diminished,
  Augmented,
}

pub enum ChordKind {
  Triad,
}

// TODO: be able to construct a note with just its name.
pub struct Note {
  name: String,
  // pitch: i8, 
  // variant: PitchVariant,
}

// use this value to calculate the pitch from a letter
#[derive(Debug, Copy, Clone)]
pub enum PitchVariant {
  flatdbl = -2,
  flat = -1,
  natural = 0,
  sharp = 1,
  sharpdbl = 2,
}


//  Because most songs are in the key of C,
//  that is the lowest value in this enum that everything else is based around.
//  The pitch values are relative to C being the root.
//  only use natural letter values to measure pitch. 
//  variants can describe changes from the root.
// 
//  to get Db, subtract 1 from D.
//  to get D#, add 1 to D.
enum MusicNote {
  C = 1,
  D = 3,
  E = 5,
  F = 6,
  G = 8,
  A = 10,
  B = 12,
}

impl Chord {
  pub fn new(root: Note, quality: ChordQuality, kind: ChordKind) -> Chord {
    let name = get_chord_name(&root);
    let third = find_interval(&root, 4);
    let fifth = find_interval(&root, 7);
    let seventh = find_interval(&root, 10);
    Chord {
      name,
      quality,
      root,
      third,
      fifth,
      seventh,
    }
  }
}

impl Note {
  pub fn new(note_name: &str) -> Note {
    let name = String::from(note_name);
    Note {
      name,
    }
  }
}

// get the name of a chord. for now it just returns the name of the root note without any additional description. 
fn get_chord_name(root: &Note) -> String {
  String::from(&root.name)
}

// find a note in relation to another by using semitones.
// find_interval(C, 4) -> E
fn find_interval(root: &Note, semitones: i8) -> Note {
  let name = String::from(&root.name);
  // let pitch = root.pitch;
  // let variant = root.variant;
  Note {
    name,
    // pitch,
    // variant,
  }
}