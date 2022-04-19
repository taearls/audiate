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
            notes: Scale::notes_from_root(root_note, kind, direction),
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

    pub fn direction(self) -> ScaleDirection {
        self.direction
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

    fn notes_from_root(root_note: Note, kind: ScaleKind, direction: ScaleDirection) -> Vec<Note> {
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
                let second_half = Scale::notes_from_root(root_note, kind, Descending);
                result.extend_from_slice(&second_half[1..]);
            }
            DescendingAscending => {
                let second_half = Scale::notes_from_root(root_note, kind, Ascending);
                result.extend_from_slice(&second_half[1..]);
            }
            _ => {}
        }
        result
    }
}
