use std::io::Error;

use rust_arpeggiator::{Note, Chord, ChordQuality, ChordKind};

fn main() -> Result<(), Error> {
    let root_note = Note::new("C");
    let chord = Chord::new(root_note, ChordQuality::Major, ChordKind::Triad);
    println!("Chord name is {}", chord.name);
    Ok(())
}
