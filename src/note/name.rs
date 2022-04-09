use std::fmt::{Display, Formatter};

use super::interval::NotePitchInterval;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref NOTE_PITCH_NAMES: [NotePitchName; 7] = [
        NotePitchName::A,
        NotePitchName::B,
        NotePitchName::C,
        NotePitchName::D,
        NotePitchName::E,
        NotePitchName::F,
        NotePitchName::G,
    ];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NotePitchName {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl NotePitchName {
    pub fn by_interval(&self, pitch_interval: NotePitchInterval) -> NotePitchName {
        use NotePitchInterval::*;

        let original_idx = NOTE_PITCH_NAMES
            .iter()
            .position(|&name: &NotePitchName| name == *self)
            .unwrap();
        let interval_index: usize = match pitch_interval {
            PerfectUnison => 0,
            MinorSecond | MajorSecond | AugmentedSecond => 1,
            MinorThird | MajorThird => 2,
            PerfectFourth | AugmentedFourth => 3,
            DiminishedFifth | PerfectFifth => 4,
            MinorSixth | MajorSixth => 5,
            DiminishedSeventh | MinorSeventh | MajorSeventh => 6,
        };
        let new_index = (original_idx + interval_index) % NOTE_PITCH_NAMES.len();
        NOTE_PITCH_NAMES[new_index]
    }
}

impl Display for NotePitchName {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        use NotePitchName::*;
        let name: char = match self {
            A => 'A',
            B => 'B',
            C => 'C',
            D => 'D',
            E => 'E',
            F => 'F',
            G => 'G',
        };
        write!(f, "{name}")
    }
}

impl TryFrom<&str> for NotePitchName {
    type Error = String;

    fn try_from(name: &str) -> Result<Self, Self::Error> {
        use NotePitchName::*;
        match name.to_uppercase().chars().next() {
            Some('A') => Ok(A),
            Some('B') => Ok(B),
            Some('C') => Ok(C),
            Some('D') => Ok(D),
            Some('E') => Ok(E),
            Some('F') => Ok(F),
            Some('G') => Ok(G),
            Some(c) => Err(format!("{c} is not a valid note pitch name")),
            None => Err("An empty &str is not valid".to_string()),
        }
    }
}

impl TryFrom<&String> for NotePitchName {
    type Error = String;

    fn try_from(name: &String) -> Result<Self, Self::Error> {
        use NotePitchName::*;
        match name.to_uppercase().chars().next() {
            Some('A') => Ok(A),
            Some('B') => Ok(B),
            Some('C') => Ok(C),
            Some('D') => Ok(D),
            Some('E') => Ok(E),
            Some('F') => Ok(F),
            Some('G') => Ok(G),
            Some(c) => Err(format!("{c} is not a valid note pitch name")),
            None => Err("An empty &String is not valid".to_string()),
        }
    }
}

impl From<NotePitchName> for u8 {
    fn from(note_pitch_name: NotePitchName) -> u8 {
        use NotePitchName::*;
        match note_pitch_name {
            A => 1,
            B => 3,
            C => 4,
            D => 6,
            E => 8,
            F => 9,
            G => 11,
        }
    }
}

impl From<NotePitchName> for &str {
    fn from(note_pitch_name: NotePitchName) -> &'static str {
        use NotePitchName::*;
        match note_pitch_name {
            A => "A",
            B => "B",
            C => "C",
            D => "D",
            E => "E",
            F => "F",
            G => "G",
        }
    }
}

// global static regex to parse a note from a string slice that's only compiled once
lazy_static! {
  // check if str has a-g or A-G in one occurrence
  // check for one or two flats, or one or two sharps
  static ref NOTE_REGEX: Regex = Regex::new(
    r"^(?P<note_name>(?i)[a-g]{1})(?P<note_variant>(?-i)(b{1,2})|(#{1,2}))?$"
  ).unwrap();
}

pub fn is_note_name_valid(note_name: &str) -> bool {
    (1..=3).contains(&note_name.len()) && NOTE_REGEX.is_match(note_name)
}

pub fn note_variant_from_note_name(note_name: &str) -> Option<&str> {
    NOTE_REGEX.captures(note_name).and_then(|cap| {
        cap.name("note_variant")
            .map(|note_variant| note_variant.as_str())
    })
}

////////////////
// UNIT TESTS //
////////////////

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
mod is_note_name_valid_test {
    use super::*;

    #[test]
    fn is_note_name_valid_false_when_invalid_string_passed() {
        let note = is_note_name_valid("");
        assert!(!note, " is not a note");
        let note = is_note_name_valid("Ac");
        assert!(!note, "Ac is not a note");
        let note = is_note_name_valid("H");
        assert!(!note, "H is not a note");
        let note = is_note_name_valid("Ab#");
        assert!(!note, "Ab# is not a note");
        let note = is_note_name_valid("Abbb");
        assert!(!note, "Abbb is not a note");
    }
    #[test]
    fn is_note_true_when_valid_string_passed_with_natural_variant() {
        for str in test_helper_fns::natural_note_name_str().split(' ') {
            let note = is_note_name_valid(str);
            assert!(note, "{} is a note", str);
        }
    }
    #[test]
    fn is_note_true_when_valid_string_passed_with_flat_variant() {
        for str in test_helper_fns::flat_note_name_str().split(' ') {
            let note = is_note_name_valid(str);
            assert!(note, "{} is a note", str);
        }
    }
    #[test]
    fn is_note_true_when_valid_string_passed_with_flatdbl_variant() {
        for str in test_helper_fns::flatdbl_note_name_str().split(' ') {
            let note = is_note_name_valid(str);
            assert!(note, "{} is a note", str);
        }
    }
    #[test]
    fn is_note_true_when_valid_string_passed_with_sharp_variant() {
        for str in test_helper_fns::sharp_note_name_str().split(' ') {
            let note = is_note_name_valid(str);
            assert!(note, "{} is a note", str);
        }
    }
    #[test]
    fn is_note_true_when_valid_string_passed_with_sharpdbl_variant() {
        for str in test_helper_fns::sharpdbl_note_name_str().split(' ') {
            let note = is_note_name_valid(str);
            assert!(note, "{} is a note", str);
        }
    }
}
