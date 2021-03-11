use std::io::Error;

mod chord;
mod note;

fn main() -> Result<(), Error> {
    // let chord = Chord
    //     ::new(root_note, ChordQuality::Major)
    //     .with_extension(ChordExtensionKind::Triad);

    println!("successful build!");
    Ok(())
}
