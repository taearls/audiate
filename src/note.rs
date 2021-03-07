use regex::Regex;
use lazy_static::lazy_static;

// TODO: be able to construct a note with just its name.
#[derive(Debug, Clone)]
pub struct Note {
  pub name: String,
  // pub letter: NoteLetter,
  pitch_value: u8,
  pitch_variant: NotePitchVariant,
  // pitch: NotePitch
}

// use this value to calculate the pitch from a letter
#[derive(Debug, Copy, Clone)]
pub enum NotePitchVariant {
  Flatdbl = -2,
  Flat = -1,
  Natural = 0,
  Sharp = 1,
  Sharpdbl = 2,
}

//  Because most songs are in the key of C,
//  that is the lowest value in this enum that everything else is based around.
//  The pitch values are relative to C being the root.
//  only use natural letter values to measure pitch. 
//  variants can describe changes from the root.
// 
//  to get Db, subtract 1 from D.
//  to get D#, add 1 to D.
// #[derive(Debug, Copy, Clone)]
// pub enum NoteLetter {
//   C = 1,
//   D = 3,
//   E = 5,
//   F = 6,
//   G = 8,
//   A = 10,
//   B = 12,
// }

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
  pub fn new(&self, note_name: &str) -> Option<Self> {
    if !self.validate_note(note_name) { return None }
    
    // TODO: now that note_name str is validated, parse pitch info
    let pitch_value: u8 = Note::get_pitch_value(note_name);
    let pitch_variant = Note::get_pitch_variant(note_name);

    Some(Note {
      name: String::from(note_name),
      pitch_value,
      pitch_variant,
    })
  }

  fn validate_note(&self, note: &str) -> bool {
    // https://docs.rs/regex/1.4.3/regex/#repetitions
    lazy_static! {
      // check if str has a-g or A-G in one occurrence
      // check for one or two flats, or one or two sharps
      static ref NOTE_REGEX: Regex = Regex::new(
        r"[a-gA-G]{1}(b{1,2})?(#{1,2})?"
      ).unwrap();
    }
    (1..=3).contains(&note.len()) && NOTE_REGEX.is_match(note)
  }

  fn get_pitch_value(note_name: &str) -> u8 {
    unimplemented!("get pitch value based on note name")
  }

  fn get_pitch_variant(note_name: &str) -> NotePitchVariant {
    unimplemented!("get pitch variant based on note name")
  }
}
