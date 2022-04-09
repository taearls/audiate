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
            descending_test_case("A", Major, "A G# F# E D C# B A");
        }

        #[test]
        fn creates_descending_melodic_minor_scale() {
            descending_test_case("G", MelodicMinor, "G F Eb D C Bb A G");
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
        }
    }

    mod descending_ascending_scales {
        use super::*;

        fn descending_ascending_test_case(root_note_str: &str, kind: ScaleKind, expected: &str) {
            test_case(root_note_str, kind, DescendingAscending, expected);
        }

        #[test]
        fn creates_descending_ascending_major_scale() {
            descending_ascending_test_case("A", Major, "A G# F# E D C# B A B C# D E F# G# A");
        }
    }
}
