// use super::note::Note;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NotePitchInterval {
    PerfectUnison,
    MinorSecond,
    MajorSecond,
    MinorThird,
    MajorThird,
    PerfectFourth,
    AugmentedFourth,
    DiminishedFifth,
    PerfectFifth,
    MinorSixth,
    MajorSixth,
    MinorSeventh,
    MajorSeventh,
    // Octave,
    // MinorNinth,
    // MajorNinth,
    // MinorTenth,
    // MajorTenth,
    // MinorEleventh,
    // MajorEleventh,
    // MinorTwelfth,
    // MajorTwelfth,
    // MinorThirteenth,
    // MajorThirteenth,
}

impl NotePitchInterval {
    // returns the inverted pitch if you change between an ascending and descending interval
    pub fn invert(&self) -> NotePitchInterval {
        match self {
            NotePitchInterval::PerfectUnison => NotePitchInterval::PerfectUnison,
            NotePitchInterval::MinorSecond => NotePitchInterval::MajorSeventh,
            NotePitchInterval::MajorSecond => NotePitchInterval::MinorSeventh,
            NotePitchInterval::MinorThird => NotePitchInterval::MajorSixth,
            NotePitchInterval::MajorThird => NotePitchInterval::MinorSixth,
            NotePitchInterval::PerfectFourth => NotePitchInterval::PerfectFifth,
            NotePitchInterval::AugmentedFourth => NotePitchInterval::DiminishedFifth,
            NotePitchInterval::DiminishedFifth => NotePitchInterval::AugmentedFourth,
            NotePitchInterval::PerfectFifth => NotePitchInterval::PerfectFourth,
            NotePitchInterval::MinorSixth => NotePitchInterval::MajorThird,
            NotePitchInterval::MajorSixth => NotePitchInterval::MinorThird,
            NotePitchInterval::MinorSeventh => NotePitchInterval::MajorSecond,
            NotePitchInterval::MajorSeventh => NotePitchInterval::MinorSecond,
        }
    }
}

impl std::ops::Add<NotePitchInterval> for u8 {
    type Output = Self;
    fn add(self, other: NotePitchInterval) -> Self {
        let pitch_interval_value: u8 = match other {
            NotePitchInterval::PerfectUnison => 0,
            NotePitchInterval::MinorSecond => 1,
            NotePitchInterval::MajorSecond => 2,
            NotePitchInterval::MinorThird => 3,
            NotePitchInterval::MajorThird => 4,
            NotePitchInterval::PerfectFourth => 5,
            NotePitchInterval::AugmentedFourth | NotePitchInterval::DiminishedFifth => 6,
            NotePitchInterval::PerfectFifth => 7,
            NotePitchInterval::MinorSixth => 8,
            NotePitchInterval::MajorSixth => 9,
            NotePitchInterval::MinorSeventh => 10,
            NotePitchInterval::MajorSeventh => 11,
        };
        let sum = self + pitch_interval_value;
        // we want only pitch values to be in the range of 0-11 inclusive
        sum % 12
    }
}

impl std::ops::Add<u8> for NotePitchInterval {
    type Output = Self;
    fn add(self, other: u8) -> Self {
        let pitch_interval_value: u8 = match self {
            NotePitchInterval::PerfectUnison => 0,
            NotePitchInterval::MinorSecond => 1,
            NotePitchInterval::MajorSecond => 2,
            NotePitchInterval::MinorThird => 3,
            NotePitchInterval::MajorThird => 4,
            NotePitchInterval::PerfectFourth => 5,
            NotePitchInterval::AugmentedFourth | NotePitchInterval::DiminishedFifth => 6,
            NotePitchInterval::PerfectFifth => 7,
            NotePitchInterval::MinorSixth => 8,
            NotePitchInterval::MajorSixth => 9,
            NotePitchInterval::MinorSeventh => 10,
            NotePitchInterval::MajorSeventh => 11,
        };
        let sum = pitch_interval_value + other;
        // we want only pitch values to be in the range of 0-11 inclusive
        match sum % 12 {
            1 => NotePitchInterval::MinorSecond,
            2 => NotePitchInterval::MajorSecond,
            3 => NotePitchInterval::MinorThird,
            4 => NotePitchInterval::MajorThird,
            5 => NotePitchInterval::PerfectFourth,
            6 => {
                // if adding to AugmentedFourth, return that; if adding to DiminishedFifth, return that
                self
            }
            7 => NotePitchInterval::PerfectFifth,
            8 => NotePitchInterval::MinorSixth,
            9 => NotePitchInterval::MajorSixth,
            10 => NotePitchInterval::MinorSeventh,
            11 => NotePitchInterval::MajorSeventh,
            _ => NotePitchInterval::PerfectUnison,
        }
    }
}

