use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Clone, PartialEq)]
pub struct Note<'a> {
    pub name: &'a str,
    pitch_value: u8,
    pitch_variant: NotePitchVariant,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum NotePitchVariant {
    Flatdbl,
    Flat,
    Natural,
    Sharp,
    Sharpdbl,
}

// global static regex to parse a note from a string slice that's only compiled once
lazy_static! {
  // check if str has a-g or A-G in one occurrence
  // check for one or two flats, or one or two sharps
  static ref NOTE_REGEX: Regex = Regex::new(
    r"^(?P<note_name>(?i)[a-g]{1})(?P<note_variant>(?-i)(b{1,2})|(#{1,2}))?$"
  ).unwrap();
}

impl<'a> Note<'a> {
    pub fn new(note_name: &'a str) -> Option<Self> {
        if !Note::is_note(note_name) {
            return None;
        }

        let pitch_variant: NotePitchVariant = Note::get_pitch_variant(note_name)?;
        let pitch_value: u8 = Note::get_pitch_value(note_name, pitch_variant)?;

        Some(Note {
            name: note_name,
            pitch_value,
            pitch_variant,
        })
    }

    fn is_note(note: &str) -> bool {
        (1..=3).contains(&note.len()) && NOTE_REGEX.is_match(note)
    }

    fn get_pitch_variant(note_name: &str) -> Option<NotePitchVariant> {
        if !Note::is_note(note_name) { return None; }

        let note_variant = NOTE_REGEX.captures(note_name).and_then(|cap| {
            cap.name("note_variant")
                .map(|note_variant| note_variant.as_str())
        });
        match note_variant {
            Some("b") => Some(NotePitchVariant::Flat),
            Some("bb") => Some(NotePitchVariant::Flatdbl),
            Some("#") => Some(NotePitchVariant::Sharp),
            Some("##") => Some(NotePitchVariant::Sharpdbl),
            Some(_) => None,
            None => Some(NotePitchVariant::Natural),
        }
    }

    fn get_pitch_value(note_name: &str, pitch_variant: NotePitchVariant) -> Option<u8> {
        if !Note::is_note(note_name) { return None; }

        let note_name = NOTE_REGEX
            .captures(note_name)
            .and_then(|cap| cap.name("note_name").map(|note_name| note_name.as_str()))?;

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

        Some(note_name_pitch_value + pitch_variant)
    }
}

impl std::ops::Add<NotePitchVariant> for u8 {
    type Output = Self;
    fn add(self, other: NotePitchVariant) -> Self {
        let pitch_variant_value: i8 = match other {
            NotePitchVariant::Flatdbl => -2,
            NotePitchVariant::Flat => -1,
            NotePitchVariant::Natural => 0,
            NotePitchVariant::Sharp => 1,
            NotePitchVariant::Sharpdbl => 2,
        };

        // we add 12 to prevent underflow
        let sum = (self as i8 + 12 + pitch_variant_value) as u8;

        // we want only pitch values to be in the range of 1-12 inclusive
        match sum % 12 {
            0 => 12,
            sum => sum,
        }
    }
}

////////////////
// UNIT TESTS //
////////////////

// TODO:
// - write macros to reduce repetition
// - create helper fn to return a static str of invalid note_name values
// - test for none values

#[cfg(test)]
mod test_helper_fns {
    pub fn natural_note_name_str() -> &'static str {
        "a b c d e f g A B C D E F G"
    }
    pub fn flat_note_name_str() -> &'static str {
        "ab bb cb db eb fb gb Ab Bb Cb Db Eb Fb Gb"
    }
    pub fn flatdbl_note_name_str() -> &'static str {
        "abb bbb cbb dbb ebb fbb gbb Abb Bbb Cbb Dbb Ebb Fbb Gbb"
    }
    pub fn sharp_note_name_str() -> &'static str {
        "a# b# c# d# e# f# g# A# B# C# D# E# F# G#"
    }
    pub fn sharpdbl_note_name_str() -> &'static str {
        "a## b## c## d## e## f## g## A## B## C## D## E## F## G##"
    }
}

#[cfg(test)]
mod is_note_test {
    use super::*;

