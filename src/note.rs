#![allow(dead_code)]
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub struct Note {
    name: NotePitchName,
    pitch_value: u8, // TODO: investigate if I can type this as a const generic range between 0 - 11.
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
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let name: &str = match self {
            NotePitchVariant::Flatdbl => "bb",
            NotePitchVariant::Flat => "b",
            NotePitchVariant::Natural => "",
            NotePitchVariant::Sharp => "#",
            NotePitchVariant::Sharpdbl => "##",
        };
        write!(f, "{name}")
    }
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

impl Display for NotePitchName {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let name: char = match self {
            NotePitchName::A => 'A',
            NotePitchName::B => 'B',
            NotePitchName::C => 'C',
            NotePitchName::D => 'D',
            NotePitchName::E => 'E',
            NotePitchName::F => 'F',
            NotePitchName::G => 'G',
        };
        write!(f, "{name}")
    }
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
    static ref NOTE_NAME_TO_PITCH: HashMap<NotePitchName, u8> = HashMap::from([
        (NotePitchName::A, 1),
        (NotePitchName::B, 3),
        (NotePitchName::C, 4),
        (NotePitchName::D, 6),
        (NotePitchName::E, 8),
        (NotePitchName::F, 9),
        (NotePitchName::G, 11),
    ]);
}