////////////////
// UNIT TESTS //
////////////////

#[cfg(test)]
mod by_interval_ascending_test {
    use super::*;
    use crate::Note;
    use NotePitchInterval::*;

    fn test_case(start_note_name: &str, interval: NotePitchInterval, end_note_name: &str) {
        let note = Note::try_from(start_note_name).unwrap();
        let actual = note.by_interval_ascending(interval).unwrap();
        let expected = Note::try_from(end_note_name).unwrap();
        assert_eq!(format!("{actual}"), format!("{expected}"));
    }

    #[test]
    fn by_interval_ascending_works_with_a_flat() {
        test_case("Ab", PerfectUnison, "Ab");
        test_case("Ab", MinorSecond, "Bbb");
        test_case("Ab", MajorSecond, "Bb");
        test_case("Ab", MinorThird, "Cb");
        test_case("Ab", MajorThird, "C");
        test_case("Ab", PerfectFourth, "Db");
        test_case("Ab", AugmentedFourth, "D");
        test_case("Ab", DiminishedFifth, "Ebb");
        test_case("Ab", PerfectFifth, "Eb");
        test_case("Ab", MinorSixth, "Fb");
        test_case("Ab", MajorSixth, "F");
        test_case("Ab", MinorSeventh, "Gb");
        test_case("Ab", MajorSeventh, "G");
    }

    #[test]
    fn by_interval_ascending_works_with_a_natural() {
        test_case("A", PerfectUnison, "A");
        test_case("A", MinorSecond, "Bb");
        test_case("A", MajorSecond, "B");
        test_case("A", MinorThird, "C");
        test_case("A", MajorThird, "C#");
        test_case("A", PerfectFourth, "D");
        test_case("A", AugmentedFourth, "D#");
        test_case("A", DiminishedFifth, "Eb");
        test_case("A", PerfectFifth, "E");
        test_case("A", MinorSixth, "F");
        test_case("A", MajorSixth, "F#");
        test_case("A", MinorSeventh, "G");
        test_case("A", MajorSeventh, "G#");
    }

    #[test]
    fn by_interval_ascending_works_with_a_sharp() {
        test_case("A#", PerfectUnison, "A#");
        test_case("A#", MinorSecond, "B");
        test_case("A#", MajorSecond, "B#");
        test_case("A#", MinorThird, "C#");
        test_case("A#", MajorThird, "C##");
        test_case("A#", PerfectFourth, "D#");
        test_case("A#", AugmentedFourth, "D##");
        test_case("A#", DiminishedFifth, "E");
        test_case("A#", PerfectFifth, "E#");
        test_case("A#", MinorSixth, "F#");
        test_case("A#", MajorSixth, "F##");
        test_case("A#", MinorSeventh, "G#");
        test_case("A#", MajorSeventh, "G##");
    }

    #[test]
    fn by_interval_ascending_works_with_b_flat() {
        test_case("Bb", PerfectUnison, "Bb");
        test_case("Bb", MinorSecond, "Cb");
        test_case("Bb", MajorSecond, "C");
        test_case("Bb", MinorThird, "Db");
        test_case("Bb", MajorThird, "D");
        test_case("Bb", PerfectFourth, "Eb");
        test_case("Bb", AugmentedFourth, "E");
        test_case("Bb", DiminishedFifth, "Fb");
        test_case("Bb", PerfectFifth, "F");
        test_case("Bb", MinorSixth, "Gb");
        test_case("Bb", MajorSixth, "G");
        test_case("Bb", MinorSeventh, "Ab");
        test_case("Bb", MajorSeventh, "A");
    }

    #[test]
    fn by_interval_ascending_works_with_b_natural() {
        test_case("B", PerfectUnison, "B");
        test_case("B", MinorSecond, "C");
        test_case("B", MajorSecond, "C#");
        test_case("B", MinorThird, "D");
        test_case("B", MajorThird, "D#");
        test_case("B", PerfectFourth, "E");
        test_case("B", AugmentedFourth, "E#");
        test_case("B", DiminishedFifth, "F");
        test_case("B", PerfectFifth, "F#");
        test_case("B", MinorSixth, "G");
        test_case("B", MajorSixth, "G#");
        test_case("B", MinorSeventh, "A");
        test_case("B", MajorSeventh, "A#");
    }

