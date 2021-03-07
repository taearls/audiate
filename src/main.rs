use std::io::Error;
// #[macro_use]
// extern crate lazy_static;

// TODO: come up with better names for chord quality and chord kind
// expose the least amount of this as possible
mod chord;
use chord::{Chord, ChordQuality, ChordExtensionKind};

mod note;
// use note::{Note, NotePitchVariant};

fn main() -> Result<(), Error> {
    // should this be private?
    // let root_note = "C"; 

    // let chord = Chord
    //     ::new(root_note, ChordQuality::Major)
    //     .with_extension(ChordExtensionKind::Triad); 
    
    // println!("Chord name is {}", chord.name);
    println!("successful build!");
    Ok(())
}
