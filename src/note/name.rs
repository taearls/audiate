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