    #[test]
    fn by_interval_ascending_works_with_b_sharp() {
        test_case("B#", PerfectUnison, "B#");
        test_case("B#", MinorSecond, "C#");
        test_case("B#", MajorSecond, "C##");
        test_case("B#", MinorThird, "D#");
        test_case("B#", MajorThird, "D##");
        test_case("B#", PerfectFourth, "E#");
        test_case("B#", AugmentedFourth, "E##");
        test_case("B#", DiminishedFifth, "F#");
        test_case("B#", PerfectFifth, "F##");
        test_case("B#", MinorSixth, "G#");
        test_case("B#", MajorSixth, "G##");
        test_case("B#", MinorSeventh, "A#");
        test_case("B#", MajorSeventh, "A##");
    }

    #[test]
    fn by_interval_ascending_works_with_c_flat() {
        test_case("Cb", PerfectUnison, "Cb");
        test_case("Cb", MinorSecond, "Dbb");
        test_case("Cb", MajorSecond, "Db");
        test_case("Cb", MinorThird, "Ebb");
        test_case("Cb", MajorThird, "Eb");
        test_case("Cb", PerfectFourth, "Fb");
        test_case("Cb", AugmentedFourth, "F");
        test_case("Cb", DiminishedFifth, "Gbb");
        test_case("Cb", PerfectFifth, "Gb");
        test_case("Cb", MinorSixth, "Abb");
        test_case("Cb", MajorSixth, "Ab");
        test_case("Cb", MinorSeventh, "Bbb");
        test_case("Cb", MajorSeventh, "Bb");
    }

    #[test]
    fn by_interval_ascending_works_with_c_natural() {
        test_case("C", PerfectUnison, "C");
        test_case("C", MinorSecond, "Db");
        test_case("C", MajorSecond, "D");
        test_case("C", MinorThird, "Eb");
        test_case("C", MajorThird, "E");
        test_case("C", PerfectFourth, "F");
        test_case("C", AugmentedFourth, "F#");
        test_case("C", DiminishedFifth, "Gb");
        test_case("C", PerfectFifth, "G");
        test_case("C", MinorSixth, "Ab");
        test_case("C", MajorSixth, "A");
        test_case("C", MinorSeventh, "Bb");
        test_case("C", MajorSeventh, "B");
    }

    #[test]
    fn by_interval_ascending_works_with_c_sharp() {
        test_case("C#", PerfectUnison, "C#");
        test_case("C#", MinorSecond, "D");
        test_case("C#", MajorSecond, "D#");
        test_case("C#", MinorThird, "E");
        test_case("C#", MajorThird, "E#");
        test_case("C#", PerfectFourth, "F#");
        test_case("C#", AugmentedFourth, "F##");
        test_case("C#", DiminishedFifth, "G");
        test_case("C#", PerfectFifth, "G#");
        test_case("C#", MinorSixth, "A");
        test_case("C#", MajorSixth, "A#");
        test_case("C#", MinorSeventh, "B");
        test_case("C#", MajorSeventh, "B#");
    }

    #[test]
    fn by_interval_ascending_works_with_d_flat() {
        test_case("Db", PerfectUnison, "Db");
        test_case("Db", MinorSecond, "Ebb");
        test_case("Db", MajorSecond, "Eb");
        test_case("Db", MinorThird, "Fb");
        test_case("Db", MajorThird, "F");
        test_case("Db", PerfectFourth, "Gb");
        test_case("Db", AugmentedFourth, "G");
        test_case("Db", DiminishedFifth, "Abb");
        test_case("Db", PerfectFifth, "Ab");
        test_case("Db", MinorSixth, "Bbb");
        test_case("Db", MajorSixth, "Bb");
        test_case("Db", MinorSeventh, "Cb");
        test_case("Db", MajorSeventh, "C");
    }

    #[test]
    fn by_interval_ascending_works_with_d_natural() {
        test_case("D", PerfectUnison, "D");
        test_case("D", MinorSecond, "Eb");
        test_case("D", MajorSecond, "E");
        test_case("D", MinorThird, "F");
        test_case("D", MajorThird, "F#");
        test_case("D", PerfectFourth, "G");
        test_case("D", AugmentedFourth, "G#");
        test_case("D", DiminishedFifth, "Ab");
        test_case("D", PerfectFifth, "A");
        test_case("D", MinorSixth, "Bb");
        test_case("D", MajorSixth, "B");
        test_case("D", MinorSeventh, "C");
        test_case("D", MajorSeventh, "C#");
    }

    #[test]
    fn by_interval_ascending_works_with_d_sharp() {
        test_case("D#", PerfectUnison, "D#");
        test_case("D#", MinorSecond, "E");
        test_case("D#", MajorSecond, "E#");
        test_case("D#", MinorThird, "F#");
        test_case("D#", MajorThird, "F##");
        test_case("D#", PerfectFourth, "G#");
        test_case("D#", AugmentedFourth, "G##");
        test_case("D#", DiminishedFifth, "A");
        test_case("D#", PerfectFifth, "A#");
        test_case("D#", MinorSixth, "B");
        test_case("D#", MajorSixth, "B#");
        test_case("D#", MinorSeventh, "C#");
        test_case("D#", MajorSeventh, "C##");
    }

