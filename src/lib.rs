struct Chord {
  name: String,
  root: Note,
  third: Note,
  fifth: Note,
  seventh: Note,
}

struct Note {
  letter_name: String,
  pitch: MusicNote, 
  variant: PitchVariant,
}

// use this value to calculate the pitch from a letter
enum PitchVariant {
  flatdbl = -2,
  flat = -1,
  natural = 0,
  sharp = 1,
  sharpdbl = 2,
}


//  Because most songs are in the key of C,
//  that is the lowest value in this enum that everything else is based around.
//  The pitch values are relative to C being the root.
// only use natural letter values to measure pitch. 
// variants can describe changes from the root.
// 
// to get Db, subtract 1 from D.
// to get D#, add 1 to D.
enum MusicNote {
  C = 1,
  D = 3,
  E = 5,
  F = 6,
  G = 8,
  A = 10,
  B = 12,
}

impl Note {
  // fn pitch(&mut self) -> u8 {
  //   self.pitch = self.variant + MusicNote::A::value
  // }
  // Note.pitch -> letter_name + variant
// use letter_name to get Music Note, add
//   get_major_third(&Self note) -> Note {
//     Note {
//       pitch: note.pitch + 4,

//     }
    
//   },
//   get_variant(&Self note) -> PitchVariant {

//   }
//   pub fn arpeggiate(Note note, ) -> Chord {
  
//   }
}