    #[test]
    fn is_note_false_when_invalid_string_passed() {
        // TODO: create helper_fn to reuse an invalid note name str
        let note = Note::is_note("");
        assert!(!note, " is not a note");
        let note = Note::is_note("Ac");
        assert!(!note, "Ac is not a note");
        let note = Note::is_note("H");
        assert!(!note, "H is not a note");
        let note = Note::is_note("Ab#");
        assert!(!note, "Ab# is not a note");
        let note = Note::is_note("Abbb");
        assert!(!note, "Abbb is not a note");
    }
    #[test]
    fn is_note_true_when_valid_string_passed_with_natural_variant() {
        for str in test_helper_fns::natural_note_name_str().split(" ") {
            let note = Note::is_note(str);
            assert!(note, "{} is a note", str);
        }
    }
    #[test]
    fn is_note_true_when_valid_string_passed_with_flat_variant() {
        for str in test_helper_fns::flat_note_name_str().split(" ") {
            let note = Note::is_note(str);
            assert!(note, "{} is a note", str);
        }
    }
    #[test]
    fn is_note_true_when_valid_string_passed_with_flatdbl_variant() {
        for str in test_helper_fns::flatdbl_note_name_str().split(" ") {
            let note = Note::is_note(str);
            assert!(note, "{} is a note", str);
        }
    }
    #[test]
    fn is_note_true_when_valid_string_passed_with_sharp_variant() {
        for str in test_helper_fns::sharp_note_name_str().split(" ") {
            let note = Note::is_note(str);
            assert!(note, "{} is a note", str);
        }
    }
    #[test]
    fn is_note_true_when_valid_string_passed_with_sharpdbl_variant() {
        for str in test_helper_fns::sharpdbl_note_name_str().split(" ") {
            let note = Note::is_note(str);
            assert!(note, "{} is a note", str);
        }
    }
}

#[cfg(test)]
mod get_pitch_variant_test {
    use super::*;

    #[test]
    fn get_pitch_variant_returns_natural() {
        for str in test_helper_fns::natural_note_name_str().split(" ") {
            let pitch_variant = Note::get_pitch_variant(str);
            assert_eq!(pitch_variant, Some(NotePitchVariant::Natural));
        }
    }
    #[test]
    fn get_pitch_variant_returns_flat() {
        for str in test_helper_fns::flat_note_name_str().split(" ") {
            let pitch_variant = Note::get_pitch_variant(str);
            assert_eq!(pitch_variant, Some(NotePitchVariant::Flat));
        }
    }
    #[test]
    fn get_pitch_variant_returns_flatdbl() {
        for str in test_helper_fns::flatdbl_note_name_str().split(" ") {
            let pitch_variant = Note::get_pitch_variant(str);
            assert_eq!(pitch_variant, Some(NotePitchVariant::Flatdbl));
        }
    }
    #[test]
    fn get_pitch_variant_returns_sharp() {
        for str in test_helper_fns::sharp_note_name_str().split(" ") {
            let pitch_variant = Note::get_pitch_variant(str);
            assert_eq!(pitch_variant, Some(NotePitchVariant::Sharp));
        }
    }
    #[test]
    fn get_pitch_variant_returns_sharpdbl() {
        for str in test_helper_fns::sharpdbl_note_name_str().split(" ") {
            let pitch_variant = Note::get_pitch_variant(str);
            assert_eq!(pitch_variant, Some(NotePitchVariant::Sharpdbl));
        } 
    }
}

#[cfg(test)]
mod get_pitch_value_test {
    use super::*;

    #[test]
    fn get_pitch_value_returns_correct_natural_variant() {
        let pitch_value = Note::get_pitch_value("a", NotePitchVariant::Natural);
        assert_eq!(pitch_value, Some(1));

        let pitch_value = Note::get_pitch_value("b", NotePitchVariant::Natural);
        assert_eq!(pitch_value, Some(3));

        let pitch_value = Note::get_pitch_value("c", NotePitchVariant::Natural);
        assert_eq!(pitch_value, Some(4));

        let pitch_value = Note::get_pitch_value("d", NotePitchVariant::Natural);
        assert_eq!(pitch_value, Some(6));

        let pitch_value = Note::get_pitch_value("e", NotePitchVariant::Natural);
        assert_eq!(pitch_value, Some(8));

        let pitch_value = Note::get_pitch_value("f", NotePitchVariant::Natural);
        assert_eq!(pitch_value, Some(9));

        let pitch_value = Note::get_pitch_value("g", NotePitchVariant::Natural);
        assert_eq!(pitch_value, Some(11));
    }