    #[test]
    fn by_interval_ascending_works_with_e_flat() {
        test_case("Eb", PerfectUnison, "Eb");
        test_case("Eb", MinorSecond, "Fb");
        test_case("Eb", MajorSecond, "F");
        test_case("Eb", MinorThird, "Gb");
        test_case("Eb", MajorThird, "G");
        test_case("Eb", PerfectFourth, "Ab");
        test_case("Eb", AugmentedFourth, "A");
        test_case("Eb", DiminishedFifth, "Bbb");
        test_case("Eb", PerfectFifth, "Bb");
        test_case("Eb", MinorSixth, "Cb");
        test_case("Eb", MajorSixth, "C");
        test_case("Eb", MinorSeventh, "Db");
        test_case("Eb", MajorSeventh, "D");
    }

    #[test]
    fn by_interval_ascending_works_with_e_natural() {
        test_case("E", PerfectUnison, "E");
        test_case("E", MinorSecond, "F");
        test_case("E", MajorSecond, "F#");
        test_case("E", MinorThird, "G");
        test_case("E", MajorThird, "G#");
        test_case("E", PerfectFourth, "A");
        test_case("E", AugmentedFourth, "A#");
        test_case("E", DiminishedFifth, "Bb");
        test_case("E", PerfectFifth, "B");
        test_case("E", MinorSixth, "C");
        test_case("E", MajorSixth, "C#");
        test_case("E", MinorSeventh, "D");
        test_case("E", MajorSeventh, "D#");
    }

    #[test]
    fn by_interval_ascending_works_with_e_sharp() {
        test_case("E#", PerfectUnison, "E#");
        test_case("E#", MinorSecond, "F#");
        test_case("E#", MajorSecond, "F##");
        test_case("E#", MinorThird, "G#");
        test_case("E#", MajorThird, "G##");
        test_case("E#", PerfectFourth, "A#");
        test_case("E#", AugmentedFourth, "A##");
        test_case("E#", DiminishedFifth, "B");
        test_case("E#", PerfectFifth, "B#");
        test_case("E#", MinorSixth, "C#");
        test_case("E#", MajorSixth, "C##");
        test_case("E#", MinorSeventh, "D#");
        test_case("E#", MajorSeventh, "D##");
    }

    #[test]
    fn by_interval_ascending_works_with_f_flat() {
        test_case("Fb", PerfectUnison, "Fb");
        test_case("Fb", MinorSecond, "Gbb");
        test_case("Fb", MajorSecond, "Gb");
        test_case("Fb", MinorThird, "Abb");
        test_case("Fb", MajorThird, "Ab");
        test_case("Fb", PerfectFourth, "Bbb");
        test_case("Fb", AugmentedFourth, "Bb");
        test_case("Fb", DiminishedFifth, "Cbb");
        test_case("Fb", PerfectFifth, "Cb");
        test_case("Fb", MinorSixth, "Dbb");
        test_case("Fb", MajorSixth, "Db");
        test_case("Fb", MinorSeventh, "Ebb");
        test_case("Fb", MajorSeventh, "Eb");
    }

    #[test]
    fn by_interval_ascending_works_with_f_natural() {
        test_case("F", PerfectUnison, "F");
        test_case("F", MinorSecond, "Gb");
        test_case("F", MajorSecond, "G");
        test_case("F", MinorThird, "Ab");
        test_case("F", MajorThird, "A");
        test_case("F", PerfectFourth, "Bb");
        test_case("F", AugmentedFourth, "B");
        test_case("F", DiminishedFifth, "Cb");
        test_case("F", PerfectFifth, "C");
        test_case("F", MinorSixth, "Db");
        test_case("F", MajorSixth, "D");
        test_case("F", MinorSeventh, "Eb");
        test_case("F", MajorSeventh, "E");
    }

    #[test]
    fn by_interval_ascending_works_with_f_sharp() {
        test_case("F#", PerfectUnison, "F#");
        test_case("F#", MinorSecond, "G");
        test_case("F#", MajorSecond, "G#");
        test_case("F#", MinorThird, "A");
        test_case("F#", MajorThird, "A#");
        test_case("F#", PerfectFourth, "B");
        test_case("F#", AugmentedFourth, "B#");
        test_case("F#", DiminishedFifth, "C");
        test_case("F#", PerfectFifth, "C#");
        test_case("F#", MinorSixth, "D");
        test_case("F#", MajorSixth, "D#");
        test_case("F#", MinorSeventh, "E");
        test_case("F#", MajorSeventh, "E#");
    }

