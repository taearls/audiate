// use super::note::Note;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NotePitchInterval {
    PerfectUnison,
    MinorSecond,
    MajorSecond,
    AugmentedSecond,
    MinorThird,
    MajorThird,
    DiminishedFourth,
    PerfectFourth,
    AugmentedFourth,
    DiminishedFifth,
    PerfectFifth,
    AugmentedFifth,
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
            DiminishedFourth => AugmentedFifth,
            PerfectFourth => PerfectFifth,
            AugmentedFourth => DiminishedFifth,
            DiminishedFifth => AugmentedFourth,
            PerfectFifth => PerfectFourth,
            AugmentedFifth => DiminishedFourth,
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
        use NotePitchInterval::*;
        let pitch_interval_value: u8 = match other {
            PerfectUnison => 0,
            MinorSecond => 1,
            MajorSecond => 2,
            AugmentedSecond | MinorThird => 3,
            MajorThird | DiminishedFourth => 4,
            PerfectFourth => 5,
            AugmentedFourth | DiminishedFifth => 6,
            PerfectFifth => 7,
            AugmentedFifth | MinorSixth => 8,
            MajorSixth | DiminishedSeventh => 9,
            MinorSeventh => 10,
            MajorSeventh => 11,
        };
        let sum = self + pitch_interval_value;
        // we want only pitch values to be in the range of 0-11 inclusive
        sum % 12
    }
}
