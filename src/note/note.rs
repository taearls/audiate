use std::fmt::{Display, Formatter};

use super::{
    interval::NotePitchInterval,
    name::{is_note_name_valid, NotePitchName},
    pitch_variant::NotePitchVariant,
    util,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Note {
    name: NotePitchName,
    pitch_variant: NotePitchVariant,
}

impl Note {
    pub fn new(name: NotePitchName, pitch_variant: NotePitchVariant) -> Self {
        Self {
            name,
            pitch_variant,
        }
    }

    pub fn name(&self) -> NotePitchName {
        self.name
    }

    pub fn pitch_variant(&self) -> NotePitchVariant {
        self.pitch_variant
    }

    pub fn by_interval_ascending(&self, interval: NotePitchInterval) -> Note {
        self.by_interval(interval)
    }

    pub fn by_interval_descending(&self, interval: NotePitchInterval) -> Note {
        self.by_interval(interval.invert())
    }

    fn pitch_value(name: NotePitchName, pitch_variant: NotePitchVariant) -> u8 {
        let note_name_pitch = u8::from(name);
        let note_variant_pitch = ((i8::from(pitch_variant) + 12) % 12) as u8;
        (note_name_pitch + note_variant_pitch) % 12
    }

    fn by_interval(&self, interval: NotePitchInterval) -> Note {
        let name = self.name.by_interval(interval);
        let root_pitch_value = Note::pitch_value(self.name, self.pitch_variant);
        let new_pitch_value = root_pitch_value + interval;
        let pitch_variant = calc_pitch_variant_by_name_and_pitch_value(name, new_pitch_value);
        Note::new(name, pitch_variant)
    }
}

impl Display for Note {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let note_name = self.name;
        let pitch_variant = self.pitch_variant;
        write!(f, "{note_name}{pitch_variant}")
    }
}

// cannot make this TryFrom impl generic https://github.com/rust-lang/rust/issues/50133
impl TryFrom<&str> for Note {
    type Error = String;

    fn try_from(name: &str) -> Result<Self, Self::Error> {
        let note_name_str: String = util::uppercase_first_char(name);
        if !is_note_name_valid(&note_name_str) {
            return Err(format!(
                "Note name &str provided failed validation. {name} is not a valid note name"
            ));
        }
        let note_name = NotePitchName::try_from(&note_name_str);
        let pitch_variant = NotePitchVariant::try_from(&note_name_str);

        if note_name.is_err() {
            return Err(format!("Unable to construct NotePitchName with {name}"));
        } else if pitch_variant.is_err() {
            return Err(format!("Unable to construct NotePitchVariant with {name}"));
        }
        let note_name = note_name.unwrap();
        let pitch_variant = pitch_variant.unwrap();
        Ok(Note::new(note_name, pitch_variant))
    }
}
impl TryFrom<&String> for Note {
    type Error = String;

    fn try_from(name: &String) -> Result<Self, Self::Error> {
        let note_name_str: String = util::uppercase_first_char(name);
        if !is_note_name_valid(&note_name_str) {
            return Err(format!(
                "Note name &str provided failed validation. {name} is not a valid note name"
            ));
        }
        let note_name = NotePitchName::try_from(&note_name_str);
        let pitch_variant = NotePitchVariant::try_from(&note_name_str);

        if note_name.is_err() {
            return Err(format!("Unable to construct NotePitchName with {name}"));
        } else if pitch_variant.is_err() {
            return Err(format!("Unable to construct NotePitchVariant with {name}"));
        }
        let note_name = note_name.unwrap();
        let pitch_variant = pitch_variant.unwrap();
        Ok(Note::new(note_name, pitch_variant))
    }
}

fn calc_pitch_variant_by_name_and_pitch_value(
    name: NotePitchName,
    pitch_value: u8,
) -> NotePitchVariant {
    use NotePitchVariant::*;
    let note_name_pitch_value = u8::from(name);

    if note_name_pitch_value + Flatdbl == pitch_value {
        Flatdbl
    } else if note_name_pitch_value + Flat == pitch_value {
        Flat
    } else if note_name_pitch_value + Natural == pitch_value {
        Natural
    } else if note_name_pitch_value + Sharp == pitch_value {
        Sharp
    } else if note_name_pitch_value + Sharpdbl == pitch_value {
        Sharpdbl
    } else {
        unreachable!("We know the pitch value associated with the given note name is within 2 where this is called, because the starting value is derived from the Note struct, and the new one is calculated with the NotePitchInterval struct.")
    }
}

////////////////
// UNIT TESTS //
////////////////

#[cfg(test)]
mod pitch_value_test {
    use super::*;
    use NotePitchName::*;
    use NotePitchVariant::*;

    fn test_case(note_name: NotePitchName, pitch_variant: NotePitchVariant, expected: u8) {
        let actual = Note::pitch_value(note_name, pitch_variant);
        assert_eq!(actual, expected);
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
