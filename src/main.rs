use std::io::Error;

// TODO: come up with better names for chord quality and chord kind
// expose the least amount of this as possible
use rust_arpeggiator::{Note, Chord, ChordQuality, ChordKind};

fn main() -> Result<(), Error> {
    let root_note = Note::new("C");
    let chord = Chord::new(root_note, ChordQuality::Major, ChordKind::Triad);
    println!("Chord name is {}", chord.name);
    Ok(())
}
