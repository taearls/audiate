use std::io::Error;


// TODO: come up with better names for chord quality and chord kind
// expose the least amount of this as possible
mod chord;
use chord::{Chord, ChordQuality, ChordExtensionKind};

mod note;
use note::{Note, NotePitchVariant};

fn main() -> Result<(), Error> {
    let root_note = Note::new("C", NotePitchVariant::Natural);


    let chord = Chord::new(root_note, ChordQuality::Major, Some(ChordExtensionKind::Triad));
    println!("Chord name is {}", chord.name);

    Ok(())
}
