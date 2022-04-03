use lazy_static::lazy_static;
use regex::Regex;

use super::{interval::NotePitchInterval, name::NotePitchName, pitch_variant::NotePitchVariant};

// global static regex to parse a note from a string slice that's only compiled once
lazy_static! {
  // check if str has a-g or A-G in one occurrence
  // check for one or two flats, or one or two sharps
  pub static ref NOTE_REGEX: Regex = Regex::new(
    r"^(?P<note_name>(?i)[a-g]{1})(?P<note_variant>(?-i)(b{1,2})|(#{1,2}))?$"
  ).unwrap();
}

// TODO: convert this to a From<NotePitchName> impl for u8
pub fn note_name_to_pitch(note_name: NotePitchName) -> u8 {
    use NotePitchName::*;
    match note_name {
        A => 1,
        B => 3,
        C => 4,
        D => 6,
        E => 8,
        F => 9,
        G => 11,
    }
}

// TODO: convert this to a From<NotePitchVariant> impl for i8
pub fn note_variant_to_pitch(note_variant: NotePitchVariant) -> i8 {
    use NotePitchVariant::*;
    match note_variant {
        Flatdbl => -2,
        Flat => -1,
        Natural => 0,
        Sharp => 1,
        Sharpdbl => 2,
    }
}

pub fn calc_pitch_variant_by_name_and_pitch_value(
    name: NotePitchName,
    pitch_value: u8,
) -> Option<NotePitchVariant> {
    use NotePitchVariant::*;
    let note_name_pitch_value: u8 = note_name_to_pitch(name);
    if note_name_pitch_value + Flatdbl == pitch_value {
        Some(Flatdbl)
    } else if note_name_pitch_value + Flat == pitch_value {
        Some(Flat)
    } else if note_name_pitch_value + Natural == pitch_value {
        Some(Natural)
    } else if note_name_pitch_value + Sharp == pitch_value {
        Some(Sharp)
    } else if note_name_pitch_value + Sharpdbl == pitch_value {
        Some(Sharpdbl)
    } else {
        None
    }
}

// TODO: remove this method, check REGEX inline
pub fn is_note_name_valid(note_name: &str) -> bool {
    (1..=3).contains(&note_name.len()) && NOTE_REGEX.is_match(note_name)
}

pub fn calc_name_by_interval(
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

// TODO: add unit tests for this fn
pub fn uppercase_first_char(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
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
