#[derive(Debug)]
pub enum Error {
    InvalidIntervalChar(char),
    InvalidTonic(String),
    IntervalExtendsTooFar,
    IntervalTooLong,
    IntervalDidNotWrap,
}

const MAJOR_SCALE: &[&str] = &[
    "A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#",
];
const MINOR_SCALE: &[&str] = &[
    "A", "Bb", "B", "C", "Db", "D", "Eb", "E", "F", "Gb", "G", "Ab",
];

const MAJOR_TONICS: &[&str] = &[
    "C", "G", "D", "A", "E", "B", "F#", "e", "b", "f#", "c#", "g#", "d#", "a",
];
const MINOR_TONICS: &[&str] = &[
    "F", "Bb", "Eb", "Ab", "Db", "Gb", "d", "g", "c", "f", "bb", "eb",
];

pub struct Scale {
    scale: Vec<String>,
}

impl Scale {
    pub fn new(tonic: &str, intervals: &str) -> Result<Scale, Error> {
        let mut tonic_note = tonic.to_string();
        if let Some(first) = tonic_note.get_mut(0..1) {
            first.make_ascii_uppercase();
        }

        let skips = intervals
            .chars()
            .map(|step| match step {
                'm' => Ok(0),
                'M' => Ok(1),
                'A' => Ok(2),
                c => Err(Error::InvalidIntervalChar(c)),
            })
            .collect::<Result<Vec<_>, Error>>()?;
        let num_skips = skips.len();
        let mut skip_iter = skips.into_iter();

        let scale_slice = match tonic {
            tonic if MAJOR_TONICS.contains(&tonic) => Ok(MAJOR_SCALE),
            tonic if MINOR_TONICS.contains(&tonic) => Ok(MINOR_SCALE),
            tonic => Err(Error::InvalidTonic(tonic.to_string())),
        }?;
        let mut scale_iter = std::iter::repeat(scale_slice.iter())
            .take(2)
            .flatten()
            .map(|note| note.to_string())
            .skip_while(|note| *note != tonic_note);

        let mut scale = Vec::new();
        while scale.len() < num_skips {
            let note = scale_iter.next().ok_or(Error::IntervalExtendsTooFar)?;
            scale.push(note);

            let skip = skip_iter.next().ok_or(Error::IntervalTooLong)?;
            for _ in 0..skip {
                let _ = scale_iter.next();
            }
        }
        if scale_iter.next() != Some(tonic_note) {
            return Err(Error::IntervalDidNotWrap);
        }

        Ok(Self { scale })
    }

    pub fn chromatic(tonic: &str) -> Result<Scale, Error> {
        Self::new(tonic, "mmmmmmmmmmmm")
    }

    pub fn enumerate(&self) -> Vec<String> {
        self.scale.clone()
    }
}
