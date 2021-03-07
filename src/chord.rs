use crate::note::Note;

pub struct Chord {
  pub name: String,
  quality: ChordQuality,
  root: Note,
}

// the chord quality a chord can be.
// it describes the root triad in a chord.
pub enum ChordQuality {
  Major,
  Minor,
  Diminished,
  Augmented,
}

// how to denote dominant / major seventh ?
// how to denote suspended chords? 
// 

// describes the extensions of the chord.
// each value includes the previous one.
// e.g., Ninth includes Seventh and Triad.
// TODO: more semantic name to describe this enum
pub enum ChordExtensionKind {
  Triad,
  Seventh,
  Ninth,
  Eleventh,
  Thirteenth,
}

impl Chord {
  // pub fn new(root: &str, quality: ChordQuality) -> Result<Chord, &str> {


  //   // if !root.to_lowercase().matches(&['a', 'b', 'c', 'd', 'e', 'f', 'g'])
    
  //   // let root = match root.to_lowercase() {
  //   //   "a"
  //   // }
  //   // let name = get_chord_name(&root);

  //   // TODO: add fn for grabbing triad + seventh notes
  //   // let third = find_interval(&root, 4);
  //   // let fifth = find_interval(&root, 7);

  //   // Chord {
  //   //   name,
  //   //   quality,
  //   //   root
  //   // }
  // }
}

// fn validate_root(root: &str) -> Option<Note> {
//   if root.get(0)
//          .expect("root note cannot be empty string slice")
//          .matches(|ch| ch >= 'a' && ch <= 'g') {
//             Note::new(root)
//          }
//   else {
//     None
//   }
// }

// get the name of a chord. for now it just returns the name of the root note without any additional description. 
fn get_chord_name(root: &Note) -> String {
  String::from(&root.name)
}

// fn find_interval(root: &Note, semitones: i8) -> Note {
//   let name = String::from(&root.name);
//   let letter = root.letter;
//   // let pitch = root.pitch;
//   let variant = root.variant;
//   Note {
//     name,
//     letter,
//     variant,
//   }
// }