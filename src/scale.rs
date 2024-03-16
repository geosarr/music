use crate::Note;

pub enum ScaleType {
    Major,
    Minor,
}

pub struct Scale {
    note: Note,
    scale_type: ScaleType,
}

impl Scale {
    pub fn init(note: Note, scale_type: ScaleType) -> Self {
        Self { note, scale_type }
    }
    pub fn scale_notes(&self) -> Vec<usize> {
        match self.scale_type {
            ScaleType::Major => self.note.major_scale_from_tonic(),
            ScaleType::Minor => self.note.minor_scale_from_tonic(),
        }
    }
}
