# Audiate

Audiate is a library that allows you to generate notes, chords, and scales. It utilizes the math inherent to music theory to calculate notes with enharmonic correctness.

## Usage

This library and its public API are under development and subject to change. I'm building a minimal feature set towards an eventual 0.0.1 release. More details in the Roadmap section below.

```rust
// main.rs
use audiate::{Note, Chord, ChordQuality};

fn main() {
    let root_note = Note::try_from("C").unwrap();
    let chord = Chord::new(root_note, ChordQuality::Major);

    // arpeggiate the chord, starting from the root note, C.
    assert_eq!(root_note, chord.root());
    assert_eq!(Note::try_from("E").unwrap(), chord.third());
    assert_eq!(Note::try_from("G").unwrap(), chord.fifth());
}
```

## Roadmap

### MVP before first publish to crates.io

A list of items that need to be completed before the first publish to crates.io

- [x] Configure this as library, not binary application

- [x] Note module
    - [x] instantiate new
    - [x] getter methods
    - [x] interval method
    - [x] unit tests

- [x] Scale Module
    - [x] Major
    - [x] Minor
    - [x] Modes
    - [x] Harmonic Minor
    - [x] Melodic Minor
    - [x] Whole Tone
    - [x] Half Whole
    - [x] Whole Half
    - [x] Chromatic
    - [x] Pentatonic Scale
    - [x] unit tests

- [ ] Chord Module
    - [x] instantiate new
    - [x] getter methods
    - [x] major, minor, diminished, augmented triads
    - [ ] major + minor sevenths
    - [ ] unit tests

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
    - [ ] Multiple Octaves
    - [ ] Non-Western Scales
