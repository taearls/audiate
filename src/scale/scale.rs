#![allow(dead_code)]
use crate::note::Note;

use super::ScaleKind;

pub struct Scale {
    kind: ScaleKind,
    notes: Vec<Note>,
}

impl Scale {
    pub fn new(root_note: Note, kind: ScaleKind) -> Self {
        Self {
            notes: vec![root_note; 7],
            kind,
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
            result.push_str(note.into());
            if index < self.notes.len() - 1 {
                result.push(' ');
                index += 1;
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
    fn creates_new_scale() {
        let actual = Scale::new(Note::try_from("A").unwrap(), ScaleKind::Major).print();
        let expected = String::from("A A A A A A A");
        assert_eq!(actual, expected);
    }
}
