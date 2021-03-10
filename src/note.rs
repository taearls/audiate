use regex::Regex;
use lazy_static::lazy_static;

// TODO: be able to construct a note with just its name.
#[derive(Debug, Clone, PartialEq)]
pub struct Note<'a> {
  pub name: &'a str,
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

impl<'a> Note<'a> {
  pub fn new(note_name: &'a str) -> Option<Self> {
    if !Note::validate_note(note_name) { return None }
    
    let pitch_variant: Option<NotePitchVariant> = Note::get_pitch_variant(note_name);
    if pitch_variant == None { return None; }

    let pitch_value: Option<u8> = Note::get_pitch_value(note_name, pitch_variant.unwrap());
    if pitch_value == None { return None; }

    Some(Note {
      name: note_name,
      pitch_value: pitch_value.unwrap(),
      pitch_variant: pitch_variant.unwrap(),
    })
  }

  fn validate_note(note: &str) -> bool {    
    (1..=3).contains(&note.len()) && NOTE_REGEX.is_match(note)
  }

  fn get_pitch_variant(note_name: &str) -> Option<NotePitchVariant> {
    let note_variant = NOTE_REGEX.captures(note_name).and_then(|cap| {
      cap.name("note_variant").map(|note_variant| note_variant.as_str())
    });
    match note_variant {
      Some("b")  => Some(NotePitchVariant::Flat),
      Some("bb") => Some(NotePitchVariant::Flatdbl),
      Some("#")  => Some(NotePitchVariant::Sharp),
      Some("##") => Some(NotePitchVariant::Sharpdbl),
      Some(_)    => None,
      None       => Some(NotePitchVariant::Natural),
    }
  }

  fn get_pitch_value(note_name: &str, pitch_variant: NotePitchVariant) -> Option<u8> {
    let note_name = NOTE_REGEX.captures(note_name).and_then(|cap| {
      cap.name("note_name").map(|note_name| note_name.as_str())
    }).unwrap();

    // cycle through from 1-12
    let note_name_pitch_value: u8 = match note_name.to_lowercase().as_str() {
      "a" => 1,
      "b" => 3,
      "c" => 4,
      "d" => 6,
      "e" => 8,
      "f" => 9,
      "g" => 11,
      _ => return None,
    };

    // we add 12 to prevent underflow
    let temp = note_name_pitch_value + 12 + pitch_variant;
    match temp % 12 {
      0 => Some(12),
      result => Some(result),
    }
  }
}

impl std::ops::Add<NotePitchVariant> for u8 {
  type Output = Self;
  fn add(self, other: NotePitchVariant) -> Self {
    self + other
  }
} 










// UNIT TESTS

// TODO: write macros to reduce repetition

#[cfg(test)]
mod validate_note_test {
  use super::*;

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

#[cfg(test)]
mod get_pitch_variant_test {
  use super::*;

  #[test]
  fn get_pitch_variant_returns_natural() {
    let pitch_variant = Note::get_pitch_variant("A");
    assert_eq!(pitch_variant, Some(NotePitchVariant::Natural));

    let pitch_variant = Note::get_pitch_variant("B");
    assert_eq!(pitch_variant, Some(NotePitchVariant::Natural));

    let pitch_variant = Note::get_pitch_variant("C");
    assert_eq!(pitch_variant, Some(NotePitchVariant::Natural));

    let pitch_variant = Note::get_pitch_variant("D");
    assert_eq!(pitch_variant, Some(NotePitchVariant::Natural));

    let pitch_variant = Note::get_pitch_variant("E");
    assert_eq!(pitch_variant, Some(NotePitchVariant::Natural));

    let pitch_variant = Note::get_pitch_variant("F");
    assert_eq!(pitch_variant, Some(NotePitchVariant::Natural));

    let pitch_variant = Note::get_pitch_variant("G");
    assert_eq!(pitch_variant, Some(NotePitchVariant::Natural));
  }

