use regex::Regex;
use lazy_static::lazy_static;

// TODO: be able to construct a note with just its name.
#[derive(Debug, Clone, PartialEq)]
pub struct Note {
  pub name: String,
  // pub letter: NoteLetter,
  pitch_value: u8,
  pitch_variant: NotePitchVariant,
  // pitch: NotePitch
}

// use this value to calculate the pitch from a letter
#[derive(Debug, Copy, Clone, PartialEq)]
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

lazy_static! {
  // check if str has a-g or A-G in one occurrence
  // check for one or two flats, or one or two sharps
  static ref NOTE_REGEX: Regex = Regex::new(
    r"^(?P<note_name>(?i)[a-g]{1})(?P<note_variant>(?-i)(b{1,2})|(#{1,2}))?$"
  ).unwrap();
}

// TODO: calculate pitch value 1 - 12 and cache it
impl Note {
  // TODO: make pitchvariant optional in initializer? 
  pub fn new(note_name: &str) -> Option<Self> {
    if !Note::validate_note(note_name) { return None }
    
    // TODO: now that note_name str is validated, parse pitch info
    let pitch_value: u8 = Note::get_pitch_value(note_name);
    let pitch_variant = Note::get_pitch_variant(note_name);

    Some(Note {
      name: String::from(note_name),
      pitch_value,
      pitch_variant,
    })
  }

  fn validate_note(note: &str) -> bool {
    // https://docs.rs/regex/1.4.3/regex/#repetitions
    
    (1..=3).contains(&note.len()) && NOTE_REGEX.is_match(note)
  }

  fn get_pitch_value(note_name: &str) -> u8 {
    unimplemented!("get pitch value based on note name")
  }

  fn get_pitch_variant(note_name: &str) -> NotePitchVariant {
    unimplemented!("get pitch variant based on note name")
  }
}













// UNIT TESTS

#[cfg(test)]
mod validate_note_test {
  use super::Note;

  #[test]
  fn validate_note_false_when_invalid_string_passed() {
    let note = Note::validate_note("");
    assert!(!note);
    let note = Note::validate_note("Ac");
    assert!(!note);
    let note = Note::validate_note("H");
    assert!(!note);
    let note = Note::validate_note("Ab#");
    assert!(!note);
    let note = Note::validate_note("Abbb");
    assert!(!note);
  }
  #[test]
  fn validate_note_true_when_valid_string_passed_without_variant() {
    let note = Note::validate_note("a");
    assert!(note);
    let note = Note::validate_note("A");
    assert!(note);
    let note = Note::validate_note("b");
    assert!(note);
    let note = Note::validate_note("B");
    assert!(note);
    let note = Note::validate_note("c");
    assert!(note);
    let note = Note::validate_note("C");
    assert!(note);
    let note = Note::validate_note("d");
    assert!(note);
    let note = Note::validate_note("D");
    assert!(note);
    let note = Note::validate_note("e");
    assert!(note);
    let note = Note::validate_note("E");
    assert!(note);
    let note = Note::validate_note("f");
    assert!(note);
    let note = Note::validate_note("F");
    assert!(note);
    let note = Note::validate_note("g");
    assert!(note);
    let note = Note::validate_note("G");
    assert!(note);
  }

  #[test]
  fn validate_note_true_when_valid_string_passed_with_variant() {
    let note = Note::validate_note("ab");
    assert!(note);
    let note = Note::validate_note("abb");
    assert!(note);
    let note = Note::validate_note("a#");
    assert!(note);
    let note = Note::validate_note("a##");
    assert!(note);

    let note = Note::validate_note("bb");
    assert!(note);
    let note = Note::validate_note("bbb");
    assert!(note);
    let note = Note::validate_note("b#");
    assert!(note);
    let note = Note::validate_note("b##");
    assert!(note);

    let note = Note::validate_note("bb");
    assert!(note);
    let note = Note::validate_note("bbb");
    assert!(note);
    let note = Note::validate_note("b#");
    assert!(note);
    let note = Note::validate_note("b##");
    assert!(note);

    let note = Note::validate_note("cb");
    assert!(note);
    let note = Note::validate_note("cbb");
    assert!(note);
    let note = Note::validate_note("c#");
    assert!(note);
    let note = Note::validate_note("c##");
    assert!(note);

    let note = Note::validate_note("db");
    assert!(note);
    let note = Note::validate_note("dbb");
    assert!(note);
    let note = Note::validate_note("d#");
    assert!(note);
    let note = Note::validate_note("d##");
    assert!(note);

    let note = Note::validate_note("eb");
    assert!(note);
    let note = Note::validate_note("ebb");
    assert!(note);
    let note = Note::validate_note("e#");
    assert!(note);
    let note = Note::validate_note("e##");
    assert!(note);

    let note = Note::validate_note("fb");
    assert!(note);
    let note = Note::validate_note("fbb");
    assert!(note);
    let note = Note::validate_note("f#");
    assert!(note);
    let note = Note::validate_note("f##");
    assert!(note);

    let note = Note::validate_note("gb");
    assert!(note);
    let note = Note::validate_note("gbb");
    assert!(note);
    let note = Note::validate_note("g#");
    assert!(note);
    let note = Note::validate_note("g##");
    assert!(note);
  }
}