

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

// TODO: calculate pitch value 1 - 12 and cache it
impl Note {
  // TODO: make pitchvariant optional in initializer? 
  pub fn new(note_name: &str, variant: NotePitchVariant) -> Note {
    let name = match note_name {

      // todo: refactor this to use a constant or regular expression
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
