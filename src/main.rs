// Piano convention
#[derive(Debug)]
pub struct Note<'a> {
    name: &'a str, // name of the note
    repr: u8,      // integer representing the note
}
impl<'a> Note<'a> {
    pub fn init(name: &'a str, repr: u8) -> Self {
        Self { name, repr }
    }
    pub fn name(&self) -> &'a str {
        self.name
    }
    pub fn repr(&self) -> &u8 {
        &self.repr
    }
}
#[derive(Debug)]
pub struct Sound<'a> {
    note: Note<'a>,
    octave: u8,
}
impl<'a> Sound<'a> {
    pub fn init(note: Note<'a>, octave: u8) -> Self {
        Self { note, octave }
    }
    pub fn note(&self) -> &Note<'a> {
        &self.note
    }
    pub fn octave(&self) -> &u8 {
        &self.octave
    }
}

fn main() {
    use std::collections::HashMap;

    let fr_to_eng_note_names = HashMap::from([
        ("DO", "C"),
        ("RE", "D"),
        ("MI", "E"),
        ("FA", "F"),
        ("SOL", "G"),
        ("LA", "A"),
        ("SI", "B"),
    ]);
    let bemol = "♭";
    let sharp = "#";
    let name = "do";
    let repr = 0;
    let octave = 1;
    let note = Note::init(name, repr);
    // println!("{:?}", note);
    // println!("{:?}", note.name());
    // println!("{:?}", note.repr());
    let sound = Sound::init(note, octave);
    // println!("{:?}", sound);
    // println!("{:?}", sound.note());
    // println!("{:?}", sound.octave());
    println!("{:?}", fr_to_eng_note_names);
    println!("{:?}", bemol);
    println!("{:?}", sharp);
}