    #[test]
    fn by_interval_ascending_works_with_g_flat() {
        test_case("Gb", PerfectUnison, "Gb");
        test_case("Gb", MinorSecond, "Abb");
        test_case("Gb", MajorSecond, "Ab");
        test_case("Gb", MinorThird, "Bbb");
        test_case("Gb", MajorThird, "Bb");
        test_case("Gb", PerfectFourth, "Cb");
        test_case("Gb", AugmentedFourth, "C");
        test_case("Gb", DiminishedFifth, "Dbb");
        test_case("Gb", PerfectFifth, "Db");
        test_case("Gb", MinorSixth, "Ebb");
        test_case("Gb", MajorSixth, "Eb");
        test_case("Gb", MinorSeventh, "Fb");
        test_case("Gb", MajorSeventh, "F");
    }

    #[test]
    fn by_interval_ascending_works_with_g_natural() {
        test_case("G", PerfectUnison, "G");
        test_case("G", MinorSecond, "Ab");
        test_case("G", MajorSecond, "A");
        test_case("G", MinorThird, "Bb");
        test_case("G", MajorThird, "B");
        test_case("G", PerfectFourth, "C");
        test_case("G", AugmentedFourth, "C#");
        test_case("G", DiminishedFifth, "Db");
        test_case("G", PerfectFifth, "D");
        test_case("G", MinorSixth, "Eb");
        test_case("G", MajorSixth, "E");
        test_case("G", MinorSeventh, "F");
        test_case("G", MajorSeventh, "F#");
    }

    #[test]
    fn by_interval_ascending_works_with_g_sharp() {
        test_case("G#", PerfectUnison, "G#");
        test_case("G#", MinorSecond, "A");
        test_case("G#", MajorSecond, "A#");
        test_case("G#", MinorThird, "B");
        test_case("G#", MajorThird, "B#");
        test_case("G#", PerfectFourth, "C#");
        test_case("G#", AugmentedFourth, "C##");
        test_case("G#", DiminishedFifth, "D");
        test_case("G#", PerfectFifth, "D#");
        test_case("G#", MinorSixth, "E");
        test_case("G#", MajorSixth, "E#");
        test_case("G#", MinorSeventh, "F#");
        test_case("G#", MajorSeventh, "F##");
    }
}

#[cfg(test)]
mod by_interval_descending_test {
    use super::*;
    use crate::Note;
    use NotePitchInterval::*;

    fn test_case(start_note_name: &str, interval: NotePitchInterval, end_note_name: &str) {
        let note = Note::try_from(start_note_name).unwrap();
        let actual = note.by_interval_descending(interval).unwrap();
        let expected = Note::try_from(end_note_name).unwrap();
        assert_eq!(format!("{actual}"), format!("{expected}"));
    }

    #[test]
    fn by_interval_descending_works_with_a_flat() {
        test_case("Ab", PerfectUnison, "Ab");
        test_case("Ab", MinorSecond, "G");
        test_case("Ab", MajorSecond, "Gb");
        test_case("Ab", MinorThird, "F");
        test_case("Ab", MajorThird, "Fb");
        test_case("Ab", PerfectFourth, "Eb");
        test_case("Ab", AugmentedFourth, "Ebb");
        test_case("Ab", DiminishedFifth, "D");
        test_case("Ab", PerfectFifth, "Db");
        test_case("Ab", MinorSixth, "C");
        test_case("Ab", MajorSixth, "Cb");
        test_case("Ab", MinorSeventh, "Bb");
        test_case("Ab", MajorSeventh, "Bbb");
    }

    #[test]
    fn by_interval_descending_works_with_a_natural() {
        test_case("A", PerfectUnison, "A");
        test_case("A", MinorSecond, "G#");
        test_case("A", MajorSecond, "G");
        test_case("A", MinorThird, "F#");
        test_case("A", MajorThird, "F");
        test_case("A", PerfectFourth, "E");
        test_case("A", AugmentedFourth, "Eb");
        test_case("A", DiminishedFifth, "D#");
        test_case("A", PerfectFifth, "D");
        test_case("A", MinorSixth, "C#");
        test_case("A", MajorSixth, "C");
        test_case("A", MinorSeventh, "B");
        test_case("A", MajorSeventh, "Bb");
    }

