use std::fmt::{Display, Formatter};

use super::util;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

impl TryFrom<&str> for NotePitchVariant {
    type Error = String;

    fn try_from(note_name: &str) -> Result<Self, Self::Error> {
        if !util::is_note_name_valid(note_name) {
            return Err(format!("{note_name} is not a valid note name"));
        }

        let note_variant = util::NOTE_REGEX.captures(note_name).and_then(|cap| {
            cap.name("note_variant")
                .map(|note_variant| note_variant.as_str())
        });
        match note_variant {
            Some("b") => Ok(NotePitchVariant::Flat),
            Some("bb") => Ok(NotePitchVariant::Flatdbl),
            Some("#") => Ok(NotePitchVariant::Sharp),
            Some("##") => Ok(NotePitchVariant::Sharpdbl),
            Some(s) => Err(format!("{s} is not a valid note variant")),
            None => Ok(NotePitchVariant::Natural),
        }
    }
}

impl TryFrom<&String> for NotePitchVariant {
    type Error = String;

    fn try_from(note_name: &String) -> Result<Self, Self::Error> {
        if !util::is_note_name_valid(note_name) {
            return Err(format!("{note_name} is not a valid note name"));
        }

        let note_variant = util::NOTE_REGEX.captures(note_name).and_then(|cap| {
            cap.name("note_variant")
                .map(|note_variant| note_variant.as_str())
        });
        match note_variant {
            Some("b") => Ok(NotePitchVariant::Flat),
            Some("bb") => Ok(NotePitchVariant::Flatdbl),
            Some("#") => Ok(NotePitchVariant::Sharp),
            Some("##") => Ok(NotePitchVariant::Sharpdbl),
            Some(s) => Err(format!("{s} is not a valid note variant")),
            None => Ok(NotePitchVariant::Natural),
        }
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
mod calc_pitch_variant_test {
    use super::*;

    #[test]
    fn calc_pitch_variant_returns_natural() {
        for str in test_helper_fns::natural_note_name_str().split(' ') {
            let pitch_variant = NotePitchVariant::try_from(str);
            assert_eq!(pitch_variant, Ok(NotePitchVariant::Natural));
        }
    }
    #[test]
    fn calc_pitch_variant_returns_flat() {
        for str in test_helper_fns::flat_note_name_str().split(' ') {
            let pitch_variant = NotePitchVariant::try_from(str);
            assert_eq!(pitch_variant, Ok(NotePitchVariant::Flat));
        }
    }
    #[test]
    fn calc_pitch_variant_returns_flatdbl() {
        for str in test_helper_fns::flatdbl_note_name_str().split(' ') {
            let pitch_variant = NotePitchVariant::try_from(str);
            assert_eq!(pitch_variant, Ok(NotePitchVariant::Flatdbl));
        }
    }
    #[test]
    fn calc_pitch_variant_returns_sharp() {
        for str in test_helper_fns::sharp_note_name_str().split(' ') {
            let pitch_variant = NotePitchVariant::try_from(str);
            assert_eq!(pitch_variant, Ok(NotePitchVariant::Sharp));
        }
    }
    #[test]
    fn calc_pitch_variant_returns_sharpdbl() {
        for str in test_helper_fns::sharpdbl_note_name_str().split(' ') {
            let pitch_variant = NotePitchVariant::try_from(str);
            assert_eq!(pitch_variant, Ok(NotePitchVariant::Sharpdbl));
        }
    }
}
