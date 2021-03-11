# Audiate

Audiate is a library that allows you to generate notes, chords, and scales. It utilizes the math inherent to music theory to create natural conjugations.

## Usage

... fill out with code snippets

## Roadmap

### MVP before first publish to crates.io

A list of items that need to be completed before the first publish to crates.io

Configure this as a general purpose library that can be exported to external projects like a CLI tool or web app.

- [ ] Note module
  - [x] instantiate new
  - [ ] interval method
  - [ ] getter methods
  - [ ] unit test for none values (private fns)
  - [ ] integration tests (pub fns like new)

- [ ] Chord Module
  - [ ] instantiate new
  - [ ] interval method (e.g., get fifth chord from root like G Major -> D Major)
  - [ ] notes method to get vec of notes
  - [ ] triads
  - [ ] sevenths

- [ ] Scale Module
  - [ ] new
  - [ ] Major
  - [ ] Minor
  - [ ] Modes

- [ ] Documentation
  - [ ] Note method documentation
  - [ ] Chord method documentation
  - [ ] Scale method documentation
  - [ ] Finalize explanatory comments, if any
  - [ ] Readme updates
  - [ ] Front page of crates.io with overview
 
### Road Map

An ongoing and incomplete checklist of new features to add to this library.

- [ ] Chord Module
  - [ ] ninths
  - [ ] elevenths
  - [ ] thirteenths
  - [ ] 1st inversions
  - [ ] 2nd inversions
  - [ ] 3rd inversions
  - [ ] suspensions

- [ ] Scale Module
  - [ ] Harmonic Minor
  - [ ] Melodic Minor
  - [ ] Whole Tone
  - [ ] Whole-Half
  - [ ] Half-Whole
