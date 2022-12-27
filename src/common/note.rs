use std::{fmt::Display, str::FromStr};

use lazy_static::lazy_static;
use regex::Regex;
use serde::Deserialize;

#[derive(Deserialize, PartialEq, Eq, Hash, Clone, Copy)]
#[serde(try_from = "String")]
pub(crate) struct Pitch {
    note: Note,
    octave: i8,
}

impl Pitch {
    pub(crate) fn from_midi(x: u8) -> Self {
        // Determine octave
        let octave: i8 = ((x / 12) - 1).try_into().unwrap();

        // Determine note in octave
        let note: Note = match x % 12 {
            0 => Note::C,
            1 => Note::Cs,
            2 => Note::D,
            3 => Note::Ds,
            4 => Note::E,
            5 => Note::F,
            6 => Note::Fs,
            7 => Note::G,
            8 => Note::Gs,
            9 => Note::A,
            10 => Note::As,
            11 => Note::B,
            _ => {
                panic!("For any integer x, x%12 should be from 0-11 inclusive. If you're seeing this, that didn't happen.");
            }
        };

        Pitch { note, octave }
    }
}

#[cfg(test)]
mod test {
    use super::Pitch;

    #[test]
    fn pitch_from_midi() {
        let midi_note: u8 = 60;
        let pitch = Pitch::from_midi(midi_note);

        assert_eq!(pitch.to_string(), "C4");
    }
}

impl Default for Pitch {
    fn default() -> Self {
        Self {
            note: Note::default(),
            octave: 4,
        }
    }
}

impl Display for Pitch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.note, self.octave)
    }
}

impl TryFrom<String> for Pitch {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^[ABCDEFG]{1}[#b]{0,1}[0-9]$").unwrap();
        }

        if RE.is_match(&value) {
            let chars: Vec<char> = value.chars().collect();

            let mut note: Note = match chars.first().unwrap() {
                'A' => Some(Note::A),
                'B' => Some(Note::B),
                'C' => Some(Note::C),
                'D' => Some(Note::D),
                'E' => Some(Note::E),
                'F' => Some(Note::F),
                'G' => Some(Note::G),
                _ => None,
            }
            .unwrap();

            let accidental: Option<char>;
            let mut octave: i8;

            match chars.len() {
                3 => {
                    // 3 chars means we have an accidental
                    accidental = chars.get(1).copied();
                    octave = match chars.get(2).unwrap().clone().to_string().parse() {
                        Ok(x) => Ok(x),
                        Err(_) => Err("couldn't parse octave to i8"),
                    }?;
                }
                2 => {
                    // No accidental
                    accidental = None;
                    octave = match chars.get(1).unwrap().clone().to_string().parse() {
                        Ok(x) => Ok(x),
                        Err(_) => Err("couldn't parse octave to i8"),
                    }?;
                }
                _ => {
                    return Err("invalid input".into());
                }
            }

            if let Some(x) = accidental {
                match x {
                    '#' => {
                        // Sharp
                        if note == Note::B {
                            octave += 1;
                        }
                        note = Note::sharpen(note);
                    }
                    'b' => {
                        // Flat
                        if note == Note::C {
                            octave -= 1;
                        }
                        note = Note::flatten(note);
                    }
                    _ => {
                        return Err("invalid accidental".into());
                    }
                }
            }

            return Ok(Pitch { note, octave });
        }

        Err("couldn't parse".into())
    }
}

impl FromStr for Pitch {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Pitch::try_from(s.to_string())
    }
}

impl From<Pitch> for String {
    fn from(val: Pitch) -> Self {
        format!("{}{}", val.note, val.octave)
    }
}

#[derive(PartialEq, Eq, Hash, Default, Clone, Copy)]
pub(crate) enum Note {
    #[default]
    C,
    Cs,
    D,
    Ds,
    E,
    F,
    Fs,
    G,
    Gs,
    A,
    As,
    B,
}

impl Note {
    fn sharpen(v: Self) -> Self {
        use Note::{As, Cs, Ds, Fs, Gs, A, B, C, D, E, F, G};

        match v {
            Note::C => Cs,
            Note::Cs => D,
            Note::D => Ds,
            Note::Ds => E,
            Note::E => F,
            Note::F => Fs,
            Note::Fs => G,
            Note::G => Gs,
            Note::Gs => A,
            Note::A => As,
            Note::As => B,
            Note::B => C,
        }
    }

    fn flatten(v: Self) -> Self {
        use Note::{As, Cs, Ds, Fs, Gs, A, B, C, D, E, F, G};

        match v {
            Note::C => B,
            Note::Cs => C,
            Note::D => Cs,
            Note::Ds => D,
            Note::E => Ds,
            Note::F => E,
            Note::Fs => F,
            Note::G => Fs,
            Note::Gs => G,
            Note::A => Gs,
            Note::As => A,
            Note::B => As,
        }
    }
}

impl From<Note> for String {
    fn from(val: Note) -> Self {
        match val {
            Note::C => "C",
            Note::Cs => "C#",
            Note::D => "D",
            Note::Ds => "D#",
            Note::E => "E",
            Note::F => "F",
            Note::Fs => "F#",
            Note::G => "G",
            Note::Gs => "G#",
            Note::A => "A",
            Note::As => "A#",
            Note::B => "B",
        }
        .into()
    }
}

impl Display for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from(*self))
    }
}
