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
        let mut result: Vec<Note> = vec![root_note];

        match direction {
            ScaleDirection::Ascending => {
                for interval in kind.intervals(direction) {
                    let next = result.last().unwrap().by_interval_ascending(interval);
                    result.push(next);
                }
            }
            ScaleDirection::Descending => {
                for interval in kind.intervals(direction) {
                    let next = result.last().unwrap().by_interval_descending(interval);
                    result.push(next);
                }
            }
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

    #[test]
    fn creates_ascending_major_scale() {
        let actual = Scale::new(
            Note::try_from("A").unwrap(),
            ScaleKind::Major,
            ScaleDirection::Ascending,
        )
        .print();
        let expected = String::from("A B C# D E F# G# A");
        assert_eq!(actual, expected);
    }

    #[test]
    fn creates_descending_major_scale() {
        let actual = Scale::new(
            Note::try_from("A").unwrap(),
            ScaleKind::Major,
            ScaleDirection::Descending,
        )
        .print();
        let expected = String::from("A G# F# E D C# B A");
        assert_eq!(actual, expected);
    }

    #[test]
    fn creates_ascending_melodic_minor_scale() {
        let actual = Scale::new(
            Note::try_from("G").unwrap(),
            ScaleKind::MelodicMinor,
            ScaleDirection::Ascending,
        )
        .print();
        let expected = String::from("G A Bb C D E F# G");
        assert_eq!(actual, expected);
    }

    #[test]
    fn creates_descending_melodic_minor_scale() {
        let actual = Scale::new(
            Note::try_from("G").unwrap(),
            ScaleKind::MelodicMinor,
            ScaleDirection::Descending,
        )
        .print();
        let expected = String::from("G F Eb D C Bb A G");
        assert_eq!(actual, expected);
    }
}