    #[test]
    fn by_interval_descending_works_with_a_sharp() {
        test_case("A#", PerfectUnison, "A#");
        test_case("A#", MinorSecond, "G##");
        test_case("A#", MajorSecond, "G#");
        test_case("A#", MinorThird, "F##");
        test_case("A#", MajorThird, "F#");
        test_case("A#", PerfectFourth, "E#");
        test_case("A#", AugmentedFourth, "E");
        test_case("A#", DiminishedFifth, "D##");
        test_case("A#", PerfectFifth, "D#");
        test_case("A#", MinorSixth, "C##");
        test_case("A#", MajorSixth, "C#");
        test_case("A#", MinorSeventh, "B#");
        test_case("A#", MajorSeventh, "B");
    }

    #[test]
    fn by_interval_descending_works_with_b_flat() {
        test_case("Bb", PerfectUnison, "Bb");
        test_case("Bb", MinorSecond, "A");
        test_case("Bb", MajorSecond, "Ab");
        test_case("Bb", MinorThird, "G");
        test_case("Bb", MajorThird, "Gb");
        test_case("Bb", PerfectFourth, "F");
        test_case("Bb", AugmentedFourth, "Fb");
        test_case("Bb", DiminishedFifth, "E");
        test_case("Bb", PerfectFifth, "Eb");
        test_case("Bb", MinorSixth, "D");
        test_case("Bb", MajorSixth, "Db");
        test_case("Bb", MinorSeventh, "C");
        test_case("Bb", MajorSeventh, "Cb");
    }

    #[test]
    fn by_interval_descending_works_with_b_natural() {
        test_case("B", PerfectUnison, "B");
        test_case("B", MinorSecond, "A#");
        test_case("B", MajorSecond, "A");
        test_case("B", MinorThird, "G#");
        test_case("B", MajorThird, "G");
        test_case("B", PerfectFourth, "F#");
        test_case("B", AugmentedFourth, "F");
        test_case("B", DiminishedFifth, "E#");
        test_case("B", PerfectFifth, "E");
        test_case("B", MinorSixth, "D#");
        test_case("B", MajorSixth, "D");
        test_case("B", MinorSeventh, "C#");
        test_case("B", MajorSeventh, "C");
    }

    #[test]
    fn by_interval_descending_works_with_b_sharp() {
        test_case("B#", PerfectUnison, "B#");
        test_case("B#", MinorSecond, "A##");
        test_case("B#", MajorSecond, "A#");
        test_case("B#", MinorThird, "G##");
        test_case("B#", MajorThird, "G#");
        test_case("B#", PerfectFourth, "F##");
        test_case("B#", AugmentedFourth, "F#");
        test_case("B#", DiminishedFifth, "E##");
        test_case("B#", PerfectFifth, "E#");
        test_case("B#", MinorSixth, "D##");
        test_case("B#", MajorSixth, "D#");
        test_case("B#", MinorSeventh, "C##");
        test_case("B#", MajorSeventh, "C#");
    }

    #[test]
    fn by_interval_descending_works_with_c_flat() {
        test_case("Cb", PerfectUnison, "Cb");
        test_case("Cb", MinorSecond, "Bb");
        test_case("Cb", MajorSecond, "Bbb");
        test_case("Cb", MinorThird, "Ab");
        test_case("Cb", MajorThird, "Abb");
        test_case("Cb", PerfectFourth, "Gb");
        test_case("Cb", AugmentedFourth, "Gbb");
        test_case("Cb", DiminishedFifth, "F");
        test_case("Cb", PerfectFifth, "Fb");
        test_case("Cb", MinorSixth, "Eb");
        test_case("Cb", MajorSixth, "Ebb");
        test_case("Cb", MinorSeventh, "Db");
        test_case("Cb", MajorSeventh, "Dbb");
    }

    #[test]
    fn by_interval_descending_works_with_c_natural() {
        test_case("C", PerfectUnison, "C");
        test_case("C", MinorSecond, "B");
        test_case("C", MajorSecond, "Bb");
        test_case("C", MinorThird, "A");
        test_case("C", MajorThird, "Ab");
        test_case("C", PerfectFourth, "G");
        test_case("C", AugmentedFourth, "Gb");
        test_case("C", DiminishedFifth, "F#");
        test_case("C", PerfectFifth, "F");
        test_case("C", MinorSixth, "E");
        test_case("C", MajorSixth, "Eb");
        test_case("C", MinorSeventh, "D");
        test_case("C", MajorSeventh, "Db");
    }

    #[test]
    fn by_interval_descending_works_with_c_sharp() {
        test_case("C#", PerfectUnison, "C#");
        test_case("C#", MinorSecond, "B#");
        test_case("C#", MajorSecond, "B");
        test_case("C#", MinorThird, "A#");
        test_case("C#", MajorThird, "A");
        test_case("C#", PerfectFourth, "G#");
        test_case("C#", AugmentedFourth, "G");
        test_case("C#", DiminishedFifth, "F##");
        test_case("C#", PerfectFifth, "F#");
        test_case("C#", MinorSixth, "E#");
        test_case("C#", MajorSixth, "E");
        test_case("C#", MinorSeventh, "D#");
        test_case("C#", MajorSeventh, "D");
    }

