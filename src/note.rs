use anyhow::bail;
use musical_scales::{Pitch, PitchClass};

#[derive(PartialEq, Eq, Clone)]
pub struct Note {
    midi: u8,
}

impl Default for Note {
    fn default() -> Self {
        Self { midi: 60 }
    }
}

impl Note {
    pub fn get_pitch_class_offset(&self) -> i8 {
        PitchClass::from_midi_note(self.midi).semitone_offset()
    }

    pub fn get_octave(&self) -> i8 {
        i8::try_from(self.midi).expect(
            "self.midi shouldn't be > 127, which happens to mean this should always be fine",
        ) / 12
            - 1
    }

    pub fn from_midi(midi: u8) -> Self {
        Self { midi }
    }

    pub fn new(pitch_class: u8, octave: i8) -> Self {
        let pitch_class = PitchClass::from_midi_note(pitch_class);

        Pitch::new(pitch_class, octave).into()
    }
}

impl From<Note> for String {
    fn from(value: Note) -> Self {
        Pitch::from(value).to_string()
    }
}

impl From<Note> for musical_scales::Pitch {
    fn from(value: Note) -> Self {
        Self::from_midi_note(value.midi)
    }
}

impl From<musical_scales::Pitch> for Note {
    fn from(value: musical_scales::Pitch) -> Self {
        Self {
            midi: value.to_midi(),
        }
    }
}

impl TryFrom<&str> for Note {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let Ok(pitch) = musical_scales::Pitch::try_from(value) else { bail!("Unable to parse") };

        Ok(pitch.into())
    }
}
