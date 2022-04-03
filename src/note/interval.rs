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
