#![allow(dead_code)]
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, PartialEq)]
pub struct Note {
    name: String,
    pitch_value: u8,
    pitch_variant: NotePitchVariant,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NotePitchVariant {
    Flatdbl,
    Flat,
    Natural,
    Sharp,
    Sharpdbl,
}

impl Display for NotePitchVariant {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let name: &str = match self {
            NotePitchVariant::Flatdbl => "double flat",
            NotePitchVariant::Flat => "flat",
            NotePitchVariant::Natural => "natural",
            NotePitchVariant::Sharp => "sharp",
            NotePitchVariant::Sharpdbl => "double sharp",
        };
        write!(f, "{}", name)
    }
}

pub enum NotePitchName {
    A(Note),
    B(Note),
    C(Note),
    D(Note),
    E(Note),
    F(Note),
    G(Note),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NoteIntervalDirection {
    Ascending,
    Descending,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NotePitchInterval {
    PerfectUnison,
    MinorSecond,
    MajorSecond,
    MinorThird,
    MajorThird,
    PerfectFourth,
    AugmentedFourth,
    DiminishedFifth,
    PerfectFifth,
    MinorSixth,
    MajorSixth,
    MinorSeventh,
    MajorSeventh,
    // Octave,
    // MinorNinth,
    // MajorNinth,
    // MinorTenth,
    // MajorTenth,
    // MinorEleventh,
    // MajorEleventh,
    // MinorTwelfth,
    // MajorTwelfth,
    // MinorThirteenth,
    // MajorThirteenth,
}

impl NotePitchInterval {
    // returns the inverted pitch if you change between an ascending and descending interval
    fn invert(&self) -> NotePitchInterval {
        match self {
            NotePitchInterval::PerfectUnison => NotePitchInterval::PerfectUnison,
            NotePitchInterval::MinorSecond => NotePitchInterval::MajorSeventh,
            NotePitchInterval::MajorSecond => NotePitchInterval::MinorSeventh,
            NotePitchInterval::MinorThird => NotePitchInterval::MajorSixth,
            NotePitchInterval::MajorThird => NotePitchInterval::MinorSixth,
            NotePitchInterval::PerfectFourth => NotePitchInterval::PerfectFifth,
            NotePitchInterval::AugmentedFourth => NotePitchInterval::AugmentedFourth,
            NotePitchInterval::DiminishedFifth => NotePitchInterval::DiminishedFifth,
            NotePitchInterval::PerfectFifth => NotePitchInterval::PerfectFourth,
            NotePitchInterval::MinorSixth => NotePitchInterval::MajorThird,
            NotePitchInterval::MajorSixth => NotePitchInterval::MinorThird,
            NotePitchInterval::MinorSeventh => NotePitchInterval::MajorSecond,
            NotePitchInterval::MajorSeventh => NotePitchInterval::MinorSecond,
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

lazy_static! {
    static ref NOTE_NAME_TO_PITCH: HashMap<String, u8> = HashMap::from([
        (String::from("A"), 1),
        (String::from("B"), 3),
        (String::from("C"), 4),
        (String::from("D"), 6),
        (String::from("E"), 8),
        (String::from("F"), 9),
        (String::from("G"), 11),
    ]);
}

lazy_static! {
    static ref PITCH_TO_NOTE_NAME: HashMap<u8, String> = HashMap::from([
        (1, String::from("A")),
        (3, String::from("B")),
        (4, String::from("C")),
        (6, String::from("D")),
        (8, String::from("E")),
        (9, String::from("F")),
        (11, String::from("G")),
    ]);
}

lazy_static! {
    static ref NOTE_INTERVAL_TO_PITCH_VALUE: HashMap<NotePitchInterval, u8> = HashMap::from([
        (NotePitchInterval::PerfectUnison, 0),
        (NotePitchInterval::MinorSecond, 1),
        (NotePitchInterval::MajorSecond, 2),
        (NotePitchInterval::MinorThird, 3),
        (NotePitchInterval::MajorThird, 4),
        (NotePitchInterval::PerfectFourth, 5),
        (NotePitchInterval::AugmentedFourth, 6),
        (NotePitchInterval::DiminishedFifth, 6),
        (NotePitchInterval::PerfectFifth, 7),
        (NotePitchInterval::MinorSixth, 8),
        (NotePitchInterval::MajorSixth, 9),
        (NotePitchInterval::MinorSeventh, 10),
        (NotePitchInterval::MajorSeventh, 11),
    ]);
}

impl Note {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn pitch_value(&self) -> u8 {
        self.pitch_value
    }

    pub fn pitch_variant(&self) -> NotePitchVariant {
        self.pitch_variant
    }

    // TODO: accept String and &str in this constructor fn
    // TODO: add unit tests to accept &str and String
    // https://hermanradtke.com/2015/05/06/creating-a-rust-function-that-accepts-string-or-str.html
    pub fn with_name<T: Display>(note_name: T) -> Option<Self> {
        // TODO: refactor to return an Err variant of some kind.
        let note_name_str: String = note_name.to_string().to_uppercase();
        if !Note::is_note_name_valid(&note_name_str) {
            return None;
        }

        let pitch_variant: NotePitchVariant = calc_pitch_variant(&note_name_str)?;
        let pitch_value: u8 = calc_pitch_value(&note_name_str, pitch_variant)?;

        Some(Note {
            name: note_name_str,
            pitch_value,
            pitch_variant,
        })
    }

    pub fn with_pitch(value: u8, variant: NotePitchVariant) -> Option<Self> {
        unimplemented!("Create a new Note from the pitch value {value} and its variant {variant}");
    }

    fn is_note_name_valid(note_name: &str) -> bool {
        (1..=3).contains(&note_name.len()) && NOTE_REGEX.is_match(note_name)
    }

    fn by_interval(&self, interval: NotePitchInterval) -> Option<Note> {
        let note_name = calc_name_by_interval(&self.name, interval);
        if note_name.is_none() {
            return None;
        }
        Some(Note {
            name: note_name.unwrap(),
            pitch_value: calc_pitch_value_from_interval(self.pitch_value, interval),
            pitch_variant: self.pitch_variant,
        })
    }
}

fn calc_name_by_interval(note_name: &str, pitch_interval: NotePitchInterval) -> Option<String> {
    let note_name = note_name.to_uppercase();
    if !Note::is_note_name_valid(&note_name) {
        return None;
    }
    let names: [char; 7] = ['A', 'B', 'C', 'D', 'E', 'F', 'G'];
    // let mut result = String::new();
    // if let Some(interval_value) = NOTE_INTERVAL_TO_PITCH_VALUE.get(&pitch_interval) {
    let original_idx = names
        .iter()
        .position(|&name| name.to_string() == note_name)
        .unwrap();
    let interval_index: usize = match pitch_interval {
        NotePitchInterval::PerfectUnison => 0,
        NotePitchInterval::MinorSecond | NotePitchInterval::MajorSecond => 1,
        NotePitchInterval::MinorThird | NotePitchInterval::MajorThird => 2,
        NotePitchInterval::PerfectFourth | NotePitchInterval::AugmentedFourth => 3,
        NotePitchInterval::DiminishedFifth | NotePitchInterval::PerfectFifth => 4,
        NotePitchInterval::MinorSixth | NotePitchInterval::MajorSixth => 5,
        NotePitchInterval::MinorSeventh | NotePitchInterval::MajorSeventh => 6,
    };
    let new_index = (original_idx + interval_index) % names.len();
    let result = String::from(names[new_index]);
    // }

    Some(result)
}

fn calc_pitch_value(note_name: &str, pitch_variant: NotePitchVariant) -> Option<u8> {
    if !Note::is_note_name_valid(note_name) {
        return None;
    }

    let note_name = NOTE_REGEX
        .captures(note_name)
        .and_then(|cap| cap.name("note_name").map(|note_name| note_name.as_str()));
    let note_name = match note_name {
        Some(note_name) => note_name,
        None => return None,
    };

    let note_name_pitch_value = match NOTE_NAME_TO_PITCH.get(&note_name.to_uppercase()) {
        Some(pitch_value) => *pitch_value,
        None => return None,
    };

    Some(note_name_pitch_value + pitch_variant)
}

fn calc_pitch_value_from_interval(pitch_value: u8, interval: NotePitchInterval) -> u8 {
    pitch_value + interval
}

fn calc_pitch_variant(note_name: &str) -> Option<NotePitchVariant> {
    if !Note::is_note_name_valid(note_name) {
        return None;
    }

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

impl From<&str> for Note {
    fn from(name: &str) -> Self {
        Note::with_name(name).unwrap()
    }
}

impl From<String> for Note {
    fn from(name: String) -> Self {
        Note::with_name(&name).unwrap()
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

        // we want only pitch values to be in the range of 0-11 inclusive
        sum % 12
    }
}

impl std::ops::Add<NotePitchInterval> for u8 {
    type Output = Self;
    fn add(self, other: NotePitchInterval) -> Self {
        // TODO: handle descending intervals.
        let pitch_interval_value: u8 = match other {
            NotePitchInterval::PerfectUnison => 0,
            NotePitchInterval::MinorSecond => 1,
            NotePitchInterval::MajorSecond => 2,
            NotePitchInterval::MinorThird => 3,
            NotePitchInterval::MajorThird => 4,
            NotePitchInterval::PerfectFourth => 5,
            NotePitchInterval::AugmentedFourth | NotePitchInterval::DiminishedFifth => 6,
            NotePitchInterval::PerfectFifth => 7,
            NotePitchInterval::MinorSixth => 8,
            NotePitchInterval::MajorSixth => 9,
            NotePitchInterval::MinorSeventh => 10,
            NotePitchInterval::MajorSeventh => 11,
        };
        let sum = self + pitch_interval_value;
        // we want only pitch values to be in the range of 0-11 inclusive
        sum % 12
    }
}

impl std::ops::Add<u8> for NotePitchInterval {
    type Output = Self;
    fn add(self, other: u8) -> Self {
        let pitch_interval_value: u8 = match self {
            NotePitchInterval::PerfectUnison => 0,
            NotePitchInterval::MinorSecond => 1,
            NotePitchInterval::MajorSecond => 2,
            NotePitchInterval::MinorThird => 3,
            NotePitchInterval::MajorThird => 4,
            NotePitchInterval::PerfectFourth => 5,
            NotePitchInterval::AugmentedFourth | NotePitchInterval::DiminishedFifth => 6,
            NotePitchInterval::PerfectFifth => 7,
            NotePitchInterval::MinorSixth => 8,
            NotePitchInterval::MajorSixth => 9,
            NotePitchInterval::MinorSeventh => 10,
            NotePitchInterval::MajorSeventh => 11,
        };
        let sum = pitch_interval_value + other;
        // we want only pitch values to be in the range of 0-11 inclusive
        match sum % 12 {
            1 => NotePitchInterval::MinorSecond,
            2 => NotePitchInterval::MajorSecond,
            3 => NotePitchInterval::MinorThird,
            4 => NotePitchInterval::MajorThird,
            5 => NotePitchInterval::PerfectFourth,
            6 => {
                // if adding to AugmentedFourth, return that; if adding to DiminishedFifth, return that
                self
            }
            7 => NotePitchInterval::PerfectFifth,
            8 => NotePitchInterval::MinorSixth,
            9 => NotePitchInterval::MajorSixth,
            10 => NotePitchInterval::MinorSeventh,
            11 => NotePitchInterval::MajorSeventh,
            _ => NotePitchInterval::PerfectUnison,
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
mod is_note_name_valid_test {
    use super::*;

    #[test]
    fn is_note_name_valid_false_when_invalid_string_passed() {
        // TODO: create helper_fn to reuse an invalid note name str
        let note = Note::is_note_name_valid("");
        assert!(!note, " is not a note");
        let note = Note::is_note_name_valid("Ac");
        assert!(!note, "Ac is not a note");
        let note = Note::is_note_name_valid("H");
        assert!(!note, "H is not a note");
        let note = Note::is_note_name_valid("Ab#");
        assert!(!note, "Ab# is not a note");
        let note = Note::is_note_name_valid("Abbb");
        assert!(!note, "Abbb is not a note");
    }
    #[test]
    fn is_note_true_when_valid_string_passed_with_natural_variant() {
        for str in test_helper_fns::natural_note_name_str().split(' ') {
            let note = Note::is_note_name_valid(str);
            assert!(note, "{} is a note", str);
        }
    }
    #[test]
    fn is_note_true_when_valid_string_passed_with_flat_variant() {
        for str in test_helper_fns::flat_note_name_str().split(' ') {
            let note = Note::is_note_name_valid(str);
            assert!(note, "{} is a note", str);
        }
    }
    #[test]
    fn is_note_true_when_valid_string_passed_with_flatdbl_variant() {
        for str in test_helper_fns::flatdbl_note_name_str().split(' ') {
            let note = Note::is_note_name_valid(str);
            assert!(note, "{} is a note", str);
        }
    }
    #[test]
    fn is_note_true_when_valid_string_passed_with_sharp_variant() {
        for str in test_helper_fns::sharp_note_name_str().split(' ') {
            let note = Note::is_note_name_valid(str);
            assert!(note, "{} is a note", str);
        }
    }
    #[test]
    fn is_note_true_when_valid_string_passed_with_sharpdbl_variant() {
        for str in test_helper_fns::sharpdbl_note_name_str().split(' ') {
            let note = Note::is_note_name_valid(str);
            assert!(note, "{} is a note", str);
        }
    }
}

#[cfg(test)]
mod calc_pitch_variant_test {
    use super::*;

    #[test]
    fn calc_pitch_variant_returns_natural() {
        for str in test_helper_fns::natural_note_name_str().split(' ') {
            let pitch_variant = calc_pitch_variant(str);
            assert_eq!(pitch_variant, Some(NotePitchVariant::Natural));
        }
    }
    #[test]
    fn calc_pitch_variant_returns_flat() {
        for str in test_helper_fns::flat_note_name_str().split(' ') {
            let pitch_variant = calc_pitch_variant(str);
            assert_eq!(pitch_variant, Some(NotePitchVariant::Flat));
        }
    }
    #[test]
    fn calc_pitch_variant_returns_flatdbl() {
        for str in test_helper_fns::flatdbl_note_name_str().split(' ') {
            let pitch_variant = calc_pitch_variant(str);
            assert_eq!(pitch_variant, Some(NotePitchVariant::Flatdbl));
        }
    }
    #[test]
    fn calc_pitch_variant_returns_sharp() {
        for str in test_helper_fns::sharp_note_name_str().split(' ') {
            let pitch_variant = calc_pitch_variant(str);
            assert_eq!(pitch_variant, Some(NotePitchVariant::Sharp));
        }
    }
    #[test]
    fn calc_pitch_variant_returns_sharpdbl() {
        for str in test_helper_fns::sharpdbl_note_name_str().split(' ') {
            let pitch_variant = calc_pitch_variant(str);
            assert_eq!(pitch_variant, Some(NotePitchVariant::Sharpdbl));
        }
    }
}

#[cfg(test)]
mod calc_pitch_value_test {
    use super::*;

    #[test]
    fn calc_pitch_value_returns_correct_natural_variant() {
        let pitch_value = calc_pitch_value("a", NotePitchVariant::Natural);
        assert_eq!(pitch_value, Some(1));

        let pitch_value = calc_pitch_value("b", NotePitchVariant::Natural);
        assert_eq!(pitch_value, Some(3));

        let pitch_value = calc_pitch_value("c", NotePitchVariant::Natural);
        assert_eq!(pitch_value, Some(4));

        let pitch_value = calc_pitch_value("d", NotePitchVariant::Natural);
        assert_eq!(pitch_value, Some(6));

        let pitch_value = calc_pitch_value("e", NotePitchVariant::Natural);
        assert_eq!(pitch_value, Some(8));

        let pitch_value = calc_pitch_value("f", NotePitchVariant::Natural);
        assert_eq!(pitch_value, Some(9));

        let pitch_value = calc_pitch_value("g", NotePitchVariant::Natural);
        assert_eq!(pitch_value, Some(11));
    }

    #[test]
    fn calc_pitch_value_returns_correct_flat_variant() {
        let pitch_value = calc_pitch_value("a", NotePitchVariant::Flat);
        assert_eq!(pitch_value, Some(0));

        let pitch_value = calc_pitch_value("b", NotePitchVariant::Flat);
        assert_eq!(pitch_value, Some(2));

        let pitch_value = calc_pitch_value("c", NotePitchVariant::Flat);
        assert_eq!(pitch_value, Some(3));

        let pitch_value = calc_pitch_value("d", NotePitchVariant::Flat);
        assert_eq!(pitch_value, Some(5));

        let pitch_value = calc_pitch_value("e", NotePitchVariant::Flat);
        assert_eq!(pitch_value, Some(7));

        let pitch_value = calc_pitch_value("f", NotePitchVariant::Flat);
        assert_eq!(pitch_value, Some(8));

        let pitch_value = calc_pitch_value("g", NotePitchVariant::Flat);
        assert_eq!(pitch_value, Some(10));
    }

    #[test]
    fn calc_pitch_value_returns_correct_flatdbl_variant() {
        let pitch_value = calc_pitch_value("a", NotePitchVariant::Flatdbl);
        assert_eq!(pitch_value, Some(11));

        let pitch_value = calc_pitch_value("b", NotePitchVariant::Flatdbl);
        assert_eq!(pitch_value, Some(1));

        let pitch_value = calc_pitch_value("c", NotePitchVariant::Flatdbl);
        assert_eq!(pitch_value, Some(2));

        let pitch_value = calc_pitch_value("d", NotePitchVariant::Flatdbl);
        assert_eq!(pitch_value, Some(4));

        let pitch_value = calc_pitch_value("e", NotePitchVariant::Flatdbl);
        assert_eq!(pitch_value, Some(6));

        let pitch_value = calc_pitch_value("f", NotePitchVariant::Flatdbl);
        assert_eq!(pitch_value, Some(7));

        let pitch_value = calc_pitch_value("g", NotePitchVariant::Flatdbl);
        assert_eq!(pitch_value, Some(9));
    }

    #[test]
    fn calc_pitch_value_returns_correct_sharp_variant() {
        let pitch_value = calc_pitch_value("a", NotePitchVariant::Sharp);
        assert_eq!(pitch_value, Some(2));

        let pitch_value = calc_pitch_value("b", NotePitchVariant::Sharp);
        assert_eq!(pitch_value, Some(4));

        let pitch_value = calc_pitch_value("c", NotePitchVariant::Sharp);
        assert_eq!(pitch_value, Some(5));

        let pitch_value = calc_pitch_value("d", NotePitchVariant::Sharp);
        assert_eq!(pitch_value, Some(7));

        let pitch_value = calc_pitch_value("e", NotePitchVariant::Sharp);
        assert_eq!(pitch_value, Some(9));

        let pitch_value = calc_pitch_value("f", NotePitchVariant::Sharp);
        assert_eq!(pitch_value, Some(10));

        let pitch_value = calc_pitch_value("g", NotePitchVariant::Sharp);
        assert_eq!(pitch_value, Some(0));
    }

    #[test]
    fn calc_pitch_value_returns_correct_sharpdbl_variant() {
        let pitch_value = calc_pitch_value("a", NotePitchVariant::Sharpdbl);
        assert_eq!(pitch_value, Some(3));

        let pitch_value = calc_pitch_value("b", NotePitchVariant::Sharpdbl);
        assert_eq!(pitch_value, Some(5));

        let pitch_value = calc_pitch_value("c", NotePitchVariant::Sharpdbl);
        assert_eq!(pitch_value, Some(6));

        let pitch_value = calc_pitch_value("d", NotePitchVariant::Sharpdbl);
        assert_eq!(pitch_value, Some(8));

        let pitch_value = calc_pitch_value("e", NotePitchVariant::Sharpdbl);
        assert_eq!(pitch_value, Some(10));

        let pitch_value = calc_pitch_value("f", NotePitchVariant::Sharpdbl);
        assert_eq!(pitch_value, Some(11));

        let pitch_value = calc_pitch_value("g", NotePitchVariant::Sharpdbl);
        assert_eq!(pitch_value, Some(1));
    }
}

#[cfg(test)]
mod by_interval_test {
    use super::*;

    #[test]
    fn by_interval_returns_major_third() {
        let note = Note::from("C");
        let actual = note.by_interval(NotePitchInterval::MajorThird).unwrap();
        let expected = Note::from("E");
        assert_eq!(actual, expected);
    }
}
