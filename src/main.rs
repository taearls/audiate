use std::io::Error;

// TODO: come up with better names for chord quality and chord kind
// expose the least amount of this as possible
use rust_arpeggiator::{Note, NotePitchVariant, Chord, ChordQuality, ChordExtensionKind};

fn main() -> Result<(), Error> {
    let root_note = Note::new("C", NotePitchVariant::Natural);


    let chord = Chord::new(root_note, ChordQuality::Major, Some(ChordExtensionKind::Triad));
    println!("Chord name is {}", chord.name);

    Ok(())
}
