#![allow(dead_code)]
use crate::note::NotePitchInterval;

use super::ScaleDirection;

#[derive(Clone, Copy, PartialEq)]
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
    pub fn intervals(&self, direction: ScaleDirection) -> Vec<NotePitchInterval> {
        use NotePitchInterval::*;
        use ScaleDirection::*;
        use ScaleKind::*;

        let mut result = match self {
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
        };
        match direction {
            Descending | DescendingAscending => {
                result = match self {
                    MelodicMinor => Aeolian.intervals(Descending),
                    _ => result.into_iter().rev().collect(),
                }
            }
            _ => {}
        }

        result
    }
}
