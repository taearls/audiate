// use super::note::Note;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NotePitchInterval {
    PerfectUnison,
    MinorSecond,
    MajorSecond,
    AugmentedSecond,
    MinorThird,
    MajorThird,
    PerfectFourth,
    AugmentedFourth,
    DiminishedFifth,
    PerfectFifth,
    MinorSixth,
    MajorSixth,
    DiminishedSeventh,
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
            AugmentedSecond => DiminishedSeventh,
            MinorThird => MajorSixth,
            MajorThird => MinorSixth,
            PerfectFourth => PerfectFifth,
            AugmentedFourth => DiminishedFifth,
            DiminishedFifth => AugmentedFourth,
            PerfectFifth => PerfectFourth,
            MinorSixth => MajorThird,
            MajorSixth => MinorThird,
            DiminishedSeventh => AugmentedSecond,
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
            NotePitchInterval::MinorThird | NotePitchInterval::AugmentedSecond => 3,
            NotePitchInterval::MajorThird => 4,
            NotePitchInterval::PerfectFourth => 5,
            NotePitchInterval::AugmentedFourth | NotePitchInterval::DiminishedFifth => 6,
            NotePitchInterval::PerfectFifth => 7,
            NotePitchInterval::MinorSixth => 8,
            NotePitchInterval::MajorSixth | NotePitchInterval::DiminishedSeventh => 9,
            NotePitchInterval::MinorSeventh => 10,
            NotePitchInterval::MajorSeventh => 11,
        };
        let sum = self + pitch_interval_value;
        // we want only pitch values to be in the range of 0-11 inclusive
        sum % 12
    }
}
