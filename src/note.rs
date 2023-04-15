use anyhow::bail;

#[derive(PartialEq, Eq, Clone)]
pub struct Note {
    midi: u8,
}

impl Default for Note {
    fn default() -> Self {
        Self { midi: 60 }
    }
}

impl From<Note> for String {
    fn from(value: Note) -> Self {
        musical_scales::Pitch::from_midi_note(value.midi).to_string()
    }
}

impl TryFrom<&str> for Note {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let Ok(pitch) = musical_scales::Pitch::try_from(value) else { bail!("Unable to parse") };

        let midi = pitch.to_midi();

        Ok(Self { midi })
    }
}