  #[test]
  fn get_pitch_variant_returns_flat() {
    let pitch_variant = Note::get_pitch_variant("Ab");
    assert_eq!(pitch_variant, Some(NotePitchVariant::Flat));

    let pitch_variant = Note::get_pitch_variant("Bb");
    assert_eq!(pitch_variant, Some(NotePitchVariant::Flat));

    let pitch_variant = Note::get_pitch_variant("Cb");
    assert_eq!(pitch_variant, Some(NotePitchVariant::Flat));

    let pitch_variant = Note::get_pitch_variant("Db");
    assert_eq!(pitch_variant, Some(NotePitchVariant::Flat));

    let pitch_variant = Note::get_pitch_variant("Eb");
    assert_eq!(pitch_variant, Some(NotePitchVariant::Flat));

    let pitch_variant = Note::get_pitch_variant("Fb");
    assert_eq!(pitch_variant, Some(NotePitchVariant::Flat));

    let pitch_variant = Note::get_pitch_variant("Gb");
    assert_eq!(pitch_variant, Some(NotePitchVariant::Flat));
  }

  #[test]
  fn get_pitch_variant_returns_flatdbl() {
    let pitch_variant = Note::get_pitch_variant("Abb");
    assert_eq!(pitch_variant, Some(NotePitchVariant::Flatdbl));

    let pitch_variant = Note::get_pitch_variant("Bbb");
    assert_eq!(pitch_variant, Some(NotePitchVariant::Flatdbl));

    let pitch_variant = Note::get_pitch_variant("Cbb");
    assert_eq!(pitch_variant, Some(NotePitchVariant::Flatdbl));

    let pitch_variant = Note::get_pitch_variant("Dbb");
    assert_eq!(pitch_variant, Some(NotePitchVariant::Flatdbl));

    let pitch_variant = Note::get_pitch_variant("Ebb");
    assert_eq!(pitch_variant, Some(NotePitchVariant::Flatdbl));

    let pitch_variant = Note::get_pitch_variant("Fbb");
    assert_eq!(pitch_variant, Some(NotePitchVariant::Flatdbl));

    let pitch_variant = Note::get_pitch_variant("Gbb");
    assert_eq!(pitch_variant, Some(NotePitchVariant::Flatdbl));
  }

  #[test]
  fn get_pitch_variant_returns_sharp() {
    let pitch_variant = Note::get_pitch_variant("A#");
    assert_eq!(pitch_variant, Some(NotePitchVariant::Sharp));

    let pitch_variant = Note::get_pitch_variant("B#");
    assert_eq!(pitch_variant, Some(NotePitchVariant::Sharp));

    let pitch_variant = Note::get_pitch_variant("C#");
    assert_eq!(pitch_variant, Some(NotePitchVariant::Sharp));

    let pitch_variant = Note::get_pitch_variant("D#");
    assert_eq!(pitch_variant, Some(NotePitchVariant::Sharp));

    let pitch_variant = Note::get_pitch_variant("E#");
    assert_eq!(pitch_variant, Some(NotePitchVariant::Sharp));

    let pitch_variant = Note::get_pitch_variant("F#");
    assert_eq!(pitch_variant, Some(NotePitchVariant::Sharp));

    let pitch_variant = Note::get_pitch_variant("G#");
    assert_eq!(pitch_variant, Some(NotePitchVariant::Sharp));
  }

  #[test]
  fn get_pitch_variant_returns_sharpdbl() {
    let pitch_variant = Note::get_pitch_variant("A##");
    assert_eq!(pitch_variant, Some(NotePitchVariant::Sharpdbl));

    let pitch_variant = Note::get_pitch_variant("B##");
    assert_eq!(pitch_variant, Some(NotePitchVariant::Sharpdbl));

    let pitch_variant = Note::get_pitch_variant("C##");
    assert_eq!(pitch_variant, Some(NotePitchVariant::Sharpdbl));

    let pitch_variant = Note::get_pitch_variant("D##");
    assert_eq!(pitch_variant, Some(NotePitchVariant::Sharpdbl));

    let pitch_variant = Note::get_pitch_variant("E##");
    assert_eq!(pitch_variant, Some(NotePitchVariant::Sharpdbl));

    let pitch_variant = Note::get_pitch_variant("F##");
    assert_eq!(pitch_variant, Some(NotePitchVariant::Sharpdbl));

    let pitch_variant = Note::get_pitch_variant("G##");
    assert_eq!(pitch_variant, Some(NotePitchVariant::Sharpdbl));
  }
}
