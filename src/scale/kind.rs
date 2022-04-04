#![allow(dead_code)]
use crate::note::NotePitchInterval;

#[derive(Clone, Copy)]
pub enum ScaleKind {
    Ionian,
    Dorian,
    Phrygian,
    Lydian,
    Mixolydian,
    Aeolian,
    Locrian,
    Major,
    Minor,
    MajorPentatonic,
    MinorPentatonic,
    HarmonicMinor,
    MelodicMinor,
    HalfWhole,
    WholeHalf,
    WholeTone,
}

impl ScaleKind {
    fn intervals(&self, _descending: bool) -> Vec<NotePitchInterval> {
        use NotePitchInterval::*;
        use ScaleKind::*;

        match self {
            Ionian | Major => vec![
                MajorSecond,
                MajorSecond,
                MinorSecond,
                MajorSecond,
                MajorSecond,
                MajorSecond,
                MinorSecond,
            ],
            Dorian => vec![
                MajorSecond,
                MinorSecond,
                MajorSecond,
                MajorSecond,
                MajorSecond,
                MinorSecond,
                MajorSecond,
            ],
            Phrygian => vec![
                MinorSecond,
                MajorSecond,
                MajorSecond,
                MajorSecond,
                MinorSecond,
                MajorSecond,
                MajorSecond,
            ],
            Lydian => vec![
                MajorSecond,
                MajorSecond,
                MajorSecond,
                MinorSecond,
                MajorSecond,
                MajorSecond,
                MinorSecond,
            ],
            Mixolydian => vec![
                MajorSecond,
                MajorSecond,
                MinorSecond,
                MajorSecond,
                MajorSecond,
                MinorSecond,
                MajorSecond,
            ],
            Aeolian | Minor => vec![
                MajorSecond,
                MinorSecond,
                MajorSecond,
                MajorSecond,
                MinorSecond,
                MajorSecond,
                MajorSecond,
            ],
            Locrian => vec![
                MinorSecond,
                MajorSecond,
                MajorSecond,
                MinorSecond,
                MajorSecond,
                MajorSecond,
                MajorSecond,
            ],
            MajorPentatonic => vec![
                MajorSecond,
                MajorSecond,
                MinorThird,
                MajorSecond,
                MinorThird,
            ],
            MinorPentatonic => vec![
                MinorThird,
                MajorSecond,
                MajorSecond,
                MinorThird,
                MajorSecond,
            ],
            HarmonicMinor => vec![
                MajorSecond,
                MinorSecond,
                MajorSecond,
                MajorSecond,
                MinorSecond,
                MinorThird,
                MinorSecond,
            ],
            HalfWhole => vec![
                MinorSecond,
                MajorSecond,
                MinorSecond,
                MajorSecond,
                MinorSecond,
                MajorSecond,
                MinorSecond,
                MajorSecond,
            ],
            WholeHalf => vec![
                MajorSecond,
                MinorSecond,
                MajorSecond,
                MinorSecond,
                MajorSecond,
                MinorSecond,
                MajorSecond,
                MinorSecond,
            ],
            WholeTone => vec![MajorSecond; 6],
            // TODO: handle descending direction being different
            MelodicMinor => vec![
                MajorSecond,
                MinorSecond,
                MajorSecond,
                MajorSecond,
                MajorSecond,
                MajorSecond,
                MinorSecond,
            ],
        }
        // if descending {
        //     if self != MelodicMinor {
        //         result.rev()
        //     } else {
        //         vec![MinorSecond, MajorSecond, MinorSecond, MajorSecond, MinorSecond, MajorSecond, MinorSecond, MajorSecond]
        //     }
        // }
    }
}
