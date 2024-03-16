#[cfg(test)]
mod unit_test;
use crate::{Note, Sound};

#[derive(Debug)]
pub enum ScaleType {
    Major,
    Minor,
}

#[derive(Debug)]
pub struct Scale {
    note: Note,
    scale_type: ScaleType,
}

impl Default for Scale {
    fn default() -> Self {
        Self {
            note: Note::C,
            scale_type: ScaleType::Major,
        }
    }
}

impl Scale {
    pub fn init(note: Note, scale_type: ScaleType) -> Self {
        Self { note, scale_type }
    }

    pub fn notes(&self) -> Vec<usize> {
        match self.scale_type {
            ScaleType::Major => self.note.major_scale_from_tonic(),
            ScaleType::Minor => self.note.minor_scale_from_tonic(),
        }
    }

    fn get_sound(tonic: Sound, current_note: Note) -> Sound {
        let mut current_sound = Sound::init(current_note, tonic.octave());
        if tonic > current_sound {
            current_sound = Sound::init(current_note, tonic.octave() + 1);
        }
        current_sound
    }

    pub fn sounds(&self, octave: usize) -> Vec<Sound> {
        let notes = self.notes();
        let tonic = Sound::init(self.note, octave);
        let mut sounds = Vec::with_capacity(notes.len());
        sounds.push(tonic);
        for i in 1..notes.len() {
            let note = Note::from_usize(notes[i]);
            let sound = Self::get_sound(tonic, note);
            sounds.push(sound);
        }
        sounds
    }
}
