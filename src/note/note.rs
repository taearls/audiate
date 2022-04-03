use std::fmt::{Display, Formatter};

use super::{
    interval::NotePitchInterval, name::NotePitchName, pitch_variant::NotePitchVariant, util,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Note {
    name: NotePitchName,
    // maybe I don't need the pitch_value field in this struct
    pitch_value: u8, // TODO: investigate if I can type this as a const generic range between 0 - 11
    pitch_variant: NotePitchVariant,
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

    pub fn by_interval_ascending(&self, interval: NotePitchInterval) -> Option<Note> {
        self.by_interval(interval)
    }

    pub fn by_interval_descending(&self, interval: NotePitchInterval) -> Option<Note> {
        self.by_interval(interval.invert())
    }

    // TODO: find way to remove Option, or perhaps provide a separate fn that doesn't return an Option
    fn by_interval(&self, interval: NotePitchInterval) -> Option<Note> {
        let name = util::calc_name_by_interval(self.name, interval);
        let pitch_value = util::calc_pitch_value_from_interval(self.pitch_value, interval);

        Some(Note {
            name,
            pitch_value,
            pitch_variant: util::calc_pitch_variant_by_name_and_pitch_value(name, pitch_value)?,
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

// cannot make this TryFrom impl generic https://github.com/rust-lang/rust/issues/50133
impl TryFrom<&str> for Note {
    type Error = String;

    fn try_from(name: &str) -> Result<Self, Self::Error> {
        let note_name_str: String = util::uppercase_first_char(name);
        if !util::is_note_name_valid(&note_name_str) {
            return Err(format!(
                "Note name &str provided failed validation. {name} is not a valid note name"
            ));
        }
        let note_name = util::calc_note_name(&note_name_str);
        let pitch_variant = util::calc_pitch_variant(&note_name_str);

        if note_name.is_none() {
            return Err(format!("Unable to construct NotePitchName with {name}"));
        } else if pitch_variant.is_none() {
            return Err(format!("Unable to construct NotePitchVariant with {name}"));
        }
        let note_name = note_name.unwrap();
        let pitch_variant = pitch_variant.unwrap();
        let pitch_value = util::calc_pitch_value(note_name, pitch_variant);

        if let Some(pitch_value) = pitch_value {
            Ok(Note {
                name: note_name,
                pitch_value,
                pitch_variant,
            })
        } else {
            return Err(format!("{name} is not a valid note name"));
        }
    }
}
impl TryFrom<String> for Note {
    type Error = &'static str;

    fn try_from(name: String) -> Result<Self, Self::Error> {
        let note_name_str: String = name.to_uppercase();
        if !util::is_note_name_valid(&note_name_str) {
            return Err("{name} is not a valid note name");
        }
        let note_name = util::calc_note_name(&note_name_str);
        let pitch_variant = util::calc_pitch_variant(&note_name_str);
        if note_name.is_none() || pitch_variant.is_none() {
            return Err("{name} is not a valid note name");
        }
        let note_name = note_name.unwrap();
        let pitch_variant = pitch_variant.unwrap();
        let pitch_value = util::calc_pitch_value(note_name, pitch_variant);

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