lazy_static! {
    static ref PITCH_TO_NOTE_NAME: HashMap<u8, NotePitchName> = HashMap::from([
        (1, NotePitchName::A),
        (3, NotePitchName::B),
        (4, NotePitchName::C),
        (6, NotePitchName::D),
        (8, NotePitchName::E),
        (9, NotePitchName::F),
        (11, NotePitchName::G),
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
    pub fn name(&self) -> NotePitchName {
        self.name
    }

    pub fn pitch_value(&self) -> u8 {
        self.pitch_value
    }

    pub fn pitch_variant(&self) -> NotePitchVariant {
        self.pitch_variant
    }

    pub fn with_pitch(value: u8, variant: NotePitchVariant) -> Option<Self> {
        unimplemented!("Create a new Note from the pitch value {value} and its variant {variant}");
    }

    fn is_note_name_valid(note_name: &str) -> bool {
        (1..=3).contains(&note_name.len()) && NOTE_REGEX.is_match(note_name)
    }

    // TODO: find way to remove Option, or perhaps provide a separate fn that doesn't return an Option
    fn by_interval(&self, interval: NotePitchInterval) -> Option<Note> {
        let name = calc_name_by_interval(self.name, interval);
        let pitch_value = calc_pitch_value_from_interval(self.pitch_value, interval);

        Some(Note {
            name,
            pitch_value,
            pitch_variant: calc_pitch_variant_by_name_and_pitch_value(name, pitch_value)?,
        })
    }
}

impl Display for Note {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let note_name = self.name;
        let pitch_variant = self.pitch_variant;
        write!(f, "{note_name}{pitch_variant}")
    }
}

fn calc_note_name(note_name: &str) -> Option<NotePitchName> {
    match note_name.to_uppercase().chars().next() {
        Some('A') => Some(NotePitchName::A),
        Some('B') => Some(NotePitchName::B),
        Some('C') => Some(NotePitchName::C),
        Some('D') => Some(NotePitchName::D),
        Some('E') => Some(NotePitchName::E),
        Some('F') => Some(NotePitchName::F),
        Some('G') => Some(NotePitchName::G),
        _ => None,
    }
}

fn calc_pitch_variant_by_name_and_pitch_value(
    name: NotePitchName,
    pitch_value: u8,
) -> Option<NotePitchVariant> {
    let note_name_pitch_value: u8 = *NOTE_NAME_TO_PITCH.get(&name).unwrap();

    let pitch_value_difference: i8 = std::cmp::min(
        (note_name_pitch_value + 12) as i8 - (pitch_value + 12) as i8,
        (pitch_value + 12) as i8 - (note_name_pitch_value + 12) as i8,
    );

    // if note_name_pitch_value

    // match std::cmp::max(note_name_pitch_value, pitch_value) {
    //     x if x == note_name_pitch_value => {
    //         match pitch_value_difference {
    //             -2 => Some(NotePitchVariant::Flatdbl),
    //             -1 => Some(NotePitchVariant::Flat),
    //             0 => Some(NotePitchVariant::Natural),
    //             1 => Some(NotePitchVariant::Sharp),
    //             2 => Some(NotePitchVariant::Sharpdbl),
    //             _ => None,
    //         }
    //     },
    //     _ => {
    //         match pitch_value_difference {
    //             2 => Some(NotePitchVariant::Flatdbl),
    //             1 => Some(NotePitchVariant::Flat),
    //             0 => Some(NotePitchVariant::Natural),
    //             -1 => Some(NotePitchVariant::Sharp),
    //             -2 => Some(NotePitchVariant::Sharpdbl),
    //             _ => None,
    //         }
    //     }
    // }

    match pitch_value_difference {
        -2 => Some(NotePitchVariant::Flatdbl),
        -1 => Some(NotePitchVariant::Flat),
        0 => Some(NotePitchVariant::Natural),
        1 => Some(NotePitchVariant::Sharp),
        2 => Some(NotePitchVariant::Sharpdbl),
        _ => None,
    }
}

fn calc_name_by_interval(
    note_name: NotePitchName,
    pitch_interval: NotePitchInterval,
) -> NotePitchName {
    use NotePitchInterval::*;
    use NotePitchName::*;

    let names: [NotePitchName; 7] = [A, B, C, D, E, F, G];
    let original_idx = names
        .iter()
        .position(|&name: &NotePitchName| name == note_name)
        .unwrap();
    let interval_index: usize = match pitch_interval {
        PerfectUnison => 0,
        MinorSecond | MajorSecond => 1,
        MinorThird | MajorThird => 2,
        PerfectFourth | AugmentedFourth => 3,
        DiminishedFifth | PerfectFifth => 4,
        MinorSixth | MajorSixth => 5,
        MinorSeventh | MajorSeventh => 6,
    };
    let new_index = (original_idx + interval_index) % names.len();
    names[new_index]
}

fn calc_pitch_value(note_name: NotePitchName, pitch_variant: NotePitchVariant) -> Option<u8> {
    let note_name_pitch_value = match NOTE_NAME_TO_PITCH.get(&note_name) {
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

// cannot make this TryFrom impl generic https://github.com/rust-lang/rust/issues/50133
impl TryFrom<&str> for Note {
    type Error = &'static str;

    fn try_from(name: &str) -> Result<Self, Self::Error> {
        let note_name_str: String = name.to_string().to_uppercase();
        if !Note::is_note_name_valid(&note_name_str) {
            return Err("{name} is not a valid note name");
        }
        let note_name = calc_note_name(&note_name_str);
        let pitch_variant = calc_pitch_variant(&note_name_str);
        if note_name.is_none() || pitch_variant.is_none() {
            return Err("{name} is not a valid note name");
        }
        let note_name = note_name.unwrap();
        let pitch_variant = pitch_variant.unwrap();
        let pitch_value = calc_pitch_value(note_name, pitch_variant);

        if let Some(pitch_value) = pitch_value {
            Ok(Note {
                name: note_name,
                pitch_value,
                pitch_variant,
            })
        } else {
            Err("{name} is not a valid note name")
        }
    }
}
impl TryFrom<String> for Note {
    type Error = &'static str;

    fn try_from(name: String) -> Result<Self, Self::Error> {
        let note_name_str: String = name.to_uppercase();
        if !Note::is_note_name_valid(&note_name_str) {
            return Err("{name} is not a valid note name");
        }
        let note_name = calc_note_name(&note_name_str);
        let pitch_variant = calc_pitch_variant(&note_name_str);
        if note_name.is_none() || pitch_variant.is_none() {
            return Err("{name} is not a valid note name");
        }
        let note_name = note_name.unwrap();
        let pitch_variant = pitch_variant.unwrap();
        let pitch_value = calc_pitch_value(note_name, pitch_variant);

        if let Some(pitch_value) = pitch_value {
            Ok(Note {
                name: note_name,
                pitch_value,
                pitch_variant,
            })
        } else {
            Err("{name} is not a valid note name")
        }
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
    use NotePitchName::*;
    use NotePitchVariant::*;

    fn test_case(note_name: NotePitchName, pitch_variant: NotePitchVariant, expected: u8) {
        let actual = calc_pitch_value(note_name, pitch_variant);
        assert_eq!(actual.unwrap(), expected);
    }

    #[test]
    fn calc_pitch_value_returns_correct_natural_variant() {
        test_case(A, Natural, 1);
        test_case(B, Natural, 3);
        test_case(C, Natural, 4);
        test_case(D, Natural, 6);
        test_case(E, Natural, 8);
        test_case(F, Natural, 9);
        test_case(G, Natural, 11);
    }

    #[test]
    fn calc_pitch_value_returns_correct_flat_variant() {
        test_case(A, Flat, 0);
        test_case(B, Flat, 2);
        test_case(C, Flat, 3);
        test_case(D, Flat, 5);
        test_case(E, Flat, 7);
        test_case(F, Flat, 8);
        test_case(G, Flat, 10);
    }

    #[test]
    fn calc_pitch_value_returns_correct_flatdbl_variant() {
        test_case(A, Flatdbl, 11);
        test_case(B, Flatdbl, 1);
        test_case(C, Flatdbl, 2);
        test_case(D, Flatdbl, 4);
        test_case(E, Flatdbl, 6);
        test_case(F, Flatdbl, 7);
        test_case(G, Flatdbl, 9);
    }

    #[test]
    fn calc_pitch_value_returns_correct_sharp_variant() {
        test_case(A, Sharp, 2);
        test_case(B, Sharp, 4);
        test_case(C, Sharp, 5);
        test_case(D, Sharp, 7);
        test_case(E, Sharp, 9);
        test_case(F, Sharp, 10);
        test_case(G, Sharp, 0);
    }

    #[test]
    fn calc_pitch_value_returns_correct_sharpdbl_variant() {
        test_case(A, Sharpdbl, 3);
        test_case(B, Sharpdbl, 5);
        test_case(C, Sharpdbl, 6);
        test_case(D, Sharpdbl, 8);
        test_case(E, Sharpdbl, 10);
        test_case(F, Sharpdbl, 11);
        test_case(G, Sharpdbl, 1);
    }
}

#[cfg(test)]
mod by_interval_test {
    use super::*;
    use NotePitchInterval::*;

    fn test_case(start_note_name: &str, interval: NotePitchInterval, end_note_name: &str) {
        let note = Note::try_from(start_note_name).unwrap();
        let actual = note.by_interval(interval).unwrap();
        let expected = Note::try_from(end_note_name).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn by_interval_returns_major_third_natural_root() {
        // test_case("A", MajorThird, "C#");
        // test_case("B", MajorThird, "D#");
        test_case("C", MajorThird, "E");
        // test_case("D", MajorThird, "F#");
        // test_case("E", MajorThird, "G#");
        test_case("F", MajorThird, "A");
        test_case("G", MajorThird, "B");
    }

    #[test]
    #[ignore]
    fn by_interval_returns_minor_third_natural_root() {
        test_case("A", MinorThird, "C");
        test_case("B", MinorThird, "D");
        // test_case("C", MinorThird, "Eb");
        test_case("D", MinorThird, "F");
        test_case("E", MinorThird, "G");
        // test_case("F", MinorThird, "Ab");
        // test_case("G", MinorThird, "Bb");
    }
}