    #[test]
    fn by_interval_descending_works_with_d_flat() {
        test_case("Db", PerfectUnison, "Db");
        test_case("Db", MinorSecond, "C");
        test_case("Db", MajorSecond, "Cb");
        test_case("Db", MinorThird, "Bb");
        test_case("Db", MajorThird, "Bbb");
        test_case("Db", PerfectFourth, "Ab");
        test_case("Db", AugmentedFourth, "Abb");
        test_case("Db", DiminishedFifth, "G");
        test_case("Db", PerfectFifth, "Gb");
        test_case("Db", MinorSixth, "F");
        test_case("Db", MajorSixth, "Fb");
        test_case("Db", MinorSeventh, "Eb");
        test_case("Db", MajorSeventh, "Ebb");
    }

    #[test]
    fn by_interval_descending_works_with_d_natural() {
        test_case("D", PerfectUnison, "D");
        test_case("D", MinorSecond, "C#");
        test_case("D", MajorSecond, "C");
        test_case("D", MinorThird, "B");
        test_case("D", MajorThird, "Bb");
        test_case("D", PerfectFourth, "A");
        test_case("D", AugmentedFourth, "Ab");
        test_case("D", DiminishedFifth, "G#");
        test_case("D", PerfectFifth, "G");
        test_case("D", MinorSixth, "F#");
        test_case("D", MajorSixth, "F");
        test_case("D", MinorSeventh, "E");
        test_case("D", MajorSeventh, "Eb");
    }

    #[test]
    fn by_interval_descending_works_with_d_sharp() {
        test_case("D#", PerfectUnison, "D#");
        test_case("D#", MinorSecond, "C##");
        test_case("D#", MajorSecond, "C#");
        test_case("D#", MinorThird, "B#");
        test_case("D#", MajorThird, "B");
        test_case("D#", PerfectFourth, "A#");
        test_case("D#", AugmentedFourth, "A");
        test_case("D#", DiminishedFifth, "G##");
        test_case("D#", PerfectFifth, "G#");
        test_case("D#", MinorSixth, "F##");
        test_case("D#", MajorSixth, "F#");
        test_case("D#", MinorSeventh, "E#");
        test_case("D#", MajorSeventh, "E");
    }

    #[test]
    fn by_interval_descending_works_with_e_flat() {
        test_case("Eb", PerfectUnison, "Eb");
        test_case("Eb", MinorSecond, "D");
        test_case("Eb", MajorSecond, "Db");
        test_case("Eb", MinorThird, "C");
        test_case("Eb", MajorThird, "Cb");
        test_case("Eb", PerfectFourth, "Bb");
        test_case("Eb", AugmentedFourth, "Bbb");
        test_case("Eb", DiminishedFifth, "A");
        test_case("Eb", PerfectFifth, "Ab");
        test_case("Eb", MinorSixth, "G");
        test_case("Eb", MajorSixth, "Gb");
        test_case("Eb", MinorSeventh, "F");
        test_case("Eb", MajorSeventh, "Fb");
    }

    #[test]
    fn by_interval_descending_works_with_e_natural() {
        test_case("E", PerfectUnison, "E");
        test_case("E", MinorSecond, "D#");
        test_case("E", MajorSecond, "D");
        test_case("E", MinorThird, "C#");
        test_case("E", MajorThird, "C");
        test_case("E", PerfectFourth, "B");
        test_case("E", AugmentedFourth, "Bb");
        test_case("E", DiminishedFifth, "A#");
        test_case("E", PerfectFifth, "A");
        test_case("E", MinorSixth, "G#");
        test_case("E", MajorSixth, "G");
        test_case("E", MinorSeventh, "F#");
        test_case("E", MajorSeventh, "F");
    }

    #[test]
    fn by_interval_descending_works_with_e_sharp() {
        test_case("E#", PerfectUnison, "E#");
        test_case("E#", MinorSecond, "D##");
        test_case("E#", MajorSecond, "D#");
        test_case("E#", MinorThird, "C##");
        test_case("E#", MajorThird, "C#");
        test_case("E#", PerfectFourth, "B#");
        test_case("E#", AugmentedFourth, "B");
        test_case("E#", DiminishedFifth, "A##");
        test_case("E#", PerfectFifth, "A#");
        test_case("E#", MinorSixth, "G##");
        test_case("E#", MajorSixth, "G#");
        test_case("E#", MinorSeventh, "F##");
        test_case("E#", MajorSeventh, "F#");
    }

