use lazy_static::lazy_static;
use regex::Regex;

use super::{interval::NotePitchInterval, name::NotePitchName, pitch_variant::NotePitchVariant};

// global static regex to parse a note from a string slice that's only compiled once
lazy_static! {
  // check if str has a-g or A-G in one occurrence
  // check for one or two flats, or one or two sharps
  static ref NOTE_REGEX: Regex = Regex::new(
    r"^(?P<note_name>(?i)[a-g]{1})(?P<note_variant>(?-i)(b{1,2})|(#{1,2}))?$"
  ).unwrap();
}

pub fn note_name_to_pitch(note_name: NotePitchName) -> u8 {
    match note_name {
        NotePitchName::A => 1,
        NotePitchName::B => 3,
        NotePitchName::C => 4,
        NotePitchName::D => 6,
        NotePitchName::E => 8,
        NotePitchName::F => 9,
        NotePitchName::G => 11,
    }
}

pub fn calc_note_name(note_name: &str) -> Option<NotePitchName> {
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
pub fn is_note_name_valid(note_name: &str) -> bool {
    println!(
        "length constraint: {}, regex match: {}",
        (1..=3).contains(&note_name.len()),
        NOTE_REGEX.is_match(note_name)
    );
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

pub fn calc_pitch_value(note_name: NotePitchName, pitch_variant: NotePitchVariant) -> Option<u8> {
    let note_name_pitch_value = note_name_to_pitch(note_name);

    Some(note_name_pitch_value + pitch_variant)
}

pub fn calc_pitch_value_from_interval(pitch_value: u8, interval: NotePitchInterval) -> u8 {
    pitch_value + interval
}

pub fn calc_pitch_variant(note_name: &str) -> Option<NotePitchVariant> {
    if !is_note_name_valid(note_name) {
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

// TODO: add unit tests
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
        // TODO: create helper_fn to reuse an invalid note name str
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
