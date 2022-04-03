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
        use NotePitchInterval::*;
        match self {
            PerfectUnison => PerfectUnison,
            MinorSecond => MajorSeventh,
            MajorSecond => MinorSeventh,
            MinorThird => MajorSixth,
            MajorThird => MinorSixth,
            PerfectFourth => PerfectFifth,
            AugmentedFourth => DiminishedFifth,
            DiminishedFifth => AugmentedFourth,
            PerfectFifth => PerfectFourth,
            MinorSixth => MajorThird,
            MajorSixth => MinorThird,
            MinorSeventh => MajorSecond,
            MajorSeventh => MinorSecond,
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
        use NotePitchInterval::*;
        let pitch_interval_value: u8 = match self {
            PerfectUnison => 0,
            MinorSecond => 1,
            MajorSecond => 2,
            MinorThird => 3,
            MajorThird => 4,
            PerfectFourth => 5,
            AugmentedFourth | DiminishedFifth => 6,
            PerfectFifth => 7,
            MinorSixth => 8,
            MajorSixth => 9,
            MinorSeventh => 10,
            MajorSeventh => 11,
        };
        let sum = pitch_interval_value + other;
        // we want only pitch values to be in the range of 0-11 inclusive
        match sum % 12 {
            1 => MinorSecond,
            2 => MajorSecond,
            3 => MinorThird,
            4 => MajorThird,
            5 => PerfectFourth,
            6 => {
                // if adding to AugmentedFourth, return that; if adding to DiminishedFifth, return that
                self
            }
            7 => PerfectFifth,
            8 => MinorSixth,
            9 => MajorSixth,
            10 => MinorSeventh,
            11 => MajorSeventh,
            _ => PerfectUnison,
        }
    }
}