    #[test]
    fn by_interval_descending_works_with_f_flat() {
        test_case("Fb", PerfectUnison, "Fb");
        test_case("Fb", MinorSecond, "Eb");
        test_case("Fb", MajorSecond, "Ebb");
        test_case("Fb", MinorThird, "Db");
        test_case("Fb", MajorThird, "Dbb");
        test_case("Fb", PerfectFourth, "Cb");
        test_case("Fb", AugmentedFourth, "Cbb");
        test_case("Fb", DiminishedFifth, "Bb");
        test_case("Fb", PerfectFifth, "Bbb");
        test_case("Fb", MinorSixth, "Ab");
        test_case("Fb", MajorSixth, "Abb");
        test_case("Fb", MinorSeventh, "Gb");
        test_case("Fb", MajorSeventh, "Gbb");
    }

    #[test]
    fn by_interval_descending_works_with_f_natural() {
        test_case("F", PerfectUnison, "F");
        test_case("F", MinorSecond, "E");
        test_case("F", MajorSecond, "Eb");
        test_case("F", MinorThird, "D");
        test_case("F", MajorThird, "Db");
        test_case("F", PerfectFourth, "C");
        test_case("F", AugmentedFourth, "Cb");
        test_case("F", DiminishedFifth, "B");
        test_case("F", PerfectFifth, "Bb");
        test_case("F", MinorSixth, "A");
        test_case("F", MajorSixth, "Ab");
        test_case("F", MinorSeventh, "G");
        test_case("F", MajorSeventh, "Gb");
    }

    #[test]
    fn by_interval_descending_works_with_f_sharp() {
        test_case("F#", PerfectUnison, "F#");
        test_case("F#", MinorSecond, "E#");
        test_case("F#", MajorSecond, "E");
        test_case("F#", MinorThird, "D#");
        test_case("F#", MajorThird, "D");
        test_case("F#", PerfectFourth, "C#");
        test_case("F#", AugmentedFourth, "C");
        test_case("F#", DiminishedFifth, "B#");
        test_case("F#", PerfectFifth, "B");
        test_case("F#", MinorSixth, "A#");
        test_case("F#", MajorSixth, "A");
        test_case("F#", MinorSeventh, "G#");
        test_case("F#", MajorSeventh, "G");
    }

    #[test]
    fn by_interval_descending_works_with_g_flat() {
        test_case("Gb", PerfectUnison, "Gb");
        test_case("Gb", MinorSecond, "F");
        test_case("Gb", MajorSecond, "Fb");
        test_case("Gb", MinorThird, "Eb");
        test_case("Gb", MajorThird, "Ebb");
        test_case("Gb", PerfectFourth, "Db");
        test_case("Gb", AugmentedFourth, "Dbb");
        test_case("Gb", DiminishedFifth, "C");
        test_case("Gb", PerfectFifth, "Cb");
        test_case("Gb", MinorSixth, "Bb");
        test_case("Gb", MajorSixth, "Bbb");
        test_case("Gb", MinorSeventh, "Ab");
        test_case("Gb", MajorSeventh, "Abb");
    }

    #[test]
    fn by_interval_descending_works_with_g_natural() {
        test_case("G", PerfectUnison, "G");
        test_case("G", MinorSecond, "F#");
        test_case("G", MajorSecond, "F");
        test_case("G", MinorThird, "E");
        test_case("G", MajorThird, "Eb");
        test_case("G", PerfectFourth, "D");
        test_case("G", AugmentedFourth, "Db");
        test_case("G", DiminishedFifth, "C#");
        test_case("G", PerfectFifth, "C");
        test_case("G", MinorSixth, "B");
        test_case("G", MajorSixth, "Bb");
        test_case("G", MinorSeventh, "A");
        test_case("G", MajorSeventh, "Ab");
    }

    #[test]
    fn by_interval_descending_works_with_g_sharp() {
        test_case("G#", PerfectUnison, "G#");
        test_case("G#", MinorSecond, "F##");
        test_case("G#", MajorSecond, "F#");
        test_case("G#", MinorThird, "E#");
        test_case("G#", MajorThird, "E");
        test_case("G#", PerfectFourth, "D#");
        test_case("G#", AugmentedFourth, "D");
        test_case("G#", DiminishedFifth, "C##");
        test_case("G#", PerfectFifth, "C#");
        test_case("G#", MinorSixth, "B#");
        test_case("G#", MajorSixth, "B");
        test_case("G#", MinorSeventh, "A#");
        test_case("G#", MajorSeventh, "A");
    }
}
