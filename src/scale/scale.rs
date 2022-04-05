#![allow(dead_code)]
use crate::note::Note;

use super::ScaleKind;

pub struct Scale {
    direction: ScaleDirection,
    kind: ScaleKind,
    notes: Vec<Note>,
}

#[derive(Copy, Clone, PartialEq)]
pub enum ScaleDirection {
    Ascending,
    Descending,
    AscendingDescending,
    DescendingAscending,
}

impl Scale {
    pub fn new(root_note: Note, kind: ScaleKind, direction: ScaleDirection) -> Self {
        Self {
            notes: Scale::from_root_note_and_kind(root_note, kind, direction),
            kind,
            direction,
        }
    }

    pub fn kind(&self) -> ScaleKind {
        self.kind
    }

    pub fn notes(self) -> Vec<Note> {
        self.notes
    }

    pub fn print(&self) -> String {
        let mut result = String::with_capacity(self.notes.len() * 2 - 1); // minimum length is twice the amount - 1 space at the end
        let mut index = 0;
        for &note in self.notes.iter() {
            result.push_str(&note.to_string());
            if index < self.notes.len() - 1 {
                result.push(' ');
                index += 1;
            }
        }
        result
    }

    fn from_root_note_and_kind(
        root_note: Note,
        kind: ScaleKind,
        direction: ScaleDirection,
    ) -> Vec<Note> {
        use ScaleDirection::*;
        let mut result: Vec<Note> = vec![root_note];

        for interval in kind.intervals(direction) {
            let next = match direction {
                Ascending | AscendingDescending => {
                    result.last().unwrap().by_interval_ascending(interval)
                }
                Descending | DescendingAscending => {
                    result.last().unwrap().by_interval_descending(interval)
                }
            };
            result.push(next);
        }
        match direction {
            AscendingDescending => {
                let second_half = Scale::from_root_note_and_kind(root_note, kind, Descending);
                result.extend_from_slice(&second_half[1..]);
            }
            DescendingAscending => {
                let second_half = Scale::from_root_note_and_kind(root_note, kind, Ascending);
                result.extend_from_slice(&second_half[1..]);
            }
            _ => {}
        }

        result
    }
}

////////////////
// UNIT TESTS //
////////////////

#[cfg(test)]
mod scale_print_test {
    use super::*;
    use ScaleDirection::*;
    use ScaleKind::*;

    fn test_case(root_note_str: &str, kind: ScaleKind, direction: ScaleDirection, expected: &str) {
        let actual = Scale::new(Note::try_from(root_note_str).unwrap(), kind, direction).print();
        assert_eq!(actual, expected.to_string());
    }

    #[test]
    fn creates_ascending_major_scale() {
        test_case("Ab", Major, Ascending, "Ab Bb C Db Eb F G Ab");
        test_case("A", Major, Ascending, "A B C# D E F# G# A");
        test_case("A#", Major, Ascending, "A# B# C## D# E# F## G## A#");

        test_case("Bb", Major, Ascending, "Bb C D Eb F G A Bb");
    }

    #[test]
    fn creates_descending_major_scale() {
        test_case("A", Major, Descending, "A G# F# E D C# B A");
    }

    #[test]
    fn creates_ascending_descending_major_scale() {
        test_case(
            "Ab",
            Major,
            AscendingDescending,
            "Ab Bb C Db Eb F G Ab G F Eb Db C Bb Ab",
        );
        test_case(
            "A",
            Major,
            AscendingDescending,
            "A B C# D E F# G# A G# F# E D C# B A",
        );
        test_case(
            "A#",
            Major,
            AscendingDescending,
            "A# B# C## D# E# F## G## A# G## F## E# D# C## B# A#",
        );

        test_case(
            "Bb",
            Major,
            AscendingDescending,
            "Bb C D Eb F G A Bb A G F Eb D C Bb",
        );
    }

    #[test]
    fn creates_descending_ascending_major_scale() {
        test_case(
            "A",
            Major,
            DescendingAscending,
            "A G# F# E D C# B A B C# D E F# G# A",
        );
    }

    #[test]
    fn creates_ascending_melodic_minor_scale() {
        test_case("G", MelodicMinor, Ascending, "G A Bb C D E F# G");
    }

    #[test]
    fn creates_descending_melodic_minor_scale() {
        test_case("G", MelodicMinor, Descending, "G F Eb D C Bb A G");
    }
}