    #[test]
    fn get_pitch_value_returns_correct_flat_variant() {
        let pitch_value = Note::get_pitch_value("a", NotePitchVariant::Flat);
        assert_eq!(pitch_value, Some(12));

        let pitch_value = Note::get_pitch_value("b", NotePitchVariant::Flat);
        assert_eq!(pitch_value, Some(2));

        let pitch_value = Note::get_pitch_value("c", NotePitchVariant::Flat);
        assert_eq!(pitch_value, Some(3));

        let pitch_value = Note::get_pitch_value("d", NotePitchVariant::Flat);
        assert_eq!(pitch_value, Some(5));

        let pitch_value = Note::get_pitch_value("e", NotePitchVariant::Flat);
        assert_eq!(pitch_value, Some(7));

        let pitch_value = Note::get_pitch_value("f", NotePitchVariant::Flat);
        assert_eq!(pitch_value, Some(8));

        let pitch_value = Note::get_pitch_value("g", NotePitchVariant::Flat);
        assert_eq!(pitch_value, Some(10));
    }

    #[test]
    fn get_pitch_value_returns_correct_flatdbl_variant() {
        let pitch_value = Note::get_pitch_value("a", NotePitchVariant::Flatdbl);
        assert_eq!(pitch_value, Some(11));

        let pitch_value = Note::get_pitch_value("b", NotePitchVariant::Flatdbl);
        assert_eq!(pitch_value, Some(1));

        let pitch_value = Note::get_pitch_value("c", NotePitchVariant::Flatdbl);
        assert_eq!(pitch_value, Some(2));

        let pitch_value = Note::get_pitch_value("d", NotePitchVariant::Flatdbl);
        assert_eq!(pitch_value, Some(4));

        let pitch_value = Note::get_pitch_value("e", NotePitchVariant::Flatdbl);
        assert_eq!(pitch_value, Some(6));

        let pitch_value = Note::get_pitch_value("f", NotePitchVariant::Flatdbl);
        assert_eq!(pitch_value, Some(7));

        let pitch_value = Note::get_pitch_value("g", NotePitchVariant::Flatdbl);
        assert_eq!(pitch_value, Some(9));
    }

    #[test]
    fn get_pitch_value_returns_correct_sharp_variant() {
        let pitch_value = Note::get_pitch_value("a", NotePitchVariant::Sharp);
        assert_eq!(pitch_value, Some(2));

        let pitch_value = Note::get_pitch_value("b", NotePitchVariant::Sharp);
        assert_eq!(pitch_value, Some(4));

        let pitch_value = Note::get_pitch_value("c", NotePitchVariant::Sharp);
        assert_eq!(pitch_value, Some(5));

        let pitch_value = Note::get_pitch_value("d", NotePitchVariant::Sharp);
        assert_eq!(pitch_value, Some(7));

        let pitch_value = Note::get_pitch_value("e", NotePitchVariant::Sharp);
        assert_eq!(pitch_value, Some(9));

        let pitch_value = Note::get_pitch_value("f", NotePitchVariant::Sharp);
        assert_eq!(pitch_value, Some(10));

        let pitch_value = Note::get_pitch_value("g", NotePitchVariant::Sharp);
        assert_eq!(pitch_value, Some(12));
    }

    #[test]
    fn get_pitch_value_returns_correct_sharpdbl_variant() {
        let pitch_value = Note::get_pitch_value("a", NotePitchVariant::Sharpdbl);
        assert_eq!(pitch_value, Some(3));

        let pitch_value = Note::get_pitch_value("b", NotePitchVariant::Sharpdbl);
        assert_eq!(pitch_value, Some(5));

        let pitch_value = Note::get_pitch_value("c", NotePitchVariant::Sharpdbl);
        assert_eq!(pitch_value, Some(6));

        let pitch_value = Note::get_pitch_value("d", NotePitchVariant::Sharpdbl);
        assert_eq!(pitch_value, Some(8));

        let pitch_value = Note::get_pitch_value("e", NotePitchVariant::Sharpdbl);
        assert_eq!(pitch_value, Some(10));

        let pitch_value = Note::get_pitch_value("f", NotePitchVariant::Sharpdbl);
        assert_eq!(pitch_value, Some(11));

        let pitch_value = Note::get_pitch_value("g", NotePitchVariant::Sharpdbl);
        assert_eq!(pitch_value, Some(1));
    }
}
