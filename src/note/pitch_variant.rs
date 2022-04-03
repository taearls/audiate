use std::fmt::{Display, Formatter};

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
