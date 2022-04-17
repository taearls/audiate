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
        use NotePitchInterval::*;
        let pitch_interval_value: u8 = match other {
            PerfectUnison => 0,
            MinorSecond => 1,
            MajorSecond => 2,
            MinorThird | AugmentedSecond => 3,
            MajorThird => 4,
            PerfectFourth => 5,
            AugmentedFourth | DiminishedFifth => 6,
            PerfectFifth => 7,
            MinorSixth => 8,
            MajorSixth | DiminishedSeventh => 9,
            MinorSeventh => 10,
            MajorSeventh => 11,
        };
        let sum = self + pitch_interval_value;
        // pitch values are 0-11 inclusive
        sum % 12
    }
}
