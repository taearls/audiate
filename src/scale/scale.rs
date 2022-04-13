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

    fn notes_from_root(
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

    mod ascending_scales {
        use super::*;

        fn ascending_scale_test_case(root_note_str: &str, kind: ScaleKind, expected: &str) {
            test_case(root_note_str, kind, Ascending, expected);
        }

        #[test]
        fn creates_ascending_major_scale() {
            ascending_scale_test_case("Ab", Major, "Ab Bb C Db Eb F G Ab");
            ascending_scale_test_case("A", Major, "A B C# D E F# G# A");
            ascending_scale_test_case("A#", Major, "A# B# C## D# E# F## G## A#");

            ascending_scale_test_case("Bb", Major, "Bb C D Eb F G A Bb");
            ascending_scale_test_case("B", Major, "B C# D# E F# G# A# B");
            ascending_scale_test_case("B#", Major, "B# C## D## E# F## G## A## B#");

            ascending_scale_test_case("Cb", Major, "Cb Db Eb Fb Gb Ab Bb Cb");
            ascending_scale_test_case("C", Major, "C D E F G A B C");
            ascending_scale_test_case("C#", Major, "C# D# E# F# G# A# B# C#");

            ascending_scale_test_case("Db", Major, "Db Eb F Gb Ab Bb C Db");
            ascending_scale_test_case("D", Major, "D E F# G A B C# D");
            ascending_scale_test_case("D#", Major, "D# E# F## G# A# B# C## D#");

            ascending_scale_test_case("Eb", Major, "Eb F G Ab Bb C D Eb");
            ascending_scale_test_case("E", Major, "E F# G# A B C# D# E");
            ascending_scale_test_case("E#", Major, "E# F## G## A# B# C## D## E#");

            ascending_scale_test_case("Fb", Major, "Fb Gb Ab Bbb Cb Db Eb Fb");
            ascending_scale_test_case("F", Major, "F G A Bb C D E F");
            ascending_scale_test_case("F#", Major, "F# G# A# B C# D# E# F#");

            ascending_scale_test_case("Gb", Major, "Gb Ab Bb Cb Db Eb F Gb");
            ascending_scale_test_case("G", Major, "G A B C D E F# G");
            ascending_scale_test_case("G#", Major, "G# A# B# C# D# E# F## G#");
        }

        #[test]
        fn creates_ascending_minor_scale() {
            ascending_scale_test_case("Ab", Minor, "Ab Bb Cb Db Eb Fb Gb Ab");
            ascending_scale_test_case("A", Minor, "A B C D E F G A");
            ascending_scale_test_case("A#", Minor, "A# B# C# D# E# F# G# A#");

            ascending_scale_test_case("Bb", Minor, "Bb C Db Eb F Gb Ab Bb");
            ascending_scale_test_case("B", Minor, "B C# D E F# G A B");
            ascending_scale_test_case("B#", Minor, "B# C## D# E# F## G# A# B#");

            ascending_scale_test_case("Cb", Minor, "Cb Db Ebb Fb Gb Abb Bbb Cb");
            ascending_scale_test_case("C", Minor, "C D Eb F G Ab Bb C");
            ascending_scale_test_case("C#", Minor, "C# D# E F# G# A B C#");

            ascending_scale_test_case("Db", Minor, "Db Eb Fb Gb Ab Bbb Cb Db");
            ascending_scale_test_case("D", Minor, "D E F G A Bb C D");
            ascending_scale_test_case("D#", Minor, "D# E# F# G# A# B C# D#");

            ascending_scale_test_case("Eb", Minor, "Eb F Gb Ab Bb Cb Db Eb");
            ascending_scale_test_case("E", Minor, "E F# G A B C D E");
            ascending_scale_test_case("E#", Minor, "E# F## G# A# B# C# D# E#");

            ascending_scale_test_case("Fb", Minor, "Fb Gb Abb Bbb Cb Dbb Ebb Fb");
            ascending_scale_test_case("F", Minor, "F G Ab Bb C Db Eb F");
            ascending_scale_test_case("F#", Minor, "F# G# A B C# D E F#");

            ascending_scale_test_case("Gb", Minor, "Gb Ab Bbb Cb Db Ebb Fb Gb");
            ascending_scale_test_case("G", Minor, "G A Bb C D Eb F G");
            ascending_scale_test_case("G#", Minor, "G# A# B C# D# E F# G#");
        }

        #[test]
        fn creates_ascending_ionian_scale() {
            ascending_scale_test_case("Ab", Ionian, "Ab Bb C Db Eb F G Ab");
            ascending_scale_test_case("A", Ionian, "A B C# D E F# G# A");
            ascending_scale_test_case("A#", Ionian, "A# B# C## D# E# F## G## A#");

            ascending_scale_test_case("Bb", Ionian, "Bb C D Eb F G A Bb");
            ascending_scale_test_case("B", Ionian, "B C# D# E F# G# A# B");
            ascending_scale_test_case("B#", Ionian, "B# C## D## E# F## G## A## B#");

            ascending_scale_test_case("Cb", Ionian, "Cb Db Eb Fb Gb Ab Bb Cb");
            ascending_scale_test_case("C", Ionian, "C D E F G A B C");
            ascending_scale_test_case("C#", Ionian, "C# D# E# F# G# A# B# C#");

            ascending_scale_test_case("Db", Ionian, "Db Eb F Gb Ab Bb C Db");
            ascending_scale_test_case("D", Ionian, "D E F# G A B C# D");
            ascending_scale_test_case("D#", Ionian, "D# E# F## G# A# B# C## D#");

            ascending_scale_test_case("Eb", Ionian, "Eb F G Ab Bb C D Eb");
            ascending_scale_test_case("E", Ionian, "E F# G# A B C# D# E");
            ascending_scale_test_case("E#", Ionian, "E# F## G## A# B# C## D## E#");

            ascending_scale_test_case("Fb", Ionian, "Fb Gb Ab Bbb Cb Db Eb Fb");
            ascending_scale_test_case("F", Ionian, "F G A Bb C D E F");
            ascending_scale_test_case("F#", Ionian, "F# G# A# B C# D# E# F#");

            ascending_scale_test_case("Gb", Ionian, "Gb Ab Bb Cb Db Eb F Gb");
            ascending_scale_test_case("G", Ionian, "G A B C D E F# G");
            ascending_scale_test_case("G#", Ionian, "G# A# B# C# D# E# F## G#");
        }

        #[test]
        fn creates_ascending_dorian_scale() {
            ascending_scale_test_case("Ab", Dorian, "Ab Bb Cb Db Eb F Gb Ab");
            ascending_scale_test_case("A", Dorian, "A B C D E F# G A");
            ascending_scale_test_case("A#", Dorian, "A# B# C# D# E# F## G# A#");

            ascending_scale_test_case("Bb", Dorian, "Bb C Db Eb F G Ab Bb");
            ascending_scale_test_case("B", Dorian, "B C# D E F# G# A B");
            ascending_scale_test_case("B#", Dorian, "B# C## D# E# F## G## A# B#");

            ascending_scale_test_case("Cb", Dorian, "Cb Db Ebb Fb Gb Ab Bbb Cb");
            ascending_scale_test_case("C", Dorian, "C D Eb F G A Bb C");
            ascending_scale_test_case("C#", Dorian, "C# D# E F# G# A# B C#");

            ascending_scale_test_case("Db", Dorian, "Db Eb Fb Gb Ab Bb Cb Db");
            ascending_scale_test_case("D", Dorian, "D E F G A B C D");
            ascending_scale_test_case("D#", Dorian, "D# E# F# G# A# B# C# D#");

            ascending_scale_test_case("Eb", Dorian, "Eb F Gb Ab Bb C Db Eb");
            ascending_scale_test_case("E", Dorian, "E F# G A B C# D E");
            ascending_scale_test_case("E#", Dorian, "E# F## G# A# B# C## D# E#");

            ascending_scale_test_case("Fb", Dorian, "Fb Gb Abb Bbb Cb Db Ebb Fb");
            ascending_scale_test_case("F", Dorian, "F G Ab Bb C D Eb F");
            ascending_scale_test_case("F#", Dorian, "F# G# A B C# D# E F#");

            ascending_scale_test_case("Gb", Dorian, "Gb Ab Bbb Cb Db Eb Fb Gb");
            ascending_scale_test_case("G", Dorian, "G A Bb C D E F G");
            ascending_scale_test_case("G#", Dorian, "G# A# B C# D# E# F# G#");
        }

        #[test]
        fn creates_ascending_phrygian_scale() {
            ascending_scale_test_case("Ab", Phrygian, "Ab Bbb Cb Db Eb Fb Gb Ab");
            ascending_scale_test_case("A", Phrygian, "A Bb C D E F G A");
            ascending_scale_test_case("A#", Phrygian, "A# B C# D# E# F# G# A#");

            ascending_scale_test_case("Bb", Phrygian, "Bb Cb Db Eb F Gb Ab Bb");
            ascending_scale_test_case("B", Phrygian, "B C D E F# G A B");
            ascending_scale_test_case("B#", Phrygian, "B# C# D# E# F## G# A# B#");

            ascending_scale_test_case("Cb", Phrygian, "Cb Dbb Ebb Fb Gb Abb Bbb Cb");
            ascending_scale_test_case("C", Phrygian, "C Db Eb F G Ab Bb C");
            ascending_scale_test_case("C#", Phrygian, "C# D E F# G# A B C#");

            ascending_scale_test_case("Db", Phrygian, "Db Ebb Fb Gb Ab Bbb Cb Db");
            ascending_scale_test_case("D", Phrygian, "D Eb F G A Bb C D");
            ascending_scale_test_case("D#", Phrygian, "D# E F# G# A# B C# D#");

            ascending_scale_test_case("Eb", Phrygian, "Eb Fb Gb Ab Bb Cb Db Eb");
            ascending_scale_test_case("E", Phrygian, "E F G A B C D E");
            ascending_scale_test_case("E#", Phrygian, "E# F# G# A# B# C# D# E#");

            ascending_scale_test_case("Fb", Phrygian, "Fb Gbb Abb Bbb Cb Dbb Ebb Fb");
            ascending_scale_test_case("F", Phrygian, "F Gb Ab Bb C Db Eb F");
            ascending_scale_test_case("F#", Phrygian, "F# G A B C# D E F#");

            ascending_scale_test_case("Gb", Phrygian, "Gb Abb Bbb Cb Db Ebb Fb Gb");
            ascending_scale_test_case("G", Phrygian, "G Ab Bb C D Eb F G");
            ascending_scale_test_case("G#", Phrygian, "G# A B C# D# E F# G#");
        }

        #[test]
        fn creates_ascending_lydian_scale() {
            ascending_scale_test_case("Ab", Lydian, "Ab Bb C D Eb F G Ab");
            ascending_scale_test_case("A", Lydian, "A B C# D# E F# G# A");
            ascending_scale_test_case("A#", Lydian, "A# B# C## D## E# F## G## A#");

            ascending_scale_test_case("Bb", Lydian, "Bb C D E F G A Bb");
            ascending_scale_test_case("B", Lydian, "B C# D# E# F# G# A# B");
            ascending_scale_test_case("B#", Lydian, "B# C## D## E## F## G## A## B#");

            ascending_scale_test_case("Cb", Lydian, "Cb Db Eb F Gb Ab Bb Cb");
            ascending_scale_test_case("C", Lydian, "C D E F# G A B C");
            ascending_scale_test_case("C#", Lydian, "C# D# E# F## G# A# B# C#");

            ascending_scale_test_case("Db", Lydian, "Db Eb F G Ab Bb C Db");
            ascending_scale_test_case("D", Lydian, "D E F# G# A B C# D");
            ascending_scale_test_case("D#", Lydian, "D# E# F## G## A# B# C## D#");

            ascending_scale_test_case("Eb", Lydian, "Eb F G A Bb C D Eb");
            ascending_scale_test_case("E", Lydian, "E F# G# A# B C# D# E");
            ascending_scale_test_case("E#", Lydian, "E# F## G## A## B# C## D## E#");

            ascending_scale_test_case("Fb", Lydian, "Fb Gb Ab Bb Cb Db Eb Fb");
            ascending_scale_test_case("F", Lydian, "F G A B C D E F");
            ascending_scale_test_case("F#", Lydian, "F# G# A# B# C# D# E# F#");

            ascending_scale_test_case("Gb", Lydian, "Gb Ab Bb C Db Eb F Gb");
            ascending_scale_test_case("G", Lydian, "G A B C# D E F# G");
            ascending_scale_test_case("G#", Lydian, "G# A# B# C## D# E# F## G#");
        }

        #[test]
        fn creates_ascending_mixolydian_scale() {
            ascending_scale_test_case("Ab", Mixolydian, "Ab Bb C Db Eb F Gb Ab");
            ascending_scale_test_case("A", Mixolydian, "A B C# D E F# G A");
            ascending_scale_test_case("A#", Mixolydian, "A# B# C## D# E# F## G# A#");

            ascending_scale_test_case("Bb", Mixolydian, "Bb C D Eb F G Ab Bb");
            ascending_scale_test_case("B", Mixolydian, "B C# D# E F# G# A B");
            ascending_scale_test_case("B#", Mixolydian, "B# C## D## E# F## G## A# B#");

            ascending_scale_test_case("Cb", Mixolydian, "Cb Db Eb Fb Gb Ab Bbb Cb");
            ascending_scale_test_case("C", Mixolydian, "C D E F G A Bb C");
            ascending_scale_test_case("C#", Mixolydian, "C# D# E# F# G# A# B C#");

            ascending_scale_test_case("Db", Mixolydian, "Db Eb F Gb Ab Bb Cb Db");
            ascending_scale_test_case("D", Mixolydian, "D E F# G A B C D");
            ascending_scale_test_case("D#", Mixolydian, "D# E# F## G# A# B# C# D#");

            ascending_scale_test_case("Eb", Mixolydian, "Eb F G Ab Bb C Db Eb");
            ascending_scale_test_case("E", Mixolydian, "E F# G# A B C# D E");
            ascending_scale_test_case("E#", Mixolydian, "E# F## G## A# B# C## D# E#");

            ascending_scale_test_case("Fb", Mixolydian, "Fb Gb Ab Bbb Cb Db Ebb Fb");
            ascending_scale_test_case("F", Mixolydian, "F G A Bb C D Eb F");
            ascending_scale_test_case("F#", Mixolydian, "F# G# A# B C# D# E F#");

            ascending_scale_test_case("Gb", Mixolydian, "Gb Ab Bb Cb Db Eb Fb Gb");
            ascending_scale_test_case("G", Mixolydian, "G A B C D E F G");
            ascending_scale_test_case("G#", Mixolydian, "G# A# B# C# D# E# F# G#");
        }

        #[test]
        fn creates_ascending_aeolian_scale() {
            ascending_scale_test_case("Ab", Aeolian, "Ab Bb Cb Db Eb Fb Gb Ab");
            ascending_scale_test_case("A", Aeolian, "A B C D E F G A");
            ascending_scale_test_case("A#", Aeolian, "A# B# C# D# E# F# G# A#");

            ascending_scale_test_case("Bb", Aeolian, "Bb C Db Eb F Gb Ab Bb");
            ascending_scale_test_case("B", Aeolian, "B C# D E F# G A B");
            ascending_scale_test_case("B#", Aeolian, "B# C## D# E# F## G# A# B#");

            ascending_scale_test_case("Cb", Aeolian, "Cb Db Ebb Fb Gb Abb Bbb Cb");
            ascending_scale_test_case("C", Aeolian, "C D Eb F G Ab Bb C");
            ascending_scale_test_case("C#", Aeolian, "C# D# E F# G# A B C#");

            ascending_scale_test_case("Db", Aeolian, "Db Eb Fb Gb Ab Bbb Cb Db");
            ascending_scale_test_case("D", Aeolian, "D E F G A Bb C D");
            ascending_scale_test_case("D#", Aeolian, "D# E# F# G# A# B C# D#");

            ascending_scale_test_case("Eb", Aeolian, "Eb F Gb Ab Bb Cb Db Eb");
            ascending_scale_test_case("E", Aeolian, "E F# G A B C D E");
            ascending_scale_test_case("E#", Aeolian, "E# F## G# A# B# C# D# E#");

            ascending_scale_test_case("Fb", Aeolian, "Fb Gb Abb Bbb Cb Dbb Ebb Fb");
            ascending_scale_test_case("F", Aeolian, "F G Ab Bb C Db Eb F");
            ascending_scale_test_case("F#", Aeolian, "F# G# A B C# D E F#");

            ascending_scale_test_case("Gb", Aeolian, "Gb Ab Bbb Cb Db Ebb Fb Gb");
            ascending_scale_test_case("G", Aeolian, "G A Bb C D Eb F G");
            ascending_scale_test_case("G#", Aeolian, "G# A# B C# D# E F# G#");
        }

        #[test]
        fn creates_ascending_locrian_scale() {
            ascending_scale_test_case("Ab", Locrian, "Ab Bbb Cb Db Ebb Fb Gb Ab");
            ascending_scale_test_case("A", Locrian, "A Bb C D Eb F G A");
            ascending_scale_test_case("A#", Locrian, "A# B C# D# E F# G# A#");

            ascending_scale_test_case("Bb", Locrian, "Bb Cb Db Eb Fb Gb Ab Bb");
            ascending_scale_test_case("B", Locrian, "B C D E F G A B");
            ascending_scale_test_case("B#", Locrian, "B# C# D# E# F# G# A# B#");

            ascending_scale_test_case("Cb", Locrian, "Cb Dbb Ebb Fb Gbb Abb Bbb Cb");
            ascending_scale_test_case("C", Locrian, "C Db Eb F Gb Ab Bb C");
            ascending_scale_test_case("C#", Locrian, "C# D E F# G A B C#");

            ascending_scale_test_case("Db", Locrian, "Db Ebb Fb Gb Abb Bbb Cb Db");
            ascending_scale_test_case("D", Locrian, "D Eb F G Ab Bb C D");
            ascending_scale_test_case("D#", Locrian, "D# E F# G# A B C# D#");

            ascending_scale_test_case("Eb", Locrian, "Eb Fb Gb Ab Bbb Cb Db Eb");
            ascending_scale_test_case("E", Locrian, "E F G A Bb C D E");
            ascending_scale_test_case("E#", Locrian, "E# F# G# A# B C# D# E#");

            ascending_scale_test_case("Fb", Locrian, "Fb Gbb Abb Bbb Cbb Dbb Ebb Fb");
            ascending_scale_test_case("F", Locrian, "F Gb Ab Bb Cb Db Eb F");
            ascending_scale_test_case("F#", Locrian, "F# G A B C D E F#");

            ascending_scale_test_case("Gb", Locrian, "Gb Abb Bbb Cb Dbb Ebb Fb Gb");
            ascending_scale_test_case("G", Locrian, "G Ab Bb C Db Eb F G");
            ascending_scale_test_case("G#", Locrian, "G# A B C# D E F# G#");
        }

        #[test]
        fn creates_ascending_major_pentatonic_scale() {
            ascending_scale_test_case("Ab", MajorPentatonic, "Ab Bb C Eb F Ab");
            ascending_scale_test_case("A", MajorPentatonic, "A B C# E F# A");
            ascending_scale_test_case("A#", MajorPentatonic, "A# B# C## E# F## A#");

            ascending_scale_test_case("Bb", MajorPentatonic, "Bb C D F G Bb");
            ascending_scale_test_case("B", MajorPentatonic, "B C# D# F# G# B");
            ascending_scale_test_case("B#", MajorPentatonic, "B# C## D## F## G## B#");

            ascending_scale_test_case("Cb", MajorPentatonic, "Cb Db Eb Gb Ab Cb");
            ascending_scale_test_case("C", MajorPentatonic, "C D E G A C");
            ascending_scale_test_case("C#", MajorPentatonic, "C# D# E# G# A# C#");

            ascending_scale_test_case("Db", MajorPentatonic, "Db Eb F Ab Bb Db");
            ascending_scale_test_case("D", MajorPentatonic, "D E F# A B D");
            ascending_scale_test_case("D#", MajorPentatonic, "D# E# F## A# B# D#");

            ascending_scale_test_case("Eb", MajorPentatonic, "Eb F G Bb C Eb");
            ascending_scale_test_case("E", MajorPentatonic, "E F# G# B C# E");
            ascending_scale_test_case("E#", MajorPentatonic, "E# F## G## B# C## E#");

            ascending_scale_test_case("Fb", MajorPentatonic, "Fb Gb Ab Cb Db Fb");
            ascending_scale_test_case("F", MajorPentatonic, "F G A C D F");
            ascending_scale_test_case("F#", MajorPentatonic, "F# G# A# C# D# F#");

            ascending_scale_test_case("Gb", MajorPentatonic, "Gb Ab Bb Db Eb Gb");
            ascending_scale_test_case("G", MajorPentatonic, "G A B D E G");
            ascending_scale_test_case("G#", MajorPentatonic, "G# A# B# D# E# G#");
        }

        #[test]
        fn creates_ascending_minor_pentatonic_scale() {
            ascending_scale_test_case("Ab", MinorPentatonic, "Ab Cb Db Eb Gb Ab");
            ascending_scale_test_case("A", MinorPentatonic, "A C D E G A");
            ascending_scale_test_case("A#", MinorPentatonic, "A# C# D# E# G# A#");

            ascending_scale_test_case("Bb", MinorPentatonic, "Bb Db Eb F Ab Bb");
            ascending_scale_test_case("B", MinorPentatonic, "B D E F# A B");
            ascending_scale_test_case("B#", MinorPentatonic, "B# D# E# F## A# B#");

            ascending_scale_test_case("Cb", MinorPentatonic, "Cb Ebb Fb Gb Bbb Cb");
            ascending_scale_test_case("C", MinorPentatonic, "C Eb F G Bb C");
            ascending_scale_test_case("C#", MinorPentatonic, "C# E F# G# B C#");

            ascending_scale_test_case("Db", MinorPentatonic, "Db Fb Gb Ab Cb Db");
            ascending_scale_test_case("D", MinorPentatonic, "D F G A C D");
            ascending_scale_test_case("D#", MinorPentatonic, "D# F# G# A# C# D#");

            ascending_scale_test_case("Eb", MinorPentatonic, "Eb Gb Ab Bb Db Eb");
            ascending_scale_test_case("E", MinorPentatonic, "E G A B D E");
            ascending_scale_test_case("E#", MinorPentatonic, "E# G# A# B# D# E#");

            ascending_scale_test_case("Fb", MinorPentatonic, "Fb Abb Bbb Cb Ebb Fb");
            ascending_scale_test_case("F", MinorPentatonic, "F Ab Bb C Eb F");
            ascending_scale_test_case("F#", MinorPentatonic, "F# A B C# E F#");

            ascending_scale_test_case("Gb", MinorPentatonic, "Gb Bbb Cb Db Fb Gb");
            ascending_scale_test_case("G", MinorPentatonic, "G Bb C D F G");
            ascending_scale_test_case("G#", MinorPentatonic, "G# B C# D# F# G#");
        }

        #[test]
        fn creates_ascending_harmonic_minor_scale() {
            ascending_scale_test_case("Ab", HarmonicMinor, "Ab Bb Cb Db Eb Fb G Ab");
            ascending_scale_test_case("A", HarmonicMinor, "A B C D E F G# A");
            ascending_scale_test_case("A#", HarmonicMinor, "A# B# C# D# E# F# G## A#");

            ascending_scale_test_case("Bb", HarmonicMinor, "Bb C Db Eb F Gb A Bb");
            ascending_scale_test_case("B", HarmonicMinor, "B C# D E F# G A# B");
            ascending_scale_test_case("B#", HarmonicMinor, "B# C## D# E# F## G# A## B#");

            ascending_scale_test_case("Cb", HarmonicMinor, "Cb Db Ebb Fb Gb Abb Bb Cb");
            ascending_scale_test_case("C", HarmonicMinor, "C D Eb F G Ab B C");
            ascending_scale_test_case("C#", HarmonicMinor, "C# D# E F# G# A B# C#");

            ascending_scale_test_case("Db", HarmonicMinor, "Db Eb Fb Gb Ab Bbb C Db");
            ascending_scale_test_case("D", HarmonicMinor, "D E F G A Bb C# D");
            ascending_scale_test_case("D#", HarmonicMinor, "D# E# F# G# A# B C## D#");

            ascending_scale_test_case("Eb", HarmonicMinor, "Eb F Gb Ab Bb Cb D Eb");
            ascending_scale_test_case("E", HarmonicMinor, "E F# G A B C D# E");
            ascending_scale_test_case("E#", HarmonicMinor, "E# F## G# A# B# C# D## E#");

            ascending_scale_test_case("Fb", HarmonicMinor, "Fb Gb Abb Bbb Cb Dbb Eb Fb");
            ascending_scale_test_case("F", HarmonicMinor, "F G Ab Bb C Db E F");
            ascending_scale_test_case("F#", HarmonicMinor, "F# G# A B C# D E# F#");

            ascending_scale_test_case("Gb", HarmonicMinor, "Gb Ab Bbb Cb Db Ebb F Gb");
            ascending_scale_test_case("G", HarmonicMinor, "G A Bb C D Eb F# G");
            ascending_scale_test_case("G#", HarmonicMinor, "G# A# B C# D# E F## G#");
        }

        #[test]
        fn creates_ascending_melodic_minor_scale() {
            ascending_scale_test_case("Ab", MelodicMinor, "Ab Bb Cb Db Eb F G Ab");
            ascending_scale_test_case("A", MelodicMinor, "A B C D E F# G# A");
            ascending_scale_test_case("A#", MelodicMinor, "A# B# C# D# E# F## G## A#");

            ascending_scale_test_case("Bb", MelodicMinor, "Bb C Db Eb F G A Bb");
            ascending_scale_test_case("B", MelodicMinor, "B C# D E F# G# A# B");
            ascending_scale_test_case("B#", MelodicMinor, "B# C## D# E# F## G## A## B#");

            ascending_scale_test_case("Cb", MelodicMinor, "Cb Db Ebb Fb Gb Ab Bb Cb");
            ascending_scale_test_case("C", MelodicMinor, "C D Eb F G A B C");
            ascending_scale_test_case("C#", MelodicMinor, "C# D# E F# G# A# B# C#");

            ascending_scale_test_case("Db", MelodicMinor, "Db Eb Fb Gb Ab Bb C Db");
            ascending_scale_test_case("D", MelodicMinor, "D E F G A B C# D");
            ascending_scale_test_case("D#", MelodicMinor, "D# E# F# G# A# B# C## D#");

            ascending_scale_test_case("Eb", MelodicMinor, "Eb F Gb Ab Bb C D Eb");
            ascending_scale_test_case("E", MelodicMinor, "E F# G A B C# D# E");
            ascending_scale_test_case("E#", MelodicMinor, "E# F## G# A# B# C## D## E#");

            ascending_scale_test_case("Fb", MelodicMinor, "Fb Gb Abb Bbb Cb Db Eb Fb");
            ascending_scale_test_case("F", MelodicMinor, "F G Ab Bb C D E F");
            ascending_scale_test_case("F#", MelodicMinor, "F# G# A B C# D# E# F#");

            ascending_scale_test_case("Gb", MelodicMinor, "Gb Ab Bbb Cb Db Eb F Gb");
            ascending_scale_test_case("G", MelodicMinor, "G A Bb C D E F# G");
            ascending_scale_test_case("G#", MelodicMinor, "G# A# B C# D# E# F## G#");
        }
    }

    mod descending_scales {
        use super::*;

        fn descending_test_case(root_note_str: &str, kind: ScaleKind, expected: &str) {
            test_case(root_note_str, kind, Descending, expected);
        }

        #[test]
        fn creates_descending_major_scale() {
            descending_test_case("Ab", Major, "Ab G F Eb Db C Bb Ab");
            descending_test_case("A", Major, "A G# F# E D C# B A");
            descending_test_case("A#", Major, "A# G## F## E# D# C## B# A#");

            descending_test_case("Bb", Major, "Bb A G F Eb D C Bb");
            descending_test_case("B", Major, "B A# G# F# E D# C# B");
            descending_test_case("B#", Major, "B# A## G## F## E# D## C## B#");

            descending_test_case("Cb", Major, "Cb Bb Ab Gb Fb Eb Db Cb");
            descending_test_case("C", Major, "C B A G F E D C");
            descending_test_case("C#", Major, "C# B# A# G# F# E# D# C#");

            descending_test_case("Db", Major, "Db C Bb Ab Gb F Eb Db");
            descending_test_case("D", Major, "D C# B A G F# E D");
            descending_test_case("D#", Major, "D# C## B# A# G# F## E# D#");

            descending_test_case("Eb", Major, "Eb D C Bb Ab G F Eb");
            descending_test_case("E", Major, "E D# C# B A G# F# E");
            descending_test_case("E#", Major, "E# D## C## B# A# G## F## E#");

            descending_test_case("Fb", Major, "Fb Eb Db Cb Bbb Ab Gb Fb");
            descending_test_case("F", Major, "F E D C Bb A G F");
            descending_test_case("F#", Major, "F# E# D# C# B A# G# F#");

            descending_test_case("Gb", Major, "Gb F Eb Db Cb Bb Ab Gb");
            descending_test_case("G", Major, "G F# E D C B A G");
            descending_test_case("G#", Major, "G# F## E# D# C# B# A# G#");
        }

        #[test]
        fn creates_descending_minor_scale() {
            descending_test_case("Ab", Minor, "Ab Gb Fb Eb Db Cb Bb Ab");
            descending_test_case("A", Minor, "A G F E D C B A");
            descending_test_case("A#", Minor, "A# G# F# E# D# C# B# A#");

            descending_test_case("Bb", Minor, "Bb Ab Gb F Eb Db C Bb");
            descending_test_case("B", Minor, "B A G F# E D C# B");
            descending_test_case("B#", Minor, "B# A# G# F## E# D# C## B#");

            descending_test_case("Cb", Minor, "Cb Bbb Abb Gb Fb Ebb Db Cb");
            descending_test_case("C", Minor, "C Bb Ab G F Eb D C");
            descending_test_case("C#", Minor, "C# B A G# F# E D# C#");

            descending_test_case("Db", Minor, "Db Cb Bbb Ab Gb Fb Eb Db");
            descending_test_case("D", Minor, "D C Bb A G F E D");
            descending_test_case("D#", Minor, "D# C# B A# G# F# E# D#");

            descending_test_case("Eb", Minor, "Eb Db Cb Bb Ab Gb F Eb");
            descending_test_case("E", Minor, "E D C B A G F# E");
            descending_test_case("E#", Minor, "E# D# C# B# A# G# F## E#");

            descending_test_case("Fb", Minor, "Fb Ebb Dbb Cb Bbb Abb Gb Fb");
            descending_test_case("F", Minor, "F Eb Db C Bb Ab G F");
            descending_test_case("F#", Minor, "F# E D C# B A G# F#");

            descending_test_case("Gb", Minor, "Gb Fb Ebb Db Cb Bbb Ab Gb");
            descending_test_case("G", Minor, "G F Eb D C Bb A G");
            descending_test_case("G#", Minor, "G# F# E D# C# B A# G#");
        }

        #[test]
        fn creates_descending_ionian_scale() {
            descending_test_case("Ab", Ionian, "Ab G F Eb Db C Bb Ab");
            descending_test_case("A", Ionian, "A G# F# E D C# B A");
            descending_test_case("A#", Ionian, "A# G## F## E# D# C## B# A#");

            descending_test_case("Bb", Ionian, "Bb A G F Eb D C Bb");
            descending_test_case("B", Ionian, "B A# G# F# E D# C# B");
            descending_test_case("B#", Ionian, "B# A## G## F## E# D## C## B#");

            descending_test_case("Cb", Ionian, "Cb Bb Ab Gb Fb Eb Db Cb");
            descending_test_case("C", Ionian, "C B A G F E D C");
            descending_test_case("C#", Ionian, "C# B# A# G# F# E# D# C#");

            descending_test_case("Db", Ionian, "Db C Bb Ab Gb F Eb Db");
            descending_test_case("D", Ionian, "D C# B A G F# E D");
            descending_test_case("D#", Ionian, "D# C## B# A# G# F## E# D#");

            descending_test_case("Eb", Ionian, "Eb D C Bb Ab G F Eb");
            descending_test_case("E", Ionian, "E D# C# B A G# F# E");
            descending_test_case("E#", Ionian, "E# D## C## B# A# G## F## E#");

            descending_test_case("Fb", Ionian, "Fb Eb Db Cb Bbb Ab Gb Fb");
            descending_test_case("F", Ionian, "F E D C Bb A G F");
            descending_test_case("F#", Ionian, "F# E# D# C# B A# G# F#");

            descending_test_case("Gb", Ionian, "Gb F Eb Db Cb Bb Ab Gb");
            descending_test_case("G", Ionian, "G F# E D C B A G");
            descending_test_case("G#", Ionian, "G# F## E# D# C# B# A# G#");
        }

        #[test]
        fn creates_descending_dorian_scale() {
            descending_test_case("Ab", Dorian, "Ab Gb F Eb Db Cb Bb Ab");
            descending_test_case("A", Dorian, "A G F# E D C B A");
            descending_test_case("A#", Dorian, "A# G# F## E# D# C# B# A#");

            descending_test_case("Bb", Dorian, "Bb Ab G F Eb Db C Bb");
            descending_test_case("B", Dorian, "B A G# F# E D C# B");
            descending_test_case("B#", Dorian, "B# A# G## F## E# D# C## B#");

            descending_test_case("Cb", Dorian, "Cb Bbb Ab Gb Fb Ebb Db Cb");
            descending_test_case("C", Dorian, "C Bb A G F Eb D C");
            descending_test_case("C#", Dorian, "C# B A# G# F# E D# C#");

            descending_test_case("Db", Dorian, "Db Cb Bb Ab Gb Fb Eb Db");
            descending_test_case("D", Dorian, "D C B A G F E D");
            descending_test_case("D#", Dorian, "D# C# B# A# G# F# E# D#");

            descending_test_case("Eb", Dorian, "Eb Db C Bb Ab Gb F Eb");
            descending_test_case("E", Dorian, "E D C# B A G F# E");
            descending_test_case("E#", Dorian, "E# D# C## B# A# G# F## E#");

            descending_test_case("Fb", Dorian, "Fb Ebb Db Cb Bbb Abb Gb Fb");
            descending_test_case("F", Dorian, "F Eb D C Bb Ab G F");
            descending_test_case("F#", Dorian, "F# E D# C# B A G# F#");

            descending_test_case("Gb", Dorian, "Gb Fb Eb Db Cb Bbb Ab Gb");
            descending_test_case("G", Dorian, "G F E D C Bb A G");
            descending_test_case("G#", Dorian, "G# F# E# D# C# B A# G#");
        }

        #[test]
        fn creates_descending_phrygian_scale() {
            descending_test_case("Ab", Phrygian, "Ab Gb Fb Eb Db Cb Bbb Ab");
            descending_test_case("A", Phrygian, "A G F E D C Bb A");
            descending_test_case("A#", Phrygian, "A# G# F# E# D# C# B A#");

            descending_test_case("Bb", Phrygian, "Bb Ab Gb F Eb Db Cb Bb");
            descending_test_case("B", Phrygian, "B A G F# E D C B");
            descending_test_case("B#", Phrygian, "B# A# G# F## E# D# C# B#");

            descending_test_case("Cb", Phrygian, "Cb Bbb Abb Gb Fb Ebb Dbb Cb");
            descending_test_case("C", Phrygian, "C Bb Ab G F Eb Db C");
            descending_test_case("C#", Phrygian, "C# B A G# F# E D C#");

            descending_test_case("Db", Phrygian, "Db Cb Bbb Ab Gb Fb Ebb Db");
            descending_test_case("D", Phrygian, "D C Bb A G F Eb D");
            descending_test_case("D#", Phrygian, "D# C# B A# G# F# E D#");

            descending_test_case("Eb", Phrygian, "Eb Db Cb Bb Ab Gb Fb Eb");
            descending_test_case("E", Phrygian, "E D C B A G F E");
            descending_test_case("E#", Phrygian, "E# D# C# B# A# G# F# E#");

            descending_test_case("Fb", Phrygian, "Fb Ebb Dbb Cb Bbb Abb Gbb Fb");
            descending_test_case("F", Phrygian, "F Eb Db C Bb Ab Gb F");
            descending_test_case("F#", Phrygian, "F# E D C# B A G F#");

            descending_test_case("Gb", Phrygian, "Gb Fb Ebb Db Cb Bbb Abb Gb");
            descending_test_case("G", Phrygian, "G F Eb D C Bb Ab G");
            descending_test_case("G#", Phrygian, "G# F# E D# C# B A G#");
        }

        #[test]
        fn creates_descending_lydian_scale() {
            descending_test_case("Ab", Lydian, "Ab G F Eb D C Bb Ab");
            descending_test_case("A", Lydian, "A G# F# E D# C# B A");
            descending_test_case("A#", Lydian, "A# G## F## E# D## C## B# A#");

            descending_test_case("Bb", Lydian, "Bb A G F E D C Bb");
            descending_test_case("B", Lydian, "B A# G# F# E# D# C# B");
            descending_test_case("B#", Lydian, "B# A## G## F## E## D## C## B#");

            descending_test_case("Cb", Lydian, "Cb Bb Ab Gb F Eb Db Cb");
            descending_test_case("C", Lydian, "C B A G F# E D C");
            descending_test_case("C#", Lydian, "C# B# A# G# F## E# D# C#");

            descending_test_case("Db", Lydian, "Db C Bb Ab G F Eb Db");
            descending_test_case("D", Lydian, "D C# B A G# F# E D");
            descending_test_case("D#", Lydian, "D# C## B# A# G## F## E# D#");

            descending_test_case("Eb", Lydian, "Eb D C Bb A G F Eb");
            descending_test_case("E", Lydian, "E D# C# B A# G# F# E");
            descending_test_case("E#", Lydian, "E# D## C## B# A## G## F## E#");

            descending_test_case("Fb", Lydian, "Fb Eb Db Cb Bb Ab Gb Fb");
            descending_test_case("F", Lydian, "F E D C B A G F");
            descending_test_case("F#", Lydian, "F# E# D# C# B# A# G# F#");

            descending_test_case("Gb", Lydian, "Gb F Eb Db C Bb Ab Gb");
            descending_test_case("G", Lydian, "G F# E D C# B A G");
            descending_test_case("G#", Lydian, "G# F## E# D# C## B# A# G#");
        }

        #[test]
        fn creates_descending_mixolydian_scale() {
            descending_test_case("Ab", Mixolydian, "Ab Gb F Eb Db C Bb Ab");
            descending_test_case("A", Mixolydian, "A G F# E D C# B A");
            descending_test_case("A#", Mixolydian, "A# G# F## E# D# C## B# A#");

            descending_test_case("Bb", Mixolydian, "Bb Ab G F Eb D C Bb");
            descending_test_case("B", Mixolydian, "B A G# F# E D# C# B");
            descending_test_case("B#", Mixolydian, "B# A# G## F## E# D## C## B#");

            descending_test_case("Cb", Mixolydian, "Cb Bbb Ab Gb Fb Eb Db Cb");
            descending_test_case("C", Mixolydian, "C Bb A G F E D C");
            descending_test_case("C#", Mixolydian, "C# B A# G# F# E# D# C#");

            descending_test_case("Db", Mixolydian, "Db Cb Bb Ab Gb F Eb Db");
            descending_test_case("D", Mixolydian, "D C B A G F# E D");
            descending_test_case("D#", Mixolydian, "D# C# B# A# G# F## E# D#");

            descending_test_case("Eb", Mixolydian, "Eb Db C Bb Ab G F Eb");
            descending_test_case("E", Mixolydian, "E D C# B A G# F# E");
            descending_test_case("E#", Mixolydian, "E# D# C## B# A# G## F## E#");

            descending_test_case("Fb", Mixolydian, "Fb Ebb Db Cb Bbb Ab Gb Fb");
            descending_test_case("F", Mixolydian, "F Eb D C Bb A G F");
            descending_test_case("F#", Mixolydian, "F# E D# C# B A# G# F#");

            descending_test_case("Gb", Mixolydian, "Gb Fb Eb Db Cb Bb Ab Gb");
            descending_test_case("G", Mixolydian, "G F E D C B A G");
            descending_test_case("G#", Mixolydian, "G# F# E# D# C# B# A# G#");
        }

        #[test]
        fn creates_descending_aeolian_scale() {
            descending_test_case("Ab", Aeolian, "Ab Gb Fb Eb Db Cb Bb Ab");
            descending_test_case("A", Aeolian, "A G F E D C B A");
            descending_test_case("A#", Aeolian, "A# G# F# E# D# C# B# A#");

            descending_test_case("Bb", Aeolian, "Bb Ab Gb F Eb Db C Bb");
            descending_test_case("B", Aeolian, "B A G F# E D C# B");
            descending_test_case("B#", Aeolian, "B# A# G# F## E# D# C## B#");

            descending_test_case("Cb", Aeolian, "Cb Bbb Abb Gb Fb Ebb Db Cb");
            descending_test_case("C", Aeolian, "C Bb Ab G F Eb D C");
            descending_test_case("C#", Aeolian, "C# B A G# F# E D# C#");

            descending_test_case("Db", Aeolian, "Db Cb Bbb Ab Gb Fb Eb Db");
            descending_test_case("D", Aeolian, "D C Bb A G F E D");
            descending_test_case("D#", Aeolian, "D# C# B A# G# F# E# D#");

            descending_test_case("Eb", Aeolian, "Eb Db Cb Bb Ab Gb F Eb");
            descending_test_case("E", Aeolian, "E D C B A G F# E");
            descending_test_case("E#", Aeolian, "E# D# C# B# A# G# F## E#");

            descending_test_case("Fb", Aeolian, "Fb Ebb Dbb Cb Bbb Abb Gb Fb");
            descending_test_case("F", Aeolian, "F Eb Db C Bb Ab G F");
            descending_test_case("F#", Aeolian, "F# E D C# B A G# F#");

            descending_test_case("Gb", Aeolian, "Gb Fb Ebb Db Cb Bbb Ab Gb");
            descending_test_case("G", Aeolian, "G F Eb D C Bb A G");
            descending_test_case("G#", Aeolian, "G# F# E D# C# B A# G#");
        }

        #[test]
        fn creates_descending_locrian_scale() {
            descending_test_case("Ab", Locrian, "Ab Gb Fb Ebb Db Cb Bbb Ab");
            descending_test_case("A", Locrian, "A G F Eb D C Bb A");
            descending_test_case("A#", Locrian, "A# G# F# E D# C# B A#");

            descending_test_case("Bb", Locrian, "Bb Ab Gb Fb Eb Db Cb Bb");
            descending_test_case("B", Locrian, "B A G F E D C B");
            descending_test_case("B#", Locrian, "B# A# G# F# E# D# C# B#");

            descending_test_case("Cb", Locrian, "Cb Bbb Abb Gbb Fb Ebb Dbb Cb");
            descending_test_case("C", Locrian, "C Bb Ab Gb F Eb Db C");
            descending_test_case("C#", Locrian, "C# B A G F# E D C#");

            descending_test_case("Db", Locrian, "Db Cb Bbb Abb Gb Fb Ebb Db");
            descending_test_case("D", Locrian, "D C Bb Ab G F Eb D");
            descending_test_case("D#", Locrian, "D# C# B A G# F# E D#");

            descending_test_case("Eb", Locrian, "Eb Db Cb Bbb Ab Gb Fb Eb");
            descending_test_case("E", Locrian, "E D C Bb A G F E");
            descending_test_case("E#", Locrian, "E# D# C# B A# G# F# E#");

            descending_test_case("Fb", Locrian, "Fb Ebb Dbb Cbb Bbb Abb Gbb Fb");
            descending_test_case("F", Locrian, "F Eb Db Cb Bb Ab Gb F");
            descending_test_case("F#", Locrian, "F# E D C B A G F#");

            descending_test_case("Gb", Locrian, "Gb Fb Ebb Dbb Cb Bbb Abb Gb");
            descending_test_case("G", Locrian, "G F Eb Db C Bb Ab G");
            descending_test_case("G#", Locrian, "G# F# E D C# B A G#");
        }

        #[test]
        fn creates_descending_major_pentatonic_scale() {
            descending_test_case("Ab", MajorPentatonic, "Ab F Eb C Bb Ab");
            descending_test_case("A", MajorPentatonic, "A F# E C# B A");
            descending_test_case("A#", MajorPentatonic, "A# F## E# C## B# A#");

            descending_test_case("Bb", MajorPentatonic, "Bb G F D C Bb");
            descending_test_case("B", MajorPentatonic, "B G# F# D# C# B");
            descending_test_case("B#", MajorPentatonic, "B# G## F## D## C## B#");

            descending_test_case("Cb", MajorPentatonic, "Cb Ab Gb Eb Db Cb");
            descending_test_case("C", MajorPentatonic, "C A G E D C");
            descending_test_case("C#", MajorPentatonic, "C# A# G# E# D# C#");

            descending_test_case("Db", MajorPentatonic, "Db Bb Ab F Eb Db");
            descending_test_case("D", MajorPentatonic, "D B A F# E D");
            descending_test_case("D#", MajorPentatonic, "D# B# A# F## E# D#");

            descending_test_case("Eb", MajorPentatonic, "Eb C Bb G F Eb");
            descending_test_case("E", MajorPentatonic, "E C# B G# F# E");
            descending_test_case("E#", MajorPentatonic, "E# C## B# G## F## E#");

            descending_test_case("Fb", MajorPentatonic, "Fb Db Cb Ab Gb Fb");
            descending_test_case("F", MajorPentatonic, "F D C A G F");
            descending_test_case("F#", MajorPentatonic, "F# D# C# A# G# F#");

            descending_test_case("Gb", MajorPentatonic, "Gb Eb Db Bb Ab Gb");
            descending_test_case("G", MajorPentatonic, "G E D B A G");
            descending_test_case("G#", MajorPentatonic, "G# E# D# B# A# G#");
        }

        #[test]
        fn creates_descending_minor_pentatonic_scale() {
            descending_test_case("Ab", MinorPentatonic, "Ab Gb Eb Db Cb Ab");
            descending_test_case("A", MinorPentatonic, "A G E D C A");
            descending_test_case("A#", MinorPentatonic, "A# G# E# D# C# A#");

            descending_test_case("Bb", MinorPentatonic, "Bb Ab F Eb Db Bb");
            descending_test_case("B", MinorPentatonic, "B A F# E D B");
            descending_test_case("B#", MinorPentatonic, "B# A# F## E# D# B#");

            descending_test_case("Cb", MinorPentatonic, "Cb Bbb Gb Fb Ebb Cb");
            descending_test_case("C", MinorPentatonic, "C Bb G F Eb C");
            descending_test_case("C#", MinorPentatonic, "C# B G# F# E C#");

            descending_test_case("Db", MinorPentatonic, "Db Cb Ab Gb Fb Db");
            descending_test_case("D", MinorPentatonic, "D C A G F D");
            descending_test_case("D#", MinorPentatonic, "D# C# A# G# F# D#");

            descending_test_case("Eb", MinorPentatonic, "Eb Db Bb Ab Gb Eb");
            descending_test_case("E", MinorPentatonic, "E D B A G E");
            descending_test_case("E#", MinorPentatonic, "E# D# B# A# G# E#");

            descending_test_case("Fb", MinorPentatonic, "Fb Ebb Cb Bbb Abb Fb");
            descending_test_case("F", MinorPentatonic, "F Eb C Bb Ab F");
            descending_test_case("F#", MinorPentatonic, "F# E C# B A F#");

            descending_test_case("Gb", MinorPentatonic, "Gb Fb Db Cb Bbb Gb");
            descending_test_case("G", MinorPentatonic, "G F D C Bb G");
            descending_test_case("G#", MinorPentatonic, "G# F# D# C# B G#");
        }

        #[test]
        fn creates_descending_harmonic_minor_scale() {
            descending_test_case("Ab", HarmonicMinor, "Ab G Fb Eb Db Cb Bb Ab");
            descending_test_case("A", HarmonicMinor, "A G# F E D C B A");
            descending_test_case("A#", HarmonicMinor, "A# G## F# E# D# C# B# A#");

            descending_test_case("Bb", HarmonicMinor, "Bb A Gb F Eb Db C Bb");
            descending_test_case("B", HarmonicMinor, "B A# G F# E D C# B");
            descending_test_case("B#", HarmonicMinor, "B# A## G# F## E# D# C## B#");

            descending_test_case("Cb", HarmonicMinor, "Cb Bb Abb Gb Fb Ebb Db Cb");
            descending_test_case("C", HarmonicMinor, "C B Ab G F Eb D C");
            descending_test_case("C#", HarmonicMinor, "C# B# A G# F# E D# C#");

            descending_test_case("Db", HarmonicMinor, "Db C Bbb Ab Gb Fb Eb Db");
            descending_test_case("D", HarmonicMinor, "D C# Bb A G F E D");
            descending_test_case("D#", HarmonicMinor, "D# C## B A# G# F# E# D#");

            descending_test_case("Eb", HarmonicMinor, "Eb D Cb Bb Ab Gb F Eb");
            descending_test_case("E", HarmonicMinor, "E D# C B A G F# E");
            descending_test_case("E#", HarmonicMinor, "E# D## C# B# A# G# F## E#");

            descending_test_case("Fb", HarmonicMinor, "Fb Eb Dbb Cb Bbb Abb Gb Fb");
            descending_test_case("F", HarmonicMinor, "F E Db C Bb Ab G F");
            descending_test_case("F#", HarmonicMinor, "F# E# D C# B A G# F#");

            descending_test_case("Gb", HarmonicMinor, "Gb F Ebb Db Cb Bbb Ab Gb");
            descending_test_case("G", HarmonicMinor, "G F# Eb D C Bb A G");
            descending_test_case("G#", HarmonicMinor, "G# F## E D# C# B A# G#");
        }

        #[test]
        fn creates_descending_melodic_minor_scale() {
            descending_test_case("Ab", MelodicMinor, "Ab Gb Fb Eb Db Cb Bb Ab");
            descending_test_case("A", MelodicMinor, "A G F E D C B A");
            descending_test_case("A#", MelodicMinor, "A# G# F# E# D# C# B# A#");

            descending_test_case("Bb", MelodicMinor, "Bb Ab Gb F Eb Db C Bb");
            descending_test_case("B", MelodicMinor, "B A G F# E D C# B");
            descending_test_case("B#", MelodicMinor, "B# A# G# F## E# D# C## B#");

            descending_test_case("Cb", MelodicMinor, "Cb Bbb Abb Gb Fb Ebb Db Cb");
            descending_test_case("C", MelodicMinor, "C Bb Ab G F Eb D C");
            descending_test_case("C#", MelodicMinor, "C# B A G# F# E D# C#");

            descending_test_case("Db", MelodicMinor, "Db Cb Bbb Ab Gb Fb Eb Db");
            descending_test_case("D", MelodicMinor, "D C Bb A G F E D");
            descending_test_case("D#", MelodicMinor, "D# C# B A# G# F# E# D#");

            descending_test_case("Eb", MelodicMinor, "Eb Db Cb Bb Ab Gb F Eb");
            descending_test_case("E", MelodicMinor, "E D C B A G F# E");
            descending_test_case("E#", MelodicMinor, "E# D# C# B# A# G# F## E#");

            descending_test_case("Fb", MelodicMinor, "Fb Ebb Dbb Cb Bbb Abb Gb Fb");
            descending_test_case("F", MelodicMinor, "F Eb Db C Bb Ab G F");
            descending_test_case("F#", MelodicMinor, "F# E D C# B A G# F#");

            descending_test_case("Gb", MelodicMinor, "Gb Fb Ebb Db Cb Bbb Ab Gb");
            descending_test_case("G", MelodicMinor, "G F Eb D C Bb A G");
            descending_test_case("G#", MelodicMinor, "G# F# E D# C# B A# G#");
        }
    }

    mod ascending_descending_scales {
        use super::*;

        fn ascending_descending_test_case(root_note_str: &str, kind: ScaleKind, expected: &str) {
            test_case(root_note_str, kind, AscendingDescending, expected);
        }

        #[test]
        fn creates_ascending_descending_major_scale() {
            ascending_descending_test_case("Ab", Major, "Ab Bb C Db Eb F G Ab G F Eb Db C Bb Ab");
            ascending_descending_test_case("A", Major, "A B C# D E F# G# A G# F# E D C# B A");
            ascending_descending_test_case(
                "A#",
                Major,
                "A# B# C## D# E# F## G## A# G## F## E# D# C## B# A#",
            );

            ascending_descending_test_case("Bb", Major, "Bb C D Eb F G A Bb A G F Eb D C Bb");
            ascending_descending_test_case("B", Major, "B C# D# E F# G# A# B A# G# F# E D# C# B");
            ascending_descending_test_case(
                "B#",
                Major,
                "B# C## D## E# F## G## A## B# A## G## F## E# D## C## B#",
            );

            ascending_descending_test_case(
                "Cb",
                Major,
                "Cb Db Eb Fb Gb Ab Bb Cb Bb Ab Gb Fb Eb Db Cb",
            );
            ascending_descending_test_case("C", Major, "C D E F G A B C B A G F E D C");
            ascending_descending_test_case(
                "C#",
                Major,
                "C# D# E# F# G# A# B# C# B# A# G# F# E# D# C#",
            );

            ascending_descending_test_case("Db", Major, "Db Eb F Gb Ab Bb C Db C Bb Ab Gb F Eb Db");
            ascending_descending_test_case("D", Major, "D E F# G A B C# D C# B A G F# E D");
            ascending_descending_test_case(
                "D#",
                Major,
                "D# E# F## G# A# B# C## D# C## B# A# G# F## E# D#",
            );

            ascending_descending_test_case("Eb", Major, "Eb F G Ab Bb C D Eb D C Bb Ab G F Eb");
            ascending_descending_test_case("E", Major, "E F# G# A B C# D# E D# C# B A G# F# E");
            ascending_descending_test_case(
                "E#",
                Major,
                "E# F## G## A# B# C## D## E# D## C## B# A# G## F## E#",
            );

            ascending_descending_test_case(
                "Fb",
                Major,
                "Fb Gb Ab Bbb Cb Db Eb Fb Eb Db Cb Bbb Ab Gb Fb",
            );
            ascending_descending_test_case("F", Major, "F G A Bb C D E F E D C Bb A G F");
            ascending_descending_test_case(
                "F#",
                Major,
                "F# G# A# B C# D# E# F# E# D# C# B A# G# F#",
            );

            ascending_descending_test_case(
                "Gb",
                Major,
                "Gb Ab Bb Cb Db Eb F Gb F Eb Db Cb Bb Ab Gb",
            );
            ascending_descending_test_case("G", Major, "G A B C D E F# G F# E D C B A G");
            ascending_descending_test_case(
                "G#",
                Major,
                "G# A# B# C# D# E# F## G# F## E# D# C# B# A# G#",
            );
        }

        #[test]
        fn creates_ascending_descending_minor_scale() {
            ascending_descending_test_case(
                "Ab",
                Minor,
                "Ab Bb Cb Db Eb Fb Gb Ab Gb Fb Eb Db Cb Bb Ab",
            );
            ascending_descending_test_case("A", Minor, "A B C D E F G A G F E D C B A");
            ascending_descending_test_case(
                "A#",
                Minor,
                "A# B# C# D# E# F# G# A# G# F# E# D# C# B# A#",
            );

            ascending_descending_test_case("Bb", Minor, "Bb C Db Eb F Gb Ab Bb Ab Gb F Eb Db C Bb");
            ascending_descending_test_case("B", Minor, "B C# D E F# G A B A G F# E D C# B");
            ascending_descending_test_case(
                "B#",
                Minor,
                "B# C## D# E# F## G# A# B# A# G# F## E# D# C## B#",
            );

            ascending_descending_test_case(
                "Cb",
                Minor,
                "Cb Db Ebb Fb Gb Abb Bbb Cb Bbb Abb Gb Fb Ebb Db Cb",
            );
            ascending_descending_test_case("C", Minor, "C D Eb F G Ab Bb C Bb Ab G F Eb D C");
            ascending_descending_test_case("C#", Minor, "C# D# E F# G# A B C# B A G# F# E D# C#");

            ascending_descending_test_case(
                "Db",
                Minor,
                "Db Eb Fb Gb Ab Bbb Cb Db Cb Bbb Ab Gb Fb Eb Db",
            );
            ascending_descending_test_case("D", Minor, "D E F G A Bb C D C Bb A G F E D");
            ascending_descending_test_case(
                "D#",
                Minor,
                "D# E# F# G# A# B C# D# C# B A# G# F# E# D#",
            );

            ascending_descending_test_case(
                "Eb",
                Minor,
                "Eb F Gb Ab Bb Cb Db Eb Db Cb Bb Ab Gb F Eb",
            );
            ascending_descending_test_case("E", Minor, "E F# G A B C D E D C B A G F# E");
            ascending_descending_test_case(
                "E#",
                Minor,
                "E# F## G# A# B# C# D# E# D# C# B# A# G# F## E#",
            );

            ascending_descending_test_case(
                "Fb",
                Minor,
                "Fb Gb Abb Bbb Cb Dbb Ebb Fb Ebb Dbb Cb Bbb Abb Gb Fb",
            );
            ascending_descending_test_case("F", Minor, "F G Ab Bb C Db Eb F Eb Db C Bb Ab G F");
            ascending_descending_test_case("F#", Minor, "F# G# A B C# D E F# E D C# B A G# F#");

            ascending_descending_test_case(
                "Gb",
                Minor,
                "Gb Ab Bbb Cb Db Ebb Fb Gb Fb Ebb Db Cb Bbb Ab Gb",
            );
            ascending_descending_test_case("G", Minor, "G A Bb C D Eb F G F Eb D C Bb A G");
            ascending_descending_test_case("G#", Minor, "G# A# B C# D# E F# G# F# E D# C# B A# G#");
        }

        #[test]
        fn creates_ascending_descending_ionian_scale() {
            ascending_descending_test_case("Ab", Ionian, "Ab Bb C Db Eb F G Ab G F Eb Db C Bb Ab");
            ascending_descending_test_case("A", Ionian, "A B C# D E F# G# A G# F# E D C# B A");
            ascending_descending_test_case(
                "A#",
                Ionian,
                "A# B# C## D# E# F## G## A# G## F## E# D# C## B# A#",
            );

            ascending_descending_test_case("Bb", Ionian, "Bb C D Eb F G A Bb A G F Eb D C Bb");
            ascending_descending_test_case("B", Ionian, "B C# D# E F# G# A# B A# G# F# E D# C# B");
            ascending_descending_test_case(
                "B#",
                Ionian,
                "B# C## D## E# F## G## A## B# A## G## F## E# D## C## B#",
            );

            ascending_descending_test_case(
                "Cb",
                Ionian,
                "Cb Db Eb Fb Gb Ab Bb Cb Bb Ab Gb Fb Eb Db Cb",
            );
            ascending_descending_test_case("C", Ionian, "C D E F G A B C B A G F E D C");
            ascending_descending_test_case(
                "C#",
                Ionian,
                "C# D# E# F# G# A# B# C# B# A# G# F# E# D# C#",
            );

            ascending_descending_test_case(
                "Db",
                Ionian,
                "Db Eb F Gb Ab Bb C Db C Bb Ab Gb F Eb Db",
            );
            ascending_descending_test_case("D", Ionian, "D E F# G A B C# D C# B A G F# E D");
            ascending_descending_test_case(
                "D#",
                Ionian,
                "D# E# F## G# A# B# C## D# C## B# A# G# F## E# D#",
            );

            ascending_descending_test_case("Eb", Ionian, "Eb F G Ab Bb C D Eb D C Bb Ab G F Eb");
            ascending_descending_test_case("E", Ionian, "E F# G# A B C# D# E D# C# B A G# F# E");
            ascending_descending_test_case(
                "E#",
                Ionian,
                "E# F## G## A# B# C## D## E# D## C## B# A# G## F## E#",
            );

            ascending_descending_test_case(
                "Fb",
                Ionian,
                "Fb Gb Ab Bbb Cb Db Eb Fb Eb Db Cb Bbb Ab Gb Fb",
            );
            ascending_descending_test_case("F", Ionian, "F G A Bb C D E F E D C Bb A G F");
            ascending_descending_test_case(
                "F#",
                Ionian,
                "F# G# A# B C# D# E# F# E# D# C# B A# G# F#",
            );

            ascending_descending_test_case(
                "Gb",
                Ionian,
                "Gb Ab Bb Cb Db Eb F Gb F Eb Db Cb Bb Ab Gb",
            );
            ascending_descending_test_case("G", Ionian, "G A B C D E F# G F# E D C B A G");
            ascending_descending_test_case(
                "G#",
                Ionian,
                "G# A# B# C# D# E# F## G# F## E# D# C# B# A# G#",
            );
        }

        #[test]
        fn creates_ascending_descending_dorian_scale() {
            ascending_descending_test_case(
                "Ab",
                Dorian,
                "Ab Bb Cb Db Eb F Gb Ab Gb F Eb Db Cb Bb Ab",
            );
            ascending_descending_test_case("A", Dorian, "A B C D E F# G A G F# E D C B A");
            ascending_descending_test_case(
                "A#",
                Dorian,
                "A# B# C# D# E# F## G# A# G# F## E# D# C# B# A#",
            );

            ascending_descending_test_case("Bb", Dorian, "Bb C Db Eb F G Ab Bb Ab G F Eb Db C Bb");
            ascending_descending_test_case("B", Dorian, "B C# D E F# G# A B A G# F# E D C# B");
            ascending_descending_test_case(
                "B#",
                Dorian,
                "B# C## D# E# F## G## A# B# A# G## F## E# D# C## B#",
            );

            ascending_descending_test_case(
                "Cb",
                Dorian,
                "Cb Db Ebb Fb Gb Ab Bbb Cb Bbb Ab Gb Fb Ebb Db Cb",
            );
            ascending_descending_test_case("C", Dorian, "C D Eb F G A Bb C Bb A G F Eb D C");
            ascending_descending_test_case(
                "C#",
                Dorian,
                "C# D# E F# G# A# B C# B A# G# F# E D# C#",
            );

            ascending_descending_test_case(
                "Db",
                Dorian,
                "Db Eb Fb Gb Ab Bb Cb Db Cb Bb Ab Gb Fb Eb Db",
            );
            ascending_descending_test_case("D", Dorian, "D E F G A B C D C B A G F E D");
            ascending_descending_test_case(
                "D#",
                Dorian,
                "D# E# F# G# A# B# C# D# C# B# A# G# F# E# D#",
            );

            ascending_descending_test_case(
                "Eb",
                Dorian,
                "Eb F Gb Ab Bb C Db Eb Db C Bb Ab Gb F Eb",
            );
            ascending_descending_test_case("E", Dorian, "E F# G A B C# D E D C# B A G F# E");
            ascending_descending_test_case(
                "E#",
                Dorian,
                "E# F## G# A# B# C## D# E# D# C## B# A# G# F## E#",
            );

            ascending_descending_test_case(
                "Fb",
                Dorian,
                "Fb Gb Abb Bbb Cb Db Ebb Fb Ebb Db Cb Bbb Abb Gb Fb",
            );
            ascending_descending_test_case("F", Dorian, "F G Ab Bb C D Eb F Eb D C Bb Ab G F");
            ascending_descending_test_case("F#", Dorian, "F# G# A B C# D# E F# E D# C# B A G# F#");

            ascending_descending_test_case(
                "Gb",
                Dorian,
                "Gb Ab Bbb Cb Db Eb Fb Gb Fb Eb Db Cb Bbb Ab Gb",
            );
            ascending_descending_test_case("G", Dorian, "G A Bb C D E F G F E D C Bb A G");
            ascending_descending_test_case(
                "G#",
                Dorian,
                "G# A# B C# D# E# F# G# F# E# D# C# B A# G#",
            );
        }

        #[test]
        fn creates_ascending_descending_phrygian_scale() {
            ascending_descending_test_case(
                "Ab",
                Phrygian,
                "Ab Bbb Cb Db Eb Fb Gb Ab Gb Fb Eb Db Cb Bbb Ab",
            );
            ascending_descending_test_case("A", Phrygian, "A Bb C D E F G A G F E D C Bb A");
            ascending_descending_test_case(
                "A#",
                Phrygian,
                "A# B C# D# E# F# G# A# G# F# E# D# C# B A#",
            );

            ascending_descending_test_case(
                "Bb",
                Phrygian,
                "Bb Cb Db Eb F Gb Ab Bb Ab Gb F Eb Db Cb Bb",
            );
            ascending_descending_test_case("B", Phrygian, "B C D E F# G A B A G F# E D C B");
            ascending_descending_test_case(
                "B#",
                Phrygian,
                "B# C# D# E# F## G# A# B# A# G# F## E# D# C# B#",
            );

            ascending_descending_test_case(
                "Cb",
                Phrygian,
                "Cb Dbb Ebb Fb Gb Abb Bbb Cb Bbb Abb Gb Fb Ebb Dbb Cb",
            );
            ascending_descending_test_case("C", Phrygian, "C Db Eb F G Ab Bb C Bb Ab G F Eb Db C");
            ascending_descending_test_case("C#", Phrygian, "C# D E F# G# A B C# B A G# F# E D C#");

            ascending_descending_test_case(
                "Db",
                Phrygian,
                "Db Ebb Fb Gb Ab Bbb Cb Db Cb Bbb Ab Gb Fb Ebb Db",
            );
            ascending_descending_test_case("D", Phrygian, "D Eb F G A Bb C D C Bb A G F Eb D");
            ascending_descending_test_case(
                "D#",
                Phrygian,
                "D# E F# G# A# B C# D# C# B A# G# F# E D#",
            );

            ascending_descending_test_case(
                "Eb",
                Phrygian,
                "Eb Fb Gb Ab Bb Cb Db Eb Db Cb Bb Ab Gb Fb Eb",
            );
            ascending_descending_test_case("E", Phrygian, "E F G A B C D E D C B A G F E");
            ascending_descending_test_case(
                "E#",
                Phrygian,
                "E# F# G# A# B# C# D# E# D# C# B# A# G# F# E#",
            );

            ascending_descending_test_case(
                "Fb",
                Phrygian,
                "Fb Gbb Abb Bbb Cb Dbb Ebb Fb Ebb Dbb Cb Bbb Abb Gbb Fb",
            );
            ascending_descending_test_case(
                "F",
                Phrygian,
                "F Gb Ab Bb C Db Eb F Eb Db C Bb Ab Gb F",
            );
            ascending_descending_test_case("F#", Phrygian, "F# G A B C# D E F# E D C# B A G F#");

            ascending_descending_test_case(
                "Gb",
                Phrygian,
                "Gb Abb Bbb Cb Db Ebb Fb Gb Fb Ebb Db Cb Bbb Abb Gb",
            );
            ascending_descending_test_case("G", Phrygian, "G Ab Bb C D Eb F G F Eb D C Bb Ab G");
            ascending_descending_test_case(
                "G#",
                Phrygian,
                "G# A B C# D# E F# G# F# E D# C# B A G#",
            );
        }

        #[test]
        fn creates_ascending_descending_lydian_scale() {
            ascending_descending_test_case("Ab", Lydian, "Ab Bb C D Eb F G Ab G F Eb D C Bb Ab");
            ascending_descending_test_case("A", Lydian, "A B C# D# E F# G# A G# F# E D# C# B A");
            ascending_descending_test_case(
                "A#",
                Lydian,
                "A# B# C## D## E# F## G## A# G## F## E# D## C## B# A#",
            );

            ascending_descending_test_case("Bb", Lydian, "Bb C D E F G A Bb A G F E D C Bb");
            ascending_descending_test_case(
                "B",
                Lydian,
                "B C# D# E# F# G# A# B A# G# F# E# D# C# B",
            );
            ascending_descending_test_case(
                "B#",
                Lydian,
                "B# C## D## E## F## G## A## B# A## G## F## E## D## C## B#",
            );

            ascending_descending_test_case(
                "Cb",
                Lydian,
                "Cb Db Eb F Gb Ab Bb Cb Bb Ab Gb F Eb Db Cb",
            );
            ascending_descending_test_case("C", Lydian, "C D E F# G A B C B A G F# E D C");
            ascending_descending_test_case(
                "C#",
                Lydian,
                "C# D# E# F## G# A# B# C# B# A# G# F## E# D# C#",
            );

            ascending_descending_test_case("Db", Lydian, "Db Eb F G Ab Bb C Db C Bb Ab G F Eb Db");
            ascending_descending_test_case("D", Lydian, "D E F# G# A B C# D C# B A G# F# E D");
            ascending_descending_test_case(
                "D#",
                Lydian,
                "D# E# F## G## A# B# C## D# C## B# A# G## F## E# D#",
            );

            ascending_descending_test_case("Eb", Lydian, "Eb F G A Bb C D Eb D C Bb A G F Eb");
            ascending_descending_test_case("E", Lydian, "E F# G# A# B C# D# E D# C# B A# G# F# E");
            ascending_descending_test_case(
                "E#",
                Lydian,
                "E# F## G## A## B# C## D## E# D## C## B# A## G## F## E#",
            );

            ascending_descending_test_case(
                "Fb",
                Lydian,
                "Fb Gb Ab Bb Cb Db Eb Fb Eb Db Cb Bb Ab Gb Fb",
            );
            ascending_descending_test_case("F", Lydian, "F G A B C D E F E D C B A G F");
            ascending_descending_test_case(
                "F#",
                Lydian,
                "F# G# A# B# C# D# E# F# E# D# C# B# A# G# F#",
            );

            ascending_descending_test_case(
                "Gb",
                Lydian,
                "Gb Ab Bb C Db Eb F Gb F Eb Db C Bb Ab Gb",
            );
            ascending_descending_test_case("G", Lydian, "G A B C# D E F# G F# E D C# B A G");
            ascending_descending_test_case(
                "G#",
                Lydian,
                "G# A# B# C## D# E# F## G# F## E# D# C## B# A# G#",
            );
        }

        #[test]
        fn creates_ascending_descending_mixolydian_scale() {
            ascending_descending_test_case(
                "Ab",
                Mixolydian,
                "Ab Bb C Db Eb F Gb Ab Gb F Eb Db C Bb Ab",
            );
            ascending_descending_test_case("A", Mixolydian, "A B C# D E F# G A G F# E D C# B A");
            ascending_descending_test_case(
                "A#",
                Mixolydian,
                "A# B# C## D# E# F## G# A# G# F## E# D# C## B# A#",
            );

            ascending_descending_test_case(
                "Bb",
                Mixolydian,
                "Bb C D Eb F G Ab Bb Ab G F Eb D C Bb",
            );
            ascending_descending_test_case(
                "B",
                Mixolydian,
                "B C# D# E F# G# A B A G# F# E D# C# B",
            );
            ascending_descending_test_case(
                "B#",
                Mixolydian,
                "B# C## D## E# F## G## A# B# A# G## F## E# D## C## B#",
            );

            ascending_descending_test_case(
                "Cb",
                Mixolydian,
                "Cb Db Eb Fb Gb Ab Bbb Cb Bbb Ab Gb Fb Eb Db Cb",
            );
            ascending_descending_test_case("C", Mixolydian, "C D E F G A Bb C Bb A G F E D C");
            ascending_descending_test_case(
                "C#",
                Mixolydian,
                "C# D# E# F# G# A# B C# B A# G# F# E# D# C#",
            );

            ascending_descending_test_case(
                "Db",
                Mixolydian,
                "Db Eb F Gb Ab Bb Cb Db Cb Bb Ab Gb F Eb Db",
            );
            ascending_descending_test_case("D", Mixolydian, "D E F# G A B C D C B A G F# E D");
            ascending_descending_test_case(
                "D#",
                Mixolydian,
                "D# E# F## G# A# B# C# D# C# B# A# G# F## E# D#",
            );

            ascending_descending_test_case(
                "Eb",
                Mixolydian,
                "Eb F G Ab Bb C Db Eb Db C Bb Ab G F Eb",
            );
            ascending_descending_test_case("E", Mixolydian, "E F# G# A B C# D E D C# B A G# F# E");
            ascending_descending_test_case(
                "E#",
                Mixolydian,
                "E# F## G## A# B# C## D# E# D# C## B# A# G## F## E#",
            );

            ascending_descending_test_case(
                "Fb",
                Mixolydian,
                "Fb Gb Ab Bbb Cb Db Ebb Fb Ebb Db Cb Bbb Ab Gb Fb",
            );
            ascending_descending_test_case("F", Mixolydian, "F G A Bb C D Eb F Eb D C Bb A G F");
            ascending_descending_test_case(
                "F#",
                Mixolydian,
                "F# G# A# B C# D# E F# E D# C# B A# G# F#",
            );

            ascending_descending_test_case(
                "Gb",
                Mixolydian,
                "Gb Ab Bb Cb Db Eb Fb Gb Fb Eb Db Cb Bb Ab Gb",
            );
            ascending_descending_test_case("G", Mixolydian, "G A B C D E F G F E D C B A G");
            ascending_descending_test_case(
                "G#",
                Mixolydian,
                "G# A# B# C# D# E# F# G# F# E# D# C# B# A# G#",
            );
        }

        #[test]
        fn creates_ascending_descending_aeolian_scale() {
            ascending_descending_test_case(
                "Ab",
                Aeolian,
                "Ab Bb Cb Db Eb Fb Gb Ab Gb Fb Eb Db Cb Bb Ab",
            );
            ascending_descending_test_case("A", Aeolian, "A B C D E F G A G F E D C B A");
            ascending_descending_test_case(
                "A#",
                Aeolian,
                "A# B# C# D# E# F# G# A# G# F# E# D# C# B# A#",
            );

            ascending_descending_test_case(
                "Bb",
                Aeolian,
                "Bb C Db Eb F Gb Ab Bb Ab Gb F Eb Db C Bb",
            );
            ascending_descending_test_case("B", Aeolian, "B C# D E F# G A B A G F# E D C# B");
            ascending_descending_test_case(
                "B#",
                Aeolian,
                "B# C## D# E# F## G# A# B# A# G# F## E# D# C## B#",
            );

            ascending_descending_test_case(
                "Cb",
                Aeolian,
                "Cb Db Ebb Fb Gb Abb Bbb Cb Bbb Abb Gb Fb Ebb Db Cb",
            );
            ascending_descending_test_case("C", Aeolian, "C D Eb F G Ab Bb C Bb Ab G F Eb D C");
            ascending_descending_test_case("C#", Aeolian, "C# D# E F# G# A B C# B A G# F# E D# C#");

            ascending_descending_test_case(
                "Db",
                Aeolian,
                "Db Eb Fb Gb Ab Bbb Cb Db Cb Bbb Ab Gb Fb Eb Db",
            );
            ascending_descending_test_case("D", Aeolian, "D E F G A Bb C D C Bb A G F E D");
            ascending_descending_test_case(
                "D#",
                Aeolian,
                "D# E# F# G# A# B C# D# C# B A# G# F# E# D#",
            );

            ascending_descending_test_case(
                "Eb",
                Aeolian,
                "Eb F Gb Ab Bb Cb Db Eb Db Cb Bb Ab Gb F Eb",
            );
            ascending_descending_test_case("E", Aeolian, "E F# G A B C D E D C B A G F# E");
            ascending_descending_test_case(
                "E#",
                Aeolian,
                "E# F## G# A# B# C# D# E# D# C# B# A# G# F## E#",
            );

            ascending_descending_test_case(
                "Fb",
                Aeolian,
                "Fb Gb Abb Bbb Cb Dbb Ebb Fb Ebb Dbb Cb Bbb Abb Gb Fb",
            );
            ascending_descending_test_case("F", Aeolian, "F G Ab Bb C Db Eb F Eb Db C Bb Ab G F");
            ascending_descending_test_case("F#", Aeolian, "F# G# A B C# D E F# E D C# B A G# F#");

            ascending_descending_test_case(
                "Gb",
                Aeolian,
                "Gb Ab Bbb Cb Db Ebb Fb Gb Fb Ebb Db Cb Bbb Ab Gb",
            );
            ascending_descending_test_case("G", Aeolian, "G A Bb C D Eb F G F Eb D C Bb A G");
            ascending_descending_test_case(
                "G#",
                Aeolian,
                "G# A# B C# D# E F# G# F# E D# C# B A# G#",
            );
        }

        #[test]
        fn creates_ascending_descending_locrian_scale() {
            ascending_descending_test_case(
                "Ab",
                Locrian,
                "Ab Bbb Cb Db Ebb Fb Gb Ab Gb Fb Ebb Db Cb Bbb Ab",
            );
            ascending_descending_test_case("A", Locrian, "A Bb C D Eb F G A G F Eb D C Bb A");
            ascending_descending_test_case(
                "A#",
                Locrian,
                "A# B C# D# E F# G# A# G# F# E D# C# B A#",
            );

            ascending_descending_test_case(
                "Bb",
                Locrian,
                "Bb Cb Db Eb Fb Gb Ab Bb Ab Gb Fb Eb Db Cb Bb",
            );
            ascending_descending_test_case("B", Locrian, "B C D E F G A B A G F E D C B");
            ascending_descending_test_case(
                "B#",
                Locrian,
                "B# C# D# E# F# G# A# B# A# G# F# E# D# C# B#",
            );

            ascending_descending_test_case(
                "Cb",
                Locrian,
                "Cb Dbb Ebb Fb Gbb Abb Bbb Cb Bbb Abb Gbb Fb Ebb Dbb Cb",
            );
            ascending_descending_test_case("C", Locrian, "C Db Eb F Gb Ab Bb C Bb Ab Gb F Eb Db C");
            ascending_descending_test_case("C#", Locrian, "C# D E F# G A B C# B A G F# E D C#");

            ascending_descending_test_case(
                "Db",
                Locrian,
                "Db Ebb Fb Gb Abb Bbb Cb Db Cb Bbb Abb Gb Fb Ebb Db",
            );
            ascending_descending_test_case("D", Locrian, "D Eb F G Ab Bb C D C Bb Ab G F Eb D");
            ascending_descending_test_case("D#", Locrian, "D# E F# G# A B C# D# C# B A G# F# E D#");

            ascending_descending_test_case(
                "Eb",
                Locrian,
                "Eb Fb Gb Ab Bbb Cb Db Eb Db Cb Bbb Ab Gb Fb Eb",
            );
            ascending_descending_test_case("E", Locrian, "E F G A Bb C D E D C Bb A G F E");
            ascending_descending_test_case(
                "E#",
                Locrian,
                "E# F# G# A# B C# D# E# D# C# B A# G# F# E#",
            );

            ascending_descending_test_case(
                "Fb",
                Locrian,
                "Fb Gbb Abb Bbb Cbb Dbb Ebb Fb Ebb Dbb Cbb Bbb Abb Gbb Fb",
            );
            ascending_descending_test_case(
                "F",
                Locrian,
                "F Gb Ab Bb Cb Db Eb F Eb Db Cb Bb Ab Gb F",
            );
            ascending_descending_test_case("F#", Locrian, "F# G A B C D E F# E D C B A G F#");

            ascending_descending_test_case(
                "Gb",
                Locrian,
                "Gb Abb Bbb Cb Dbb Ebb Fb Gb Fb Ebb Dbb Cb Bbb Abb Gb",
            );
            ascending_descending_test_case("G", Locrian, "G Ab Bb C Db Eb F G F Eb Db C Bb Ab G");
            ascending_descending_test_case("G#", Locrian, "G# A B C# D E F# G# F# E D C# B A G#");
        }

        #[test]
        fn creates_ascending_descending_major_pentatonic_scale() {
            ascending_descending_test_case("Ab", MajorPentatonic, "Ab Bb C Eb F Ab F Eb C Bb Ab");
            ascending_descending_test_case("A", MajorPentatonic, "A B C# E F# A F# E C# B A");
            ascending_descending_test_case(
                "A#",
                MajorPentatonic,
                "A# B# C## E# F## A# F## E# C## B# A#",
            );

            ascending_descending_test_case("Bb", MajorPentatonic, "Bb C D F G Bb G F D C Bb");
            ascending_descending_test_case("B", MajorPentatonic, "B C# D# F# G# B G# F# D# C# B");
            ascending_descending_test_case(
                "B#",
                MajorPentatonic,
                "B# C## D## F## G## B# G## F## D## C## B#",
            );

            ascending_descending_test_case(
                "Cb",
                MajorPentatonic,
                "Cb Db Eb Gb Ab Cb Ab Gb Eb Db Cb",
            );
            ascending_descending_test_case("C", MajorPentatonic, "C D E G A C A G E D C");
            ascending_descending_test_case(
                "C#",
                MajorPentatonic,
                "C# D# E# G# A# C# A# G# E# D# C#",
            );

            ascending_descending_test_case("Db", MajorPentatonic, "Db Eb F Ab Bb Db Bb Ab F Eb Db");
            ascending_descending_test_case("D", MajorPentatonic, "D E F# A B D B A F# E D");
            ascending_descending_test_case(
                "D#",
                MajorPentatonic,
                "D# E# F## A# B# D# B# A# F## E# D#",
            );

            ascending_descending_test_case("Eb", MajorPentatonic, "Eb F G Bb C Eb C Bb G F Eb");
            ascending_descending_test_case("E", MajorPentatonic, "E F# G# B C# E C# B G# F# E");
            ascending_descending_test_case(
                "E#",
                MajorPentatonic,
                "E# F## G## B# C## E# C## B# G## F## E#",
            );

            ascending_descending_test_case(
                "Fb",
                MajorPentatonic,
                "Fb Gb Ab Cb Db Fb Db Cb Ab Gb Fb",
            );
            ascending_descending_test_case("F", MajorPentatonic, "F G A C D F D C A G F");
            ascending_descending_test_case(
                "F#",
                MajorPentatonic,
                "F# G# A# C# D# F# D# C# A# G# F#",
            );

            ascending_descending_test_case(
                "Gb",
                MajorPentatonic,
                "Gb Ab Bb Db Eb Gb Eb Db Bb Ab Gb",
            );
            ascending_descending_test_case("G", MajorPentatonic, "G A B D E G E D B A G");
            ascending_descending_test_case(
                "G#",
                MajorPentatonic,
                "G# A# B# D# E# G# E# D# B# A# G#",
            );
        }

        #[test]
        fn creates_ascending_descending_minor_pentatonic_scale() {
            ascending_descending_test_case(
                "Ab",
                MinorPentatonic,
                "Ab Cb Db Eb Gb Ab Gb Eb Db Cb Ab",
            );
            ascending_descending_test_case("A", MinorPentatonic, "A C D E G A G E D C A");
            ascending_descending_test_case(
                "A#",
                MinorPentatonic,
                "A# C# D# E# G# A# G# E# D# C# A#",
            );

            ascending_descending_test_case("Bb", MinorPentatonic, "Bb Db Eb F Ab Bb Ab F Eb Db Bb");
            ascending_descending_test_case("B", MinorPentatonic, "B D E F# A B A F# E D B");
            ascending_descending_test_case(
                "B#",
                MinorPentatonic,
                "B# D# E# F## A# B# A# F## E# D# B#",
            );

            ascending_descending_test_case(
                "Cb",
                MinorPentatonic,
                "Cb Ebb Fb Gb Bbb Cb Bbb Gb Fb Ebb Cb",
            );
            ascending_descending_test_case("C", MinorPentatonic, "C Eb F G Bb C Bb G F Eb C");
            ascending_descending_test_case("C#", MinorPentatonic, "C# E F# G# B C# B G# F# E C#");

            ascending_descending_test_case(
                "Db",
                MinorPentatonic,
                "Db Fb Gb Ab Cb Db Cb Ab Gb Fb Db",
            );
            ascending_descending_test_case("D", MinorPentatonic, "D F G A C D C A G F D");
            ascending_descending_test_case(
                "D#",
                MinorPentatonic,
                "D# F# G# A# C# D# C# A# G# F# D#",
            );

            ascending_descending_test_case(
                "Eb",
                MinorPentatonic,
                "Eb Gb Ab Bb Db Eb Db Bb Ab Gb Eb",
            );
            ascending_descending_test_case("E", MinorPentatonic, "E G A B D E D B A G E");
            ascending_descending_test_case(
                "E#",
                MinorPentatonic,
                "E# G# A# B# D# E# D# B# A# G# E#",
            );

            ascending_descending_test_case(
                "Fb",
                MinorPentatonic,
                "Fb Abb Bbb Cb Ebb Fb Ebb Cb Bbb Abb Fb",
            );
            ascending_descending_test_case("F", MinorPentatonic, "F Ab Bb C Eb F Eb C Bb Ab F");
            ascending_descending_test_case("F#", MinorPentatonic, "F# A B C# E F# E C# B A F#");

            ascending_descending_test_case(
                "Gb",
                MinorPentatonic,
                "Gb Bbb Cb Db Fb Gb Fb Db Cb Bbb Gb",
            );
            ascending_descending_test_case("G", MinorPentatonic, "G Bb C D F G F D C Bb G");
            ascending_descending_test_case("G#", MinorPentatonic, "G# B C# D# F# G# F# D# C# B G#");
        }

        #[test]
        fn creates_ascending_descending_harmonic_minor_scale() {
            ascending_descending_test_case(
                "Ab",
                HarmonicMinor,
                "Ab Bb Cb Db Eb Fb G Ab G Fb Eb Db Cb Bb Ab",
            );
            ascending_descending_test_case("A", HarmonicMinor, "A B C D E F G# A G# F E D C B A");
            ascending_descending_test_case(
                "A#",
                HarmonicMinor,
                "A# B# C# D# E# F# G## A# G## F# E# D# C# B# A#",
            );

            ascending_descending_test_case(
                "Bb",
                HarmonicMinor,
                "Bb C Db Eb F Gb A Bb A Gb F Eb Db C Bb",
            );
            ascending_descending_test_case(
                "B",
                HarmonicMinor,
                "B C# D E F# G A# B A# G F# E D C# B",
            );
            ascending_descending_test_case(
                "B#",
                HarmonicMinor,
                "B# C## D# E# F## G# A## B# A## G# F## E# D# C## B#",
            );

            ascending_descending_test_case(
                "Cb",
                HarmonicMinor,
                "Cb Db Ebb Fb Gb Abb Bb Cb Bb Abb Gb Fb Ebb Db Cb",
            );
            ascending_descending_test_case("C", HarmonicMinor, "C D Eb F G Ab B C B Ab G F Eb D C");
            ascending_descending_test_case(
                "C#",
                HarmonicMinor,
                "C# D# E F# G# A B# C# B# A G# F# E D# C#",
            );

            ascending_descending_test_case(
                "Db",
                HarmonicMinor,
                "Db Eb Fb Gb Ab Bbb C Db C Bbb Ab Gb Fb Eb Db",
            );
            ascending_descending_test_case("D", HarmonicMinor, "D E F G A Bb C# D C# Bb A G F E D");
            ascending_descending_test_case(
                "D#",
                HarmonicMinor,
                "D# E# F# G# A# B C## D# C## B A# G# F# E# D#",
            );

            ascending_descending_test_case(
                "Eb",
                HarmonicMinor,
                "Eb F Gb Ab Bb Cb D Eb D Cb Bb Ab Gb F Eb",
            );
            ascending_descending_test_case("E", HarmonicMinor, "E F# G A B C D# E D# C B A G F# E");
            ascending_descending_test_case(
                "E#",
                HarmonicMinor,
                "E# F## G# A# B# C# D## E# D## C# B# A# G# F## E#",
            );

            ascending_descending_test_case(
                "Fb",
                HarmonicMinor,
                "Fb Gb Abb Bbb Cb Dbb Eb Fb Eb Dbb Cb Bbb Abb Gb Fb",
            );
            ascending_descending_test_case(
                "F",
                HarmonicMinor,
                "F G Ab Bb C Db E F E Db C Bb Ab G F",
            );
            ascending_descending_test_case(
                "F#",
                HarmonicMinor,
                "F# G# A B C# D E# F# E# D C# B A G# F#",
            );

            ascending_descending_test_case(
                "Gb",
                HarmonicMinor,
                "Gb Ab Bbb Cb Db Ebb F Gb F Ebb Db Cb Bbb Ab Gb",
            );
            ascending_descending_test_case(
                "G",
                HarmonicMinor,
                "G A Bb C D Eb F# G F# Eb D C Bb A G",
            );
            ascending_descending_test_case(
                "G#",
                HarmonicMinor,
                "G# A# B C# D# E F## G# F## E D# C# B A# G#",
            );
        }

        #[test]
        fn creates_ascending_descending_melodic_minor_scale() {
            ascending_descending_test_case(
                "Ab",
                MelodicMinor,
                "Ab Bb Cb Db Eb F G Ab Gb Fb Eb Db Cb Bb Ab",
            );
            ascending_descending_test_case("A", MelodicMinor, "A B C D E F# G# A G F E D C B A");
            ascending_descending_test_case(
                "A#",
                MelodicMinor,
                "A# B# C# D# E# F## G## A# G# F# E# D# C# B# A#",
            );

            ascending_descending_test_case(
                "Bb",
                MelodicMinor,
                "Bb C Db Eb F G A Bb Ab Gb F Eb Db C Bb",
            );
            ascending_descending_test_case(
                "B",
                MelodicMinor,
                "B C# D E F# G# A# B A G F# E D C# B",
            );
            ascending_descending_test_case(
                "B#",
                MelodicMinor,
                "B# C## D# E# F## G## A## B# A# G# F## E# D# C## B#",
            );

            ascending_descending_test_case(
                "Cb",
                MelodicMinor,
                "Cb Db Ebb Fb Gb Ab Bb Cb Bbb Abb Gb Fb Ebb Db Cb",
            );
            ascending_descending_test_case("C", MelodicMinor, "C D Eb F G A B C Bb Ab G F Eb D C");
            ascending_descending_test_case(
                "C#",
                MelodicMinor,
                "C# D# E F# G# A# B# C# B A G# F# E D# C#",
            );

            ascending_descending_test_case(
                "Db",
                MelodicMinor,
                "Db Eb Fb Gb Ab Bb C Db Cb Bbb Ab Gb Fb Eb Db",
            );
            ascending_descending_test_case("D", MelodicMinor, "D E F G A B C# D C Bb A G F E D");
            ascending_descending_test_case(
                "D#",
                MelodicMinor,
                "D# E# F# G# A# B# C## D# C# B A# G# F# E# D#",
            );

            ascending_descending_test_case(
                "Eb",
                MelodicMinor,
                "Eb F Gb Ab Bb C D Eb Db Cb Bb Ab Gb F Eb",
            );
            ascending_descending_test_case("E", MelodicMinor, "E F# G A B C# D# E D C B A G F# E");
            ascending_descending_test_case(
                "E#",
                MelodicMinor,
                "E# F## G# A# B# C## D## E# D# C# B# A# G# F## E#",
            );

            ascending_descending_test_case(
                "Fb",
                MelodicMinor,
                "Fb Gb Abb Bbb Cb Db Eb Fb Ebb Dbb Cb Bbb Abb Gb Fb",
            );
            ascending_descending_test_case(
                "F",
                MelodicMinor,
                "F G Ab Bb C D E F Eb Db C Bb Ab G F",
            );
            ascending_descending_test_case(
                "F#",
                MelodicMinor,
                "F# G# A B C# D# E# F# E D C# B A G# F#",
            );

            ascending_descending_test_case(
                "Gb",
                MelodicMinor,
                "Gb Ab Bbb Cb Db Eb F Gb Fb Ebb Db Cb Bbb Ab Gb",
            );
            ascending_descending_test_case("G", MelodicMinor, "G A Bb C D E F# G F Eb D C Bb A G");
            ascending_descending_test_case(
                "G#",
                MelodicMinor,
                "G# A# B C# D# E# F## G# F# E D# C# B A# G#",
            );
        }
    }

    mod descending_ascending_scales {
        use super::*;

        fn descending_ascending_test_case(root_note_str: &str, kind: ScaleKind, expected: &str) {
            test_case(root_note_str, kind, DescendingAscending, expected);
        }

        #[test]
        fn creates_descending_ascending_major_scale() {
            descending_ascending_test_case("Ab", Major, "Ab G F Eb Db C Bb Ab Bb C Db Eb F G Ab");
            descending_ascending_test_case("A", Major, "A G# F# E D C# B A B C# D E F# G# A");
            descending_ascending_test_case(
                "A#",
                Major,
                "A# G## F## E# D# C## B# A# B# C## D# E# F## G## A#",
            );

            descending_ascending_test_case("Bb", Major, "Bb A G F Eb D C Bb C D Eb F G A Bb");
            descending_ascending_test_case("B", Major, "B A# G# F# E D# C# B C# D# E F# G# A# B");
            descending_ascending_test_case(
                "B#",
                Major,
                "B# A## G## F## E# D## C## B# C## D## E# F## G## A## B#",
            );

            descending_ascending_test_case(
                "Cb",
                Major,
                "Cb Bb Ab Gb Fb Eb Db Cb Db Eb Fb Gb Ab Bb Cb",
            );
            descending_ascending_test_case("C", Major, "C B A G F E D C D E F G A B C");
            descending_ascending_test_case(
                "C#",
                Major,
                "C# B# A# G# F# E# D# C# D# E# F# G# A# B# C#",
            );

            descending_ascending_test_case("Db", Major, "Db C Bb Ab Gb F Eb Db Eb F Gb Ab Bb C Db");
            descending_ascending_test_case("D", Major, "D C# B A G F# E D E F# G A B C# D");
            descending_ascending_test_case(
                "D#",
                Major,
                "D# C## B# A# G# F## E# D# E# F## G# A# B# C## D#",
            );

            descending_ascending_test_case("Eb", Major, "Eb D C Bb Ab G F Eb F G Ab Bb C D Eb");
            descending_ascending_test_case("E", Major, "E D# C# B A G# F# E F# G# A B C# D# E");
            descending_ascending_test_case(
                "E#",
                Major,
                "E# D## C## B# A# G## F## E# F## G## A# B# C## D## E#",
            );

            descending_ascending_test_case(
                "Fb",
                Major,
                "Fb Eb Db Cb Bbb Ab Gb Fb Gb Ab Bbb Cb Db Eb Fb",
            );
            descending_ascending_test_case("F", Major, "F E D C Bb A G F G A Bb C D E F");
            descending_ascending_test_case(
                "F#",
                Major,
                "F# E# D# C# B A# G# F# G# A# B C# D# E# F#",
            );

            descending_ascending_test_case(
                "Gb",
                Major,
                "Gb F Eb Db Cb Bb Ab Gb Ab Bb Cb Db Eb F Gb",
            );
            descending_ascending_test_case("G", Major, "G F# E D C B A G A B C D E F# G");
            descending_ascending_test_case(
                "G#",
                Major,
                "G# F## E# D# C# B# A# G# A# B# C# D# E# F## G#",
            );
        }

        #[test]
        fn creates_descending_ascending_minor_scale() {
            descending_ascending_test_case(
                "Ab",
                Minor,
                "Ab Gb Fb Eb Db Cb Bb Ab Bb Cb Db Eb Fb Gb Ab",
            );
            descending_ascending_test_case("A", Minor, "A G F E D C B A B C D E F G A");
            descending_ascending_test_case(
                "A#",
                Minor,
                "A# G# F# E# D# C# B# A# B# C# D# E# F# G# A#",
            );

            descending_ascending_test_case("Bb", Minor, "Bb Ab Gb F Eb Db C Bb C Db Eb F Gb Ab Bb");
            descending_ascending_test_case("B", Minor, "B A G F# E D C# B C# D E F# G A B");
            descending_ascending_test_case(
                "B#",
                Minor,
                "B# A# G# F## E# D# C## B# C## D# E# F## G# A# B#",
            );

            descending_ascending_test_case(
                "Cb",
                Minor,
                "Cb Bbb Abb Gb Fb Ebb Db Cb Db Ebb Fb Gb Abb Bbb Cb",
            );
            descending_ascending_test_case("C", Minor, "C Bb Ab G F Eb D C D Eb F G Ab Bb C");
            descending_ascending_test_case("C#", Minor, "C# B A G# F# E D# C# D# E F# G# A B C#");

            descending_ascending_test_case(
                "Db",
                Minor,
                "Db Cb Bbb Ab Gb Fb Eb Db Eb Fb Gb Ab Bbb Cb Db",
            );
            descending_ascending_test_case("D", Minor, "D C Bb A G F E D E F G A Bb C D");
            descending_ascending_test_case(
                "D#",
                Minor,
                "D# C# B A# G# F# E# D# E# F# G# A# B C# D#",
            );

            descending_ascending_test_case(
                "Eb",
                Minor,
                "Eb Db Cb Bb Ab Gb F Eb F Gb Ab Bb Cb Db Eb",
            );
            descending_ascending_test_case("E", Minor, "E D C B A G F# E F# G A B C D E");
            descending_ascending_test_case(
                "E#",
                Minor,
                "E# D# C# B# A# G# F## E# F## G# A# B# C# D# E#",
            );

            descending_ascending_test_case(
                "Fb",
                Minor,
                "Fb Ebb Dbb Cb Bbb Abb Gb Fb Gb Abb Bbb Cb Dbb Ebb Fb",
            );
            descending_ascending_test_case("F", Minor, "F Eb Db C Bb Ab G F G Ab Bb C Db Eb F");
            descending_ascending_test_case("F#", Minor, "F# E D C# B A G# F# G# A B C# D E F#");

            descending_ascending_test_case(
                "Gb",
                Minor,
                "Gb Fb Ebb Db Cb Bbb Ab Gb Ab Bbb Cb Db Ebb Fb Gb",
            );
            descending_ascending_test_case("G", Minor, "G F Eb D C Bb A G A Bb C D Eb F G");
            descending_ascending_test_case("G#", Minor, "G# F# E D# C# B A# G# A# B C# D# E F# G#");
        }

        #[test]
        fn creates_descending_ascending_ionian_scale() {
            descending_ascending_test_case("Ab", Ionian, "Ab G F Eb Db C Bb Ab Bb C Db Eb F G Ab");
            descending_ascending_test_case("A", Ionian, "A G# F# E D C# B A B C# D E F# G# A");
            descending_ascending_test_case(
                "A#",
                Ionian,
                "A# G## F## E# D# C## B# A# B# C## D# E# F## G## A#",
            );

            descending_ascending_test_case("Bb", Ionian, "Bb A G F Eb D C Bb C D Eb F G A Bb");
            descending_ascending_test_case("B", Ionian, "B A# G# F# E D# C# B C# D# E F# G# A# B");
            descending_ascending_test_case(
                "B#",
                Ionian,
                "B# A## G## F## E# D## C## B# C## D## E# F## G## A## B#",
            );

            descending_ascending_test_case(
                "Cb",
                Ionian,
                "Cb Bb Ab Gb Fb Eb Db Cb Db Eb Fb Gb Ab Bb Cb",
            );
            descending_ascending_test_case("C", Ionian, "C B A G F E D C D E F G A B C");
            descending_ascending_test_case(
                "C#",
                Ionian,
                "C# B# A# G# F# E# D# C# D# E# F# G# A# B# C#",
            );

            descending_ascending_test_case(
                "Db",
                Ionian,
                "Db C Bb Ab Gb F Eb Db Eb F Gb Ab Bb C Db",
            );
            descending_ascending_test_case("D", Ionian, "D C# B A G F# E D E F# G A B C# D");
            descending_ascending_test_case(
                "D#",
                Ionian,
                "D# C## B# A# G# F## E# D# E# F## G# A# B# C## D#",
            );

            descending_ascending_test_case("Eb", Ionian, "Eb D C Bb Ab G F Eb F G Ab Bb C D Eb");
            descending_ascending_test_case("E", Ionian, "E D# C# B A G# F# E F# G# A B C# D# E");
            descending_ascending_test_case(
                "E#",
                Ionian,
                "E# D## C## B# A# G## F## E# F## G## A# B# C## D## E#",
            );

            descending_ascending_test_case(
                "Fb",
                Ionian,
                "Fb Eb Db Cb Bbb Ab Gb Fb Gb Ab Bbb Cb Db Eb Fb",
            );
            descending_ascending_test_case("F", Ionian, "F E D C Bb A G F G A Bb C D E F");
            descending_ascending_test_case(
                "F#",
                Ionian,
                "F# E# D# C# B A# G# F# G# A# B C# D# E# F#",
            );

            descending_ascending_test_case(
                "Gb",
                Ionian,
                "Gb F Eb Db Cb Bb Ab Gb Ab Bb Cb Db Eb F Gb",
            );
            descending_ascending_test_case("G", Ionian, "G F# E D C B A G A B C D E F# G");
            descending_ascending_test_case(
                "G#",
                Ionian,
                "G# F## E# D# C# B# A# G# A# B# C# D# E# F## G#",
            );
        }

        #[test]
        fn creates_descending_ascending_dorian_scale() {
            descending_ascending_test_case(
                "Ab",
                Dorian,
                "Ab Gb F Eb Db Cb Bb Ab Bb Cb Db Eb F Gb Ab",
            );
            descending_ascending_test_case("A", Dorian, "A G F# E D C B A B C D E F# G A");
            descending_ascending_test_case(
                "A#",
                Dorian,
                "A# G# F## E# D# C# B# A# B# C# D# E# F## G# A#",
            );

            descending_ascending_test_case("Bb", Dorian, "Bb Ab G F Eb Db C Bb C Db Eb F G Ab Bb");
            descending_ascending_test_case("B", Dorian, "B A G# F# E D C# B C# D E F# G# A B");
            descending_ascending_test_case(
                "B#",
                Dorian,
                "B# A# G## F## E# D# C## B# C## D# E# F## G## A# B#",
            );

            descending_ascending_test_case(
                "Cb",
                Dorian,
                "Cb Bbb Ab Gb Fb Ebb Db Cb Db Ebb Fb Gb Ab Bbb Cb",
            );
            descending_ascending_test_case("C", Dorian, "C Bb A G F Eb D C D Eb F G A Bb C");
            descending_ascending_test_case(
                "C#",
                Dorian,
                "C# B A# G# F# E D# C# D# E F# G# A# B C#",
            );

            descending_ascending_test_case(
                "Db",
                Dorian,
                "Db Cb Bb Ab Gb Fb Eb Db Eb Fb Gb Ab Bb Cb Db",
            );
            descending_ascending_test_case("D", Dorian, "D C B A G F E D E F G A B C D");
            descending_ascending_test_case(
                "D#",
                Dorian,
                "D# C# B# A# G# F# E# D# E# F# G# A# B# C# D#",
            );

            descending_ascending_test_case(
                "Eb",
                Dorian,
                "Eb Db C Bb Ab Gb F Eb F Gb Ab Bb C Db Eb",
            );
            descending_ascending_test_case("E", Dorian, "E D C# B A G F# E F# G A B C# D E");
            descending_ascending_test_case(
                "E#",
                Dorian,
                "E# D# C## B# A# G# F## E# F## G# A# B# C## D# E#",
            );

            descending_ascending_test_case(
                "Fb",
                Dorian,
                "Fb Ebb Db Cb Bbb Abb Gb Fb Gb Abb Bbb Cb Db Ebb Fb",
            );
            descending_ascending_test_case("F", Dorian, "F Eb D C Bb Ab G F G Ab Bb C D Eb F");
            descending_ascending_test_case("F#", Dorian, "F# E D# C# B A G# F# G# A B C# D# E F#");

            descending_ascending_test_case(
                "Gb",
                Dorian,
                "Gb Fb Eb Db Cb Bbb Ab Gb Ab Bbb Cb Db Eb Fb Gb",
            );
            descending_ascending_test_case("G", Dorian, "G F E D C Bb A G A Bb C D E F G");
            descending_ascending_test_case(
                "G#",
                Dorian,
                "G# F# E# D# C# B A# G# A# B C# D# E# F# G#",
            );
        }

        #[test]
        fn creates_descending_ascending_phrygian_scale() {
            descending_ascending_test_case(
                "Ab",
                Phrygian,
                "Ab Gb Fb Eb Db Cb Bbb Ab Bbb Cb Db Eb Fb Gb Ab",
            );
            descending_ascending_test_case("A", Phrygian, "A G F E D C Bb A Bb C D E F G A");
            descending_ascending_test_case(
                "A#",
                Phrygian,
                "A# G# F# E# D# C# B A# B C# D# E# F# G# A#",
            );

            descending_ascending_test_case(
                "Bb",
                Phrygian,
                "Bb Ab Gb F Eb Db Cb Bb Cb Db Eb F Gb Ab Bb",
            );
            descending_ascending_test_case("B", Phrygian, "B A G F# E D C B C D E F# G A B");
            descending_ascending_test_case(
                "B#",
                Phrygian,
                "B# A# G# F## E# D# C# B# C# D# E# F## G# A# B#",
            );

            descending_ascending_test_case(
                "Cb",
                Phrygian,
                "Cb Bbb Abb Gb Fb Ebb Dbb Cb Dbb Ebb Fb Gb Abb Bbb Cb",
            );
            descending_ascending_test_case("C", Phrygian, "C Bb Ab G F Eb Db C Db Eb F G Ab Bb C");
            descending_ascending_test_case("C#", Phrygian, "C# B A G# F# E D C# D E F# G# A B C#");

            descending_ascending_test_case(
                "Db",
                Phrygian,
                "Db Cb Bbb Ab Gb Fb Ebb Db Ebb Fb Gb Ab Bbb Cb Db",
            );
            descending_ascending_test_case("D", Phrygian, "D C Bb A G F Eb D Eb F G A Bb C D");
            descending_ascending_test_case(
                "D#",
                Phrygian,
                "D# C# B A# G# F# E D# E F# G# A# B C# D#",
            );

            descending_ascending_test_case(
                "Eb",
                Phrygian,
                "Eb Db Cb Bb Ab Gb Fb Eb Fb Gb Ab Bb Cb Db Eb",
            );
            descending_ascending_test_case("E", Phrygian, "E D C B A G F E F G A B C D E");
            descending_ascending_test_case(
                "E#",
                Phrygian,
                "E# D# C# B# A# G# F# E# F# G# A# B# C# D# E#",
            );

            descending_ascending_test_case(
                "Fb",
                Phrygian,
                "Fb Ebb Dbb Cb Bbb Abb Gbb Fb Gbb Abb Bbb Cb Dbb Ebb Fb",
            );
            descending_ascending_test_case(
                "F",
                Phrygian,
                "F Eb Db C Bb Ab Gb F Gb Ab Bb C Db Eb F",
            );
            descending_ascending_test_case("F#", Phrygian, "F# E D C# B A G F# G A B C# D E F#");

            descending_ascending_test_case(
                "Gb",
                Phrygian,
                "Gb Fb Ebb Db Cb Bbb Abb Gb Abb Bbb Cb Db Ebb Fb Gb",
            );
            descending_ascending_test_case("G", Phrygian, "G F Eb D C Bb Ab G Ab Bb C D Eb F G");
            descending_ascending_test_case(
                "G#",
                Phrygian,
                "G# F# E D# C# B A G# A B C# D# E F# G#",
            );
        }

        #[test]
        fn creates_descending_ascending_lydian_scale() {
            descending_ascending_test_case("Ab", Lydian, "Ab G F Eb D C Bb Ab Bb C D Eb F G Ab");
            descending_ascending_test_case("A", Lydian, "A G# F# E D# C# B A B C# D# E F# G# A");
            descending_ascending_test_case(
                "A#",
                Lydian,
                "A# G## F## E# D## C## B# A# B# C## D## E# F## G## A#",
            );

            descending_ascending_test_case("Bb", Lydian, "Bb A G F E D C Bb C D E F G A Bb");
            descending_ascending_test_case(
                "B",
                Lydian,
                "B A# G# F# E# D# C# B C# D# E# F# G# A# B",
            );
            descending_ascending_test_case(
                "B#",
                Lydian,
                "B# A## G## F## E## D## C## B# C## D## E## F## G## A## B#",
            );

            descending_ascending_test_case(
                "Cb",
                Lydian,
                "Cb Bb Ab Gb F Eb Db Cb Db Eb F Gb Ab Bb Cb",
            );
            descending_ascending_test_case("C", Lydian, "C B A G F# E D C D E F# G A B C");
            descending_ascending_test_case(
                "C#",
                Lydian,
                "C# B# A# G# F## E# D# C# D# E# F## G# A# B# C#",
            );

            descending_ascending_test_case("Db", Lydian, "Db C Bb Ab G F Eb Db Eb F G Ab Bb C Db");
            descending_ascending_test_case("D", Lydian, "D C# B A G# F# E D E F# G# A B C# D");
            descending_ascending_test_case(
                "D#",
                Lydian,
                "D# C## B# A# G## F## E# D# E# F## G## A# B# C## D#",
            );

            descending_ascending_test_case("Eb", Lydian, "Eb D C Bb A G F Eb F G A Bb C D Eb");
            descending_ascending_test_case("E", Lydian, "E D# C# B A# G# F# E F# G# A# B C# D# E");
            descending_ascending_test_case(
                "E#",
                Lydian,
                "E# D## C## B# A## G## F## E# F## G## A## B# C## D## E#",
            );

            descending_ascending_test_case(
                "Fb",
                Lydian,
                "Fb Eb Db Cb Bb Ab Gb Fb Gb Ab Bb Cb Db Eb Fb",
            );
            descending_ascending_test_case("F", Lydian, "F E D C B A G F G A B C D E F");
            descending_ascending_test_case(
                "F#",
                Lydian,
                "F# E# D# C# B# A# G# F# G# A# B# C# D# E# F#",
            );

            descending_ascending_test_case(
                "Gb",
                Lydian,
                "Gb F Eb Db C Bb Ab Gb Ab Bb C Db Eb F Gb",
            );
            descending_ascending_test_case("G", Lydian, "G F# E D C# B A G A B C# D E F# G");
            descending_ascending_test_case(
                "G#",
                Lydian,
                "G# F## E# D# C## B# A# G# A# B# C## D# E# F## G#",
            );
        }

        #[test]
        fn creates_descending_ascending_mixolydian_scale() {
            descending_ascending_test_case(
                "Ab",
                Mixolydian,
                "Ab Gb F Eb Db C Bb Ab Bb C Db Eb F Gb Ab",
            );
            descending_ascending_test_case("A", Mixolydian, "A G F# E D C# B A B C# D E F# G A");
            descending_ascending_test_case(
                "A#",
                Mixolydian,
                "A# G# F## E# D# C## B# A# B# C## D# E# F## G# A#",
            );

            descending_ascending_test_case(
                "Bb",
                Mixolydian,
                "Bb Ab G F Eb D C Bb C D Eb F G Ab Bb",
            );
            descending_ascending_test_case(
                "B",
                Mixolydian,
                "B A G# F# E D# C# B C# D# E F# G# A B",
            );
            descending_ascending_test_case(
                "B#",
                Mixolydian,
                "B# A# G## F## E# D## C## B# C## D## E# F## G## A# B#",
            );

            descending_ascending_test_case(
                "Cb",
                Mixolydian,
                "Cb Bbb Ab Gb Fb Eb Db Cb Db Eb Fb Gb Ab Bbb Cb",
            );
            descending_ascending_test_case("C", Mixolydian, "C Bb A G F E D C D E F G A Bb C");
            descending_ascending_test_case(
                "C#",
                Mixolydian,
                "C# B A# G# F# E# D# C# D# E# F# G# A# B C#",
            );

            descending_ascending_test_case(
                "Db",
                Mixolydian,
                "Db Cb Bb Ab Gb F Eb Db Eb F Gb Ab Bb Cb Db",
            );
            descending_ascending_test_case("D", Mixolydian, "D C B A G F# E D E F# G A B C D");
            descending_ascending_test_case(
                "D#",
                Mixolydian,
                "D# C# B# A# G# F## E# D# E# F## G# A# B# C# D#",
            );

            descending_ascending_test_case(
                "Eb",
                Mixolydian,
                "Eb Db C Bb Ab G F Eb F G Ab Bb C Db Eb",
            );
            descending_ascending_test_case("E", Mixolydian, "E D C# B A G# F# E F# G# A B C# D E");
            descending_ascending_test_case(
                "E#",
                Mixolydian,
                "E# D# C## B# A# G## F## E# F## G## A# B# C## D# E#",
            );

            descending_ascending_test_case(
                "Fb",
                Mixolydian,
                "Fb Ebb Db Cb Bbb Ab Gb Fb Gb Ab Bbb Cb Db Ebb Fb",
            );
            descending_ascending_test_case("F", Mixolydian, "F Eb D C Bb A G F G A Bb C D Eb F");
            descending_ascending_test_case(
                "F#",
                Mixolydian,
                "F# E D# C# B A# G# F# G# A# B C# D# E F#",
            );

            descending_ascending_test_case(
                "Gb",
                Mixolydian,
                "Gb Fb Eb Db Cb Bb Ab Gb Ab Bb Cb Db Eb Fb Gb",
            );
            descending_ascending_test_case("G", Mixolydian, "G F E D C B A G A B C D E F G");
            descending_ascending_test_case(
                "G#",
                Mixolydian,
                "G# F# E# D# C# B# A# G# A# B# C# D# E# F# G#",
            );
        }

        #[test]
        fn creates_descending_ascending_aeolian_scale() {
            descending_ascending_test_case(
                "Ab",
                Aeolian,
                "Ab Gb Fb Eb Db Cb Bb Ab Bb Cb Db Eb Fb Gb Ab",
            );
            descending_ascending_test_case("A", Aeolian, "A G F E D C B A B C D E F G A");
            descending_ascending_test_case(
                "A#",
                Aeolian,
                "A# G# F# E# D# C# B# A# B# C# D# E# F# G# A#",
            );

            descending_ascending_test_case(
                "Bb",
                Aeolian,
                "Bb Ab Gb F Eb Db C Bb C Db Eb F Gb Ab Bb",
            );
            descending_ascending_test_case("B", Aeolian, "B A G F# E D C# B C# D E F# G A B");
            descending_ascending_test_case(
                "B#",
                Aeolian,
                "B# A# G# F## E# D# C## B# C## D# E# F## G# A# B#",
            );

            descending_ascending_test_case(
                "Cb",
                Aeolian,
                "Cb Bbb Abb Gb Fb Ebb Db Cb Db Ebb Fb Gb Abb Bbb Cb",
            );
            descending_ascending_test_case("C", Aeolian, "C Bb Ab G F Eb D C D Eb F G Ab Bb C");
            descending_ascending_test_case("C#", Aeolian, "C# B A G# F# E D# C# D# E F# G# A B C#");

            descending_ascending_test_case(
                "Db",
                Aeolian,
                "Db Cb Bbb Ab Gb Fb Eb Db Eb Fb Gb Ab Bbb Cb Db",
            );
            descending_ascending_test_case("D", Aeolian, "D C Bb A G F E D E F G A Bb C D");
            descending_ascending_test_case(
                "D#",
                Aeolian,
                "D# C# B A# G# F# E# D# E# F# G# A# B C# D#",
            );

            descending_ascending_test_case(
                "Eb",
                Aeolian,
                "Eb Db Cb Bb Ab Gb F Eb F Gb Ab Bb Cb Db Eb",
            );
            descending_ascending_test_case("E", Aeolian, "E D C B A G F# E F# G A B C D E");
            descending_ascending_test_case(
                "E#",
                Aeolian,
                "E# D# C# B# A# G# F## E# F## G# A# B# C# D# E#",
            );

            descending_ascending_test_case(
                "Fb",
                Aeolian,
                "Fb Ebb Dbb Cb Bbb Abb Gb Fb Gb Abb Bbb Cb Dbb Ebb Fb",
            );
            descending_ascending_test_case("F", Aeolian, "F Eb Db C Bb Ab G F G Ab Bb C Db Eb F");
            descending_ascending_test_case("F#", Aeolian, "F# E D C# B A G# F# G# A B C# D E F#");

            descending_ascending_test_case(
                "Gb",
                Aeolian,
                "Gb Fb Ebb Db Cb Bbb Ab Gb Ab Bbb Cb Db Ebb Fb Gb",
            );
            descending_ascending_test_case("G", Aeolian, "G F Eb D C Bb A G A Bb C D Eb F G");
            descending_ascending_test_case(
                "G#",
                Aeolian,
                "G# F# E D# C# B A# G# A# B C# D# E F# G#",
            );
        }

        #[test]
        fn creates_descending_ascending_locrian_scale() {
            descending_ascending_test_case(
                "Ab",
                Locrian,
                "Ab Gb Fb Ebb Db Cb Bbb Ab Bbb Cb Db Ebb Fb Gb Ab",
            );
            descending_ascending_test_case("A", Locrian, "A G F Eb D C Bb A Bb C D Eb F G A");
            descending_ascending_test_case(
                "A#",
                Locrian,
                "A# G# F# E D# C# B A# B C# D# E F# G# A#",
            );

            descending_ascending_test_case(
                "Bb",
                Locrian,
                "Bb Ab Gb Fb Eb Db Cb Bb Cb Db Eb Fb Gb Ab Bb",
            );
            descending_ascending_test_case("B", Locrian, "B A G F E D C B C D E F G A B");
            descending_ascending_test_case(
                "B#",
                Locrian,
                "B# A# G# F# E# D# C# B# C# D# E# F# G# A# B#",
            );

            descending_ascending_test_case(
                "Cb",
                Locrian,
                "Cb Bbb Abb Gbb Fb Ebb Dbb Cb Dbb Ebb Fb Gbb Abb Bbb Cb",
            );
            descending_ascending_test_case("C", Locrian, "C Bb Ab Gb F Eb Db C Db Eb F Gb Ab Bb C");
            descending_ascending_test_case("C#", Locrian, "C# B A G F# E D C# D E F# G A B C#");

            descending_ascending_test_case(
                "Db",
                Locrian,
                "Db Cb Bbb Abb Gb Fb Ebb Db Ebb Fb Gb Abb Bbb Cb Db",
            );
            descending_ascending_test_case("D", Locrian, "D C Bb Ab G F Eb D Eb F G Ab Bb C D");
            descending_ascending_test_case("D#", Locrian, "D# C# B A G# F# E D# E F# G# A B C# D#");

            descending_ascending_test_case(
                "Eb",
                Locrian,
                "Eb Db Cb Bbb Ab Gb Fb Eb Fb Gb Ab Bbb Cb Db Eb",
            );
            descending_ascending_test_case("E", Locrian, "E D C Bb A G F E F G A Bb C D E");
            descending_ascending_test_case(
                "E#",
                Locrian,
                "E# D# C# B A# G# F# E# F# G# A# B C# D# E#",
            );

            descending_ascending_test_case(
                "Fb",
                Locrian,
                "Fb Ebb Dbb Cbb Bbb Abb Gbb Fb Gbb Abb Bbb Cbb Dbb Ebb Fb",
            );
            descending_ascending_test_case(
                "F",
                Locrian,
                "F Eb Db Cb Bb Ab Gb F Gb Ab Bb Cb Db Eb F",
            );
            descending_ascending_test_case("F#", Locrian, "F# E D C B A G F# G A B C D E F#");

            descending_ascending_test_case(
                "Gb",
                Locrian,
                "Gb Fb Ebb Dbb Cb Bbb Abb Gb Abb Bbb Cb Dbb Ebb Fb Gb",
            );
            descending_ascending_test_case("G", Locrian, "G F Eb Db C Bb Ab G Ab Bb C Db Eb F G");
            descending_ascending_test_case("G#", Locrian, "G# F# E D C# B A G# A B C# D E F# G#");
        }

        #[test]
        fn creates_descending_ascending_major_pentatonic_scale() {
            descending_ascending_test_case("Ab", MajorPentatonic, "Ab F Eb C Bb Ab Bb C Eb F Ab");
            descending_ascending_test_case("A", MajorPentatonic, "A F# E C# B A B C# E F# A");
            descending_ascending_test_case(
                "A#",
                MajorPentatonic,
                "A# F## E# C## B# A# B# C## E# F## A#",
            );

            descending_ascending_test_case("Bb", MajorPentatonic, "Bb G F D C Bb C D F G Bb");
            descending_ascending_test_case("B", MajorPentatonic, "B G# F# D# C# B C# D# F# G# B");
            descending_ascending_test_case(
                "B#",
                MajorPentatonic,
                "B# G## F## D## C## B# C## D## F## G## B#",
            );

            descending_ascending_test_case(
                "Cb",
                MajorPentatonic,
                "Cb Ab Gb Eb Db Cb Db Eb Gb Ab Cb",
            );
            descending_ascending_test_case("C", MajorPentatonic, "C A G E D C D E G A C");
            descending_ascending_test_case(
                "C#",
                MajorPentatonic,
                "C# A# G# E# D# C# D# E# G# A# C#",
            );

            descending_ascending_test_case("Db", MajorPentatonic, "Db Bb Ab F Eb Db Eb F Ab Bb Db");
            descending_ascending_test_case("D", MajorPentatonic, "D B A F# E D E F# A B D");
            descending_ascending_test_case(
                "D#",
                MajorPentatonic,
                "D# B# A# F## E# D# E# F## A# B# D#",
            );

            descending_ascending_test_case("Eb", MajorPentatonic, "Eb C Bb G F Eb F G Bb C Eb");
            descending_ascending_test_case("E", MajorPentatonic, "E C# B G# F# E F# G# B C# E");
            descending_ascending_test_case(
                "E#",
                MajorPentatonic,
                "E# C## B# G## F## E# F## G## B# C## E#",
            );

            descending_ascending_test_case(
                "Fb",
                MajorPentatonic,
                "Fb Db Cb Ab Gb Fb Gb Ab Cb Db Fb",
            );
            descending_ascending_test_case("F", MajorPentatonic, "F D C A G F G A C D F");
            descending_ascending_test_case(
                "F#",
                MajorPentatonic,
                "F# D# C# A# G# F# G# A# C# D# F#",
            );

            descending_ascending_test_case(
                "Gb",
                MajorPentatonic,
                "Gb Eb Db Bb Ab Gb Ab Bb Db Eb Gb",
            );
            descending_ascending_test_case("G", MajorPentatonic, "G E D B A G A B D E G");
            descending_ascending_test_case(
                "G#",
                MajorPentatonic,
                "G# E# D# B# A# G# A# B# D# E# G#",
            );
        }

        #[test]
        fn creates_descending_ascending_minor_pentatonic_scale() {
            descending_ascending_test_case(
                "Ab",
                MinorPentatonic,
                "Ab Gb Eb Db Cb Ab Cb Db Eb Gb Ab",
            );
            descending_ascending_test_case("A", MinorPentatonic, "A G E D C A C D E G A");
            descending_ascending_test_case(
                "A#",
                MinorPentatonic,
                "A# G# E# D# C# A# C# D# E# G# A#",
            );

            descending_ascending_test_case("Bb", MinorPentatonic, "Bb Ab F Eb Db Bb Db Eb F Ab Bb");
            descending_ascending_test_case("B", MinorPentatonic, "B A F# E D B D E F# A B");
            descending_ascending_test_case(
                "B#",
                MinorPentatonic,
                "B# A# F## E# D# B# D# E# F## A# B#",
            );

            descending_ascending_test_case(
                "Cb",
                MinorPentatonic,
                "Cb Bbb Gb Fb Ebb Cb Ebb Fb Gb Bbb Cb",
            );
            descending_ascending_test_case("C", MinorPentatonic, "C Bb G F Eb C Eb F G Bb C");
            descending_ascending_test_case("C#", MinorPentatonic, "C# B G# F# E C# E F# G# B C#");

            descending_ascending_test_case(
                "Db",
                MinorPentatonic,
                "Db Cb Ab Gb Fb Db Fb Gb Ab Cb Db",
            );
            descending_ascending_test_case("D", MinorPentatonic, "D C A G F D F G A C D");
            descending_ascending_test_case(
                "D#",
                MinorPentatonic,
                "D# C# A# G# F# D# F# G# A# C# D#",
            );

            descending_ascending_test_case(
                "Eb",
                MinorPentatonic,
                "Eb Db Bb Ab Gb Eb Gb Ab Bb Db Eb",
            );
            descending_ascending_test_case("E", MinorPentatonic, "E D B A G E G A B D E");
            descending_ascending_test_case(
                "E#",
                MinorPentatonic,
                "E# D# B# A# G# E# G# A# B# D# E#",
            );

            descending_ascending_test_case(
                "Fb",
                MinorPentatonic,
                "Fb Ebb Cb Bbb Abb Fb Abb Bbb Cb Ebb Fb",
            );
            descending_ascending_test_case("F", MinorPentatonic, "F Eb C Bb Ab F Ab Bb C Eb F");
            descending_ascending_test_case("F#", MinorPentatonic, "F# E C# B A F# A B C# E F#");

            descending_ascending_test_case(
                "Gb",
                MinorPentatonic,
                "Gb Fb Db Cb Bbb Gb Bbb Cb Db Fb Gb",
            );
            descending_ascending_test_case("G", MinorPentatonic, "G F D C Bb G Bb C D F G");
            descending_ascending_test_case("G#", MinorPentatonic, "G# F# D# C# B G# B C# D# F# G#");
        }

        #[test]
        fn creates_descending_ascending_harmonic_minor_scale() {
            descending_ascending_test_case(
                "Ab",
                HarmonicMinor,
                "Ab G Fb Eb Db Cb Bb Ab Bb Cb Db Eb Fb G Ab",
            );
            descending_ascending_test_case("A", HarmonicMinor, "A G# F E D C B A B C D E F G# A");
            descending_ascending_test_case(
                "A#",
                HarmonicMinor,
                "A# G## F# E# D# C# B# A# B# C# D# E# F# G## A#",
            );

            descending_ascending_test_case(
                "Bb",
                HarmonicMinor,
                "Bb A Gb F Eb Db C Bb C Db Eb F Gb A Bb",
            );
            descending_ascending_test_case(
                "B",
                HarmonicMinor,
                "B A# G F# E D C# B C# D E F# G A# B",
            );
            descending_ascending_test_case(
                "B#",
                HarmonicMinor,
                "B# A## G# F## E# D# C## B# C## D# E# F## G# A## B#",
            );

            descending_ascending_test_case(
                "Cb",
                HarmonicMinor,
                "Cb Bb Abb Gb Fb Ebb Db Cb Db Ebb Fb Gb Abb Bb Cb",
            );
            descending_ascending_test_case("C", HarmonicMinor, "C B Ab G F Eb D C D Eb F G Ab B C");
            descending_ascending_test_case(
                "C#",
                HarmonicMinor,
                "C# B# A G# F# E D# C# D# E F# G# A B# C#",
            );

            descending_ascending_test_case(
                "Db",
                HarmonicMinor,
                "Db C Bbb Ab Gb Fb Eb Db Eb Fb Gb Ab Bbb C Db",
            );
            descending_ascending_test_case("D", HarmonicMinor, "D C# Bb A G F E D E F G A Bb C# D");
            descending_ascending_test_case(
                "D#",
                HarmonicMinor,
                "D# C## B A# G# F# E# D# E# F# G# A# B C## D#",
            );

            descending_ascending_test_case(
                "Eb",
                HarmonicMinor,
                "Eb D Cb Bb Ab Gb F Eb F Gb Ab Bb Cb D Eb",
            );
            descending_ascending_test_case("E", HarmonicMinor, "E D# C B A G F# E F# G A B C D# E");
            descending_ascending_test_case(
                "E#",
                HarmonicMinor,
                "E# D## C# B# A# G# F## E# F## G# A# B# C# D## E#",
            );

            descending_ascending_test_case(
                "Fb",
                HarmonicMinor,
                "Fb Eb Dbb Cb Bbb Abb Gb Fb Gb Abb Bbb Cb Dbb Eb Fb",
            );
            descending_ascending_test_case(
                "F",
                HarmonicMinor,
                "F E Db C Bb Ab G F G Ab Bb C Db E F",
            );
            descending_ascending_test_case(
                "F#",
                HarmonicMinor,
                "F# E# D C# B A G# F# G# A B C# D E# F#",
            );

            descending_ascending_test_case(
                "Gb",
                HarmonicMinor,
                "Gb F Ebb Db Cb Bbb Ab Gb Ab Bbb Cb Db Ebb F Gb",
            );
            descending_ascending_test_case(
                "G",
                HarmonicMinor,
                "G F# Eb D C Bb A G A Bb C D Eb F# G",
            );
            descending_ascending_test_case(
                "G#",
                HarmonicMinor,
                "G# F## E D# C# B A# G# A# B C# D# E F## G#",
            );
        }

        #[test]
        fn creates_descending_ascending_melodic_minor_scale() {
            descending_ascending_test_case(
                "Ab",
                MelodicMinor,
                "Ab Gb Fb Eb Db Cb Bb Ab Bb Cb Db Eb F G Ab",
            );
            descending_ascending_test_case("A", MelodicMinor, "A G F E D C B A B C D E F# G# A");
            descending_ascending_test_case(
                "A#",
                MelodicMinor,
                "A# G# F# E# D# C# B# A# B# C# D# E# F## G## A#",
            );

            descending_ascending_test_case(
                "Bb",
                MelodicMinor,
                "Bb Ab Gb F Eb Db C Bb C Db Eb F G A Bb",
            );
            descending_ascending_test_case(
                "B",
                MelodicMinor,
                "B A G F# E D C# B C# D E F# G# A# B",
            );
            descending_ascending_test_case(
                "B#",
                MelodicMinor,
                "B# A# G# F## E# D# C## B# C## D# E# F## G## A## B#",
            );

            descending_ascending_test_case(
                "Cb",
                MelodicMinor,
                "Cb Bbb Abb Gb Fb Ebb Db Cb Db Ebb Fb Gb Ab Bb Cb",
            );
            descending_ascending_test_case("C", MelodicMinor, "C Bb Ab G F Eb D C D Eb F G A B C");
            descending_ascending_test_case(
                "C#",
                MelodicMinor,
                "C# B A G# F# E D# C# D# E F# G# A# B# C#",
            );

            descending_ascending_test_case(
                "Db",
                MelodicMinor,
                "Db Cb Bbb Ab Gb Fb Eb Db Eb Fb Gb Ab Bb C Db",
            );
            descending_ascending_test_case("D", MelodicMinor, "D C Bb A G F E D E F G A B C# D");
            descending_ascending_test_case(
                "D#",
                MelodicMinor,
                "D# C# B A# G# F# E# D# E# F# G# A# B# C## D#",
            );

            descending_ascending_test_case(
                "Eb",
                MelodicMinor,
                "Eb Db Cb Bb Ab Gb F Eb F Gb Ab Bb C D Eb",
            );
            descending_ascending_test_case("E", MelodicMinor, "E D C B A G F# E F# G A B C# D# E");
            descending_ascending_test_case(
                "E#",
                MelodicMinor,
                "E# D# C# B# A# G# F## E# F## G# A# B# C## D## E#",
            );

            descending_ascending_test_case(
                "Fb",
                MelodicMinor,
                "Fb Ebb Dbb Cb Bbb Abb Gb Fb Gb Abb Bbb Cb Db Eb Fb",
            );
            descending_ascending_test_case(
                "F",
                MelodicMinor,
                "F Eb Db C Bb Ab G F G Ab Bb C D E F",
            );
            descending_ascending_test_case(
                "F#",
                MelodicMinor,
                "F# E D C# B A G# F# G# A B C# D# E# F#",
            );

            descending_ascending_test_case(
                "Gb",
                MelodicMinor,
                "Gb Fb Ebb Db Cb Bbb Ab Gb Ab Bbb Cb Db Eb F Gb",
            );
            descending_ascending_test_case("G", MelodicMinor, "G F Eb D C Bb A G A Bb C D E F# G");
            descending_ascending_test_case(
                "G#",
                MelodicMinor,
                "G# F# E D# C# B A# G# A# B C# D# E# F## G#",
            );
        }
    }
}
