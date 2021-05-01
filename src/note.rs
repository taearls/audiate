use regex::Regex;
use lazy_static::lazy_static;

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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NoteIntervalDirection {
    Ascending,
    Descending,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NotePitchInterval {
    // PerfectUnison,
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

// const ASCENDING_NOTE_MAP: HashMap<u8, String> = HashMap::new();
// const PITCHES_ALL_KEYS: [HashMap<u8, NotePitchInterval>; 12] = [
//     HashMap::new(
        
//     )
// ]

// // let name = match pitch_value {
//     1 => { // G## / A / Bbb
//         match self.name().to_lowercase().as_ref() {
//             // still need to handle G##, which comes from a leading tone to A#
//             // Cbb
//             "a#" => {
//                 String::from("G##")
//             },
//             "g#" => {
//                 String::from("A")
//             },
//             "cbb" => {
//                 String::from("Bbb")
//             },
//             _ => unreachable!(),
//         }
//     },
//     2 => { // A# / Bb / Cbb
//         match self.name().to_lowercase().as_ref() {
//             "g#" => {
//                 String::from("A#")
//             },
//             "ab" => {
//                 String::from("Bb")
//             },
//             "dbb" => {
//                 String::from("Cbb")
//             },
//             _ => unreachable!(),
//         }
//     },
//     3 => { // A## / B / Cb
//         match self.name().to_lowercase().as_ref() {
//             "c##" => {
//                 String::from("A##")
//             },
//             "g#" => {
//                 String::from("B")
//             },
//             "ab" => {
//                 String::from("Cb")
//             },
//             _ => unreachable!(),
//         }
//     },
//     4 => { // B# / C / Dbb
//         String::from("C")
//     },
//     5 => { // B## / C# / Db
//         String::from("C#")
//     },
//     6 => { // C## / D / Ebb
//         String::from("D")
//     },
//     7 => { // D# / Eb / Fbb
//         String::from("Eb")
//     },
//     8 => { // D## / E / Fb
//         String::from("E")
//     },
//     9 => { // E# / F / Gbb
//         String::from("F")
//     },
//     10 => { // E## / F# / Gb
//         String::from("F#")
//     },
//     11 => { // F## / G / Abb
//         String::from("G")
//     },
//     12 => { // G# / Ab
//         String::from("Ab")
//     }
//     _ => unreachable!(),
// };

// global static regex to parse a note from a string slice that's only compiled once
lazy_static! {
  // check if str has a-g or A-G in one occurrence
  // check for one or two flats, or one or two sharps
  static ref NOTE_REGEX: Regex = Regex::new(
    r"^(?P<note_name>(?i)[a-g]{1})(?P<note_variant>(?-i)(b{1,2})|(#{1,2}))?$"
  ).unwrap();
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
    // https://hermanradtke.com/2015/05/06/creating-a-rust-function-that-accepts-string-or-str.html
    pub fn new(note_name: &str) -> Option<Self> {
        // TODO: refactor to return an Err variant of some kind.
        if !Note::is_note(note_name) {
            return None;
        }

        let pitch_variant: NotePitchVariant = Note::calc_pitch_variant(note_name)?;
        let pitch_value: u8 = Note::calc_pitch_value(note_name, pitch_variant)?;

        Some(Note {
            name: note_name.to_uppercase(),
            pitch_value,
            pitch_variant,
        })
    }

    fn is_note(note_name: &str) -> bool {
        (1..=3).contains(&note_name.len()) && NOTE_REGEX.is_match(note_name)
    }

    fn calc_pitch_variant(note_name: &str) -> Option<NotePitchVariant> {
        if !Note::is_note(note_name) {
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

    fn calc_pitch_value(note_name: &str, pitch_variant: NotePitchVariant) -> Option<u8> {
        if !Note::is_note(note_name) {
            return None;
        }

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

#[derive(Debug, Clone, PartialEq)]
struct NoteRelative<'a> {
    base: &'a Note,
    interval: NotePitchInterval,
    direction: NoteIntervalDirection,
}

// this is an intermediate struct used when determining a target note based on an interval from a base note
impl<'a> NoteRelative<'a> {

    pub fn new(base: &'a Note, interval: NotePitchInterval, direction: NoteIntervalDirection) -> Self {
        NoteRelative {
            base,
            interval,
            direction,
        }
    }

    // should this return a reference?
    pub fn base(&self) -> &Note {
        self.base
    }

    pub fn interval(&self) -> NotePitchInterval {
        self.interval
    }

    pub fn pitch_variant(&self) -> NoteIntervalDirection {
        self.direction
    }

    pub fn relative_by_interval(
        &self,
        interval: NotePitchInterval,
        direction: NoteIntervalDirection,
    ) -> Note {
        match direction {
            NoteIntervalDirection::Ascending => {
                self.note_by_interval_ascending(interval, direction)
            }
            NoteIntervalDirection::Descending => {
                self.note_by_interval_descending(interval, direction)
            }
        }
    }

    fn note_by_interval_ascending(
        &self,
        interval: NotePitchInterval,
        direction: NoteIntervalDirection,
    ) -> Note {
        let result = String::with_capacity(3);
        Note::from("A")
    }

    fn note_by_interval_descending(
        &self,
        interval: NotePitchInterval,
        direction: NoteIntervalDirection,
    ) -> Note {
        let result = String::with_capacity(3);
        // TODO : get pitch u8
        // TODO : match note against descending hash map
        Note::from("A")
    }
}

impl From<&str> for Note {
    fn from(s: &str) -> Self {
        Note::new(s).unwrap()
    }
}

impl From<String> for Note {
    fn from(s: String) -> Self {
        Note::new(&s).unwrap()
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

impl std::ops::Add<NotePitchInterval> for u8 {
    type Output = Self;
    fn add(self, other: NotePitchInterval) -> Self {
        // TODO: handle descending intervals.
        let pitch_interval_value: u8 = match other {
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
        for str in test_helper_fns::natural_note_name_str().split(' ') {
            let note = Note::is_note(str);
            assert!(note, "{} is a note", str);
        }
    }
    #[test]
    fn is_note_true_when_valid_string_passed_with_flat_variant() {
        for str in test_helper_fns::flat_note_name_str().split(' ') {
            let note = Note::is_note(str);
            assert!(note, "{} is a note", str);
        }
    }
    #[test]
    fn is_note_true_when_valid_string_passed_with_flatdbl_variant() {
        for str in test_helper_fns::flatdbl_note_name_str().split(' ') {
            let note = Note::is_note(str);
            assert!(note, "{} is a note", str);
        }
    }
    #[test]
    fn is_note_true_when_valid_string_passed_with_sharp_variant() {
        for str in test_helper_fns::sharp_note_name_str().split(' ') {
            let note = Note::is_note(str);
            assert!(note, "{} is a note", str);
        }
    }
    #[test]
    fn is_note_true_when_valid_string_passed_with_sharpdbl_variant() {
        for str in test_helper_fns::sharpdbl_note_name_str().split(' ') {
            let note = Note::is_note(str);
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
            let pitch_variant = Note::calc_pitch_variant(str);
            assert_eq!(pitch_variant, Some(NotePitchVariant::Natural));
        }
    }
    #[test]
    fn calc_pitch_variant_returns_flat() {
        for str in test_helper_fns::flat_note_name_str().split(' ') {
            let pitch_variant = Note::calc_pitch_variant(str);
            assert_eq!(pitch_variant, Some(NotePitchVariant::Flat));
        }
    }
    #[test]
    fn calc_pitch_variant_returns_flatdbl() {
        for str in test_helper_fns::flatdbl_note_name_str().split(' ') {
            let pitch_variant = Note::calc_pitch_variant(str);
            assert_eq!(pitch_variant, Some(NotePitchVariant::Flatdbl));
        }
    }
    #[test]
    fn calc_pitch_variant_returns_sharp() {
        for str in test_helper_fns::sharp_note_name_str().split(' ') {
            let pitch_variant = Note::calc_pitch_variant(str);
            assert_eq!(pitch_variant, Some(NotePitchVariant::Sharp));
        }
    }
    #[test]
    fn calc_pitch_variant_returns_sharpdbl() {
        for str in test_helper_fns::sharpdbl_note_name_str().split(' ') {
            let pitch_variant = Note::calc_pitch_variant(str);
            assert_eq!(pitch_variant, Some(NotePitchVariant::Sharpdbl));
        }
    }
}

#[cfg(test)]
mod calc_pitch_value_test {
    use super::*;

    #[test]
    fn calc_pitch_value_returns_correct_natural_variant() {
        let pitch_value = Note::calc_pitch_value("a", NotePitchVariant::Natural);
        assert_eq!(pitch_value, Some(1));

        let pitch_value = Note::calc_pitch_value("b", NotePitchVariant::Natural);
        assert_eq!(pitch_value, Some(3));

        let pitch_value = Note::calc_pitch_value("c", NotePitchVariant::Natural);
        assert_eq!(pitch_value, Some(4));

        let pitch_value = Note::calc_pitch_value("d", NotePitchVariant::Natural);
        assert_eq!(pitch_value, Some(6));

        let pitch_value = Note::calc_pitch_value("e", NotePitchVariant::Natural);
        assert_eq!(pitch_value, Some(8));

        let pitch_value = Note::calc_pitch_value("f", NotePitchVariant::Natural);
        assert_eq!(pitch_value, Some(9));

        let pitch_value = Note::calc_pitch_value("g", NotePitchVariant::Natural);
        assert_eq!(pitch_value, Some(11));
    }

    #[test]
    fn calc_pitch_value_returns_correct_flat_variant() {
        let pitch_value = Note::calc_pitch_value("a", NotePitchVariant::Flat);
        assert_eq!(pitch_value, Some(12));

        let pitch_value = Note::calc_pitch_value("b", NotePitchVariant::Flat);
        assert_eq!(pitch_value, Some(2));

        let pitch_value = Note::calc_pitch_value("c", NotePitchVariant::Flat);
        assert_eq!(pitch_value, Some(3));

        let pitch_value = Note::calc_pitch_value("d", NotePitchVariant::Flat);
        assert_eq!(pitch_value, Some(5));

        let pitch_value = Note::calc_pitch_value("e", NotePitchVariant::Flat);
        assert_eq!(pitch_value, Some(7));

        let pitch_value = Note::calc_pitch_value("f", NotePitchVariant::Flat);
        assert_eq!(pitch_value, Some(8));

        let pitch_value = Note::calc_pitch_value("g", NotePitchVariant::Flat);
        assert_eq!(pitch_value, Some(10));
    }

    #[test]
    fn calc_pitch_value_returns_correct_flatdbl_variant() {
        let pitch_value = Note::calc_pitch_value("a", NotePitchVariant::Flatdbl);
        assert_eq!(pitch_value, Some(11));

        let pitch_value = Note::calc_pitch_value("b", NotePitchVariant::Flatdbl);
        assert_eq!(pitch_value, Some(1));

        let pitch_value = Note::calc_pitch_value("c", NotePitchVariant::Flatdbl);
        assert_eq!(pitch_value, Some(2));

        let pitch_value = Note::calc_pitch_value("d", NotePitchVariant::Flatdbl);
        assert_eq!(pitch_value, Some(4));

        let pitch_value = Note::calc_pitch_value("e", NotePitchVariant::Flatdbl);
        assert_eq!(pitch_value, Some(6));

        let pitch_value = Note::calc_pitch_value("f", NotePitchVariant::Flatdbl);
        assert_eq!(pitch_value, Some(7));

        let pitch_value = Note::calc_pitch_value("g", NotePitchVariant::Flatdbl);
        assert_eq!(pitch_value, Some(9));
    }

    #[test]
    fn calc_pitch_value_returns_correct_sharp_variant() {
        let pitch_value = Note::calc_pitch_value("a", NotePitchVariant::Sharp);
        assert_eq!(pitch_value, Some(2));

        let pitch_value = Note::calc_pitch_value("b", NotePitchVariant::Sharp);
        assert_eq!(pitch_value, Some(4));

        let pitch_value = Note::calc_pitch_value("c", NotePitchVariant::Sharp);
        assert_eq!(pitch_value, Some(5));

        let pitch_value = Note::calc_pitch_value("d", NotePitchVariant::Sharp);
        assert_eq!(pitch_value, Some(7));

        let pitch_value = Note::calc_pitch_value("e", NotePitchVariant::Sharp);
        assert_eq!(pitch_value, Some(9));

        let pitch_value = Note::calc_pitch_value("f", NotePitchVariant::Sharp);
        assert_eq!(pitch_value, Some(10));

        let pitch_value = Note::calc_pitch_value("g", NotePitchVariant::Sharp);
        assert_eq!(pitch_value, Some(12));
    }

    #[test]
    fn calc_pitch_value_returns_correct_sharpdbl_variant() {
        let pitch_value = Note::calc_pitch_value("a", NotePitchVariant::Sharpdbl);
        assert_eq!(pitch_value, Some(3));

        let pitch_value = Note::calc_pitch_value("b", NotePitchVariant::Sharpdbl);
        assert_eq!(pitch_value, Some(5));

        let pitch_value = Note::calc_pitch_value("c", NotePitchVariant::Sharpdbl);
        assert_eq!(pitch_value, Some(6));

        let pitch_value = Note::calc_pitch_value("d", NotePitchVariant::Sharpdbl);
        assert_eq!(pitch_value, Some(8));

        let pitch_value = Note::calc_pitch_value("e", NotePitchVariant::Sharpdbl);
        assert_eq!(pitch_value, Some(10));

        let pitch_value = Note::calc_pitch_value("f", NotePitchVariant::Sharpdbl);
        assert_eq!(pitch_value, Some(11));

        let pitch_value = Note::calc_pitch_value("g", NotePitchVariant::Sharpdbl);
        assert_eq!(pitch_value, Some(1));
    }
}
