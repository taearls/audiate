# Audiate

Audiate is a library that allows you to generate notes, chords, and scales. It utilizes the math inherent to music theory to calculate notes with enharmonic correctness.

## Usage

This library and its public API are under development and subject to change. I'm building a minimal feature set towards an eventual 0.0.1 release. More details in the Roadmap section below.

### Chords
Generate C major, minor, diminished, and augmented chords from a root C note, and then read the root, third, and fifth of the chord.

```rust
use audiate::{
    chord::{Chord, ChordQuality},
    Note,
};
let root_note = Note::try_from("C").unwrap();

let major_chord = Chord::new(root_note, ChordQuality::Major);
assert_eq!(root_note, major_chord.root());
assert_eq!(Note::try_from("E").unwrap(), major_chord.third());
assert_eq!(Note::try_from("G").unwrap(), major_chord.fifth());

let minor_chord = Chord::new(root_note, ChordQuality::Minor);
assert_eq!(root_note, minor_chord.root());
assert_eq!(Note::try_from("Eb").unwrap(), minor_chord.third());
assert_eq!(Note::try_from("G").unwrap(), minor_chord.fifth());

let diminished_chord = Chord::new(root_note, ChordQuality::Diminished);
assert_eq!(root_note, diminished_chord.root());
assert_eq!(Note::try_from("Eb").unwrap(), diminished_chord.third());
assert_eq!(Note::try_from("Gb").unwrap(), diminished_chord.fifth());

let augmented_chord = Chord::new(root_note, ChordQuality::Augmented);
assert_eq!(root_note, augmented_chord.root());
assert_eq!(Note::try_from("E").unwrap(), augmented_chord.third());
assert_eq!(Note::try_from("G#").unwrap(), augmented_chord.fifth());
```

### Scales
Generate scales from a root C note, and then print the notes in the scale.

```rust
use audiate::{
    scale::{Scale, ScaleDirection, ScaleKind},
    Note,
};
let root_note = Note::try_from("C").unwrap();

let major_scale = Scale::new(root_note, ScaleKind::Major, ScaleDirection::Ascending);
assert_eq!(String::from("C D E F G A B C"), major_scale.print());

let minor_scale = Scale::new(root_note, ScaleKind::Minor, ScaleDirection::Ascending);
assert_eq!(String::from("C D Eb F G Ab Bb C"), minor_scale.print());

let harmonic_minor_scale = Scale::new(
    root_note,
    ScaleKind::HarmonicMinor,
    ScaleDirection::Ascending,
);
assert_eq!(
    String::from("C D Eb F G Ab B C"),
    harmonic_minor_scale.print()
);

let major_pentatonic_scale = Scale::new(
    root_note,
    ScaleKind::MajorPentatonic,
    ScaleDirection::Ascending,
);
assert_eq!(String::from("C D E G A C"), major_pentatonic_scale.print());
```

## Roadmap

### MVP before first publish to crates.io

A list of items that need to be completed before the first publish to crates.io

- [x] Note module
    - [x] instantiate new
    - [x] getter methods
    - [x] interval method
    - [x] Unit tests

- [x] Scale Module
    - [x] Major
    - [x] Minor
    - [x] Modes
    - [x] Harmonic Minor
    - [x] Melodic Minor
    - [x] Pentatonic Scale
    - [x] Unit tests

- [ ] Chord Module
    - [x] instantiate new
    - [x] getter methods
    - [x] major, minor, diminished, augmented triads
    - [ ] major + minor sevenths
    - [ ] Unit tests

- [ ] Documentation
    - [ ] High level overview
    - [ ] Note method documentation
    - [ ] Scale method documentation
    - [ ] Chord method documentation
    - [ ] Documentation Tests
    - [ ] Finalize explanatory comments, if any
    - [ ] Readme updates
    - [ ] Front page of crates.io with overview
 
### Future Improvements

An ongoing and incomplete checklist of new features to add to this library after publishing a 0.0.1 release to crates.io.

- [ ] Note Module
    - [ ] Multiple Octaves
    - [ ] Rhythms

- [ ] Chord Module
    - [ ] ninths
    - [ ] elevenths
    - [ ] thirteenths
    - [ ] 1st inversions
    - [ ] 2nd inversions
    - [ ] 3rd inversions
    - [ ] suspensions

- [ ] Scale Module
    - [ ] Chromatic Scale
    - [ ] Whole Tone
    - [ ] Half Whole
    - [ ] Whole Half
    - [ ] Multiple Octaves
    - [ ] Non-Western Scales
