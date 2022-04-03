use std::fmt::{Display, Formatter};

use super::name::{is_note_name_valid, note_variant_from_note_name};

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
        use NotePitchVariant::*;
        let name: &str = match self {
            Flatdbl => "bb",
            Flat => "b",
            Natural => "",
            Sharp => "#",
            Sharpdbl => "##",
        };
        write!(f, "{name}")
    }
}

impl std::ops::Add<NotePitchVariant> for u8 {
    type Output = Self;
    fn add(self, other: NotePitchVariant) -> Self {
        let pitch_variant_value = i8::from(other);

        // we add 12 to prevent underflow
        let sum = (self as i8 + 12 + pitch_variant_value) as u8;

        // we want only pitch values to be in the range of 0-11 inclusive
        sum % 12
    }
}

impl From<NotePitchVariant> for i8 {
    fn from(variant: NotePitchVariant) -> i8 {
        use NotePitchVariant::*;
        match variant {
            Flatdbl => -2,
            Flat => -1,
            Natural => 0,
            Sharp => 1,
            Sharpdbl => 2,
        }
    }
}

impl TryFrom<&str> for NotePitchVariant {
    type Error = String;

    fn try_from(note_name: &str) -> Result<Self, Self::Error> {
        use NotePitchVariant::*;
        if !is_note_name_valid(note_name) {
            return Err(format!("{note_name} is not a valid note name"));
        }

        let note_variant = note_variant_from_note_name(note_name);
        match note_variant {
            Some("b") => Ok(Flat),
            Some("bb") => Ok(Flatdbl),
            Some("#") => Ok(Sharp),
            Some("##") => Ok(Sharpdbl),
            Some(s) => Err(format!("{s} is not a valid note variant")),
            None => Ok(Natural),
        }
    }
}

impl TryFrom<&String> for NotePitchVariant {
    type Error = String;

    fn try_from(note_name: &String) -> Result<Self, Self::Error> {
        use NotePitchVariant::*;
        if !is_note_name_valid(note_name) {
            return Err(format!("{note_name} is not a valid note name"));
        }

        let note_variant = note_variant_from_note_name(note_name);
        match note_variant {
            Some("b") => Ok(Flat),
            Some("bb") => Ok(Flatdbl),
            Some("#") => Ok(Sharp),
            Some("##") => Ok(Sharpdbl),
            Some(s) => Err(format!("{s} is not a valid note variant")),
            None => Ok(Natural),
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
