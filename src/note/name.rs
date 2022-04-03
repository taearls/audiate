use std::fmt::{Display, Formatter};

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

impl TryFrom<&str> for NotePitchName {
    type Error = String;

    fn try_from(name: &str) -> Result<Self, Self::Error> {
        match name.to_uppercase().chars().next() {
            Some('A') => Ok(NotePitchName::A),
            Some('B') => Ok(NotePitchName::B),
            Some('C') => Ok(NotePitchName::C),
            Some('D') => Ok(NotePitchName::D),
            Some('E') => Ok(NotePitchName::E),
            Some('F') => Ok(NotePitchName::F),
            Some('G') => Ok(NotePitchName::G),
            Some(c) => Err(format!("{c} is not a valid note pitch name")),
            None => Err("An empty &str is not valid".to_string()),
        }
    }
}

impl TryFrom<&String> for NotePitchName {
    type Error = String;

    fn try_from(name: &String) -> Result<Self, Self::Error> {
        match name.to_uppercase().chars().next() {
            Some('A') => Ok(NotePitchName::A),
            Some('B') => Ok(NotePitchName::B),
            Some('C') => Ok(NotePitchName::C),
            Some('D') => Ok(NotePitchName::D),
            Some('E') => Ok(NotePitchName::E),
            Some('F') => Ok(NotePitchName::F),
            Some('G') => Ok(NotePitchName::G),
            Some(c) => Err(format!("{c} is not a valid note pitch name")),
            None => Err("An empty &String is not valid".to_string()),
        }
    }
}
