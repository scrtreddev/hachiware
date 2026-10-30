//! Melodies for a passive piezo buzzer.
//!
//! Pure data, like [`crate::blink::BlinkPattern`]: the firmware turns each
//! [`Note`] into a PWM square wave, and host tests can check the tables.

/// One note of a monophonic melody. `hz == 0` is a rest.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Note {
    pub hz: u16,
    pub ms: u16,
}

impl Note {
    pub const fn is_rest(self) -> bool {
        self.hz == 0
    }
}

/// Shorthand that keeps the tables below readable.
const fn n(hz: u16, ms: u16) -> Note {
    Note { hz, ms }
}

// Equal-tempered pitches (A4 = 440 Hz), rounded to the nearest hertz. Only
// the ones the tables use; `FS` is F sharp.
pub const REST: u16 = 0;
pub const D5: u16 = 587;
pub const E5: u16 = 659;
pub const FS5: u16 = 740;
pub const G5: u16 = 784;
pub const A5: u16 = 880;
pub const B5: u16 = 988;

/// Total length of a melody, in milliseconds.
pub const fn duration_ms(melody: &[Note]) -> u32 {
    let mut total = 0;
    let mut i = 0;
    while i < melody.len() {
        total += melody[i].ms as u32;
        i += 1;
    }
    total
}

/// Transcribed from `morning_exercise.mid`.
///
/// Generated, not hand-written: the loud melody line only (the soft
/// accompaniment is all velocity < 50 and a buzzer plays one note at a time),
/// the 6 s of leading silence dropped, timings at the file's implicit 120 bpm.
/// Each note runs until the next one starts; gaps of more than ~60 ms are
/// kept as rests.
///
/// Two hand edits on top: ~19 s in, a near-zero-length note in the file (a
/// recording glitch) left an 815 ms hole, filled with the A5s the same phrase
/// has everywhere else; and it ends on the D5 held ~2.4 s at ~57 s, cutting
/// the ~9 s the file plays after it.
pub const MORNING_EXERCISE: &[Note] = &[
    n(A5, 279),
    n(A5, 533),
    n(B5, 291),
    n(A5, 267),
    n(FS5, 268),
    n(A5, 547),
    n(A5, 267),
    n(A5, 546),
    n(B5, 267),
    n(A5, 279),
    n(FS5, 268),
    n(A5, 453),
    n(REST, 93),
    n(D5, 268),
    n(D5, 278),
    n(D5, 267),
    n(E5, 279),
    n(FS5, 802),
    n(D5, 268),
    n(FS5, 824),
    n(A5, 278),
    n(A5, 1081),
    n(D5, 279),
    n(D5, 267),
    n(D5, 267),
    n(E5, 257),
    n(FS5, 1103),
    n(D5, 267),
    n(D5, 278),
    n(D5, 257),
    n(E5, 278),
    n(FS5, 986),
    n(REST, 105),
    n(E5, 267),
    n(E5, 279),
    n(E5, 268),
    n(D5, 268),
    n(E5, 546),
    n(FS5, 406),
    n(REST, 139),
    n(A5, 396),
    n(REST, 140),
    n(G5, 465),
    n(REST, 81),
    n(FS5, 429),
    n(REST, 117),
    n(E5, 546),
    n(A5, 256),
    n(A5, 546),
    n(B5, 279),
    n(A5, 278),
    n(FS5, 268),
    n(A5, 545),
    n(A5, 257),
    n(A5, 546),
    n(B5, 278),
    n(A5, 268),
    n(FS5, 278),
    n(A5, 454),
    n(REST, 93),
    n(D5, 267),
    n(D5, 268),
    n(D5, 267),
    n(E5, 279),
    n(FS5, 814),
    n(D5, 267),
    n(FS5, 814),
    n(A5, 278),
    n(A5, 1081),
    n(D5, 267),
    n(D5, 291),
    n(D5, 267),
    n(E5, 268),
    n(FS5, 1092),
    n(D5, 268),
    n(D5, 267),
    n(D5, 268),
    n(E5, 278),
    n(FS5, 976),
    n(REST, 105),
    n(E5, 267),
    n(E5, 279),
    n(E5, 278),
    n(D5, 257),
    n(E5, 545),
    n(FS5, 395),
    n(REST, 151),
    n(A5, 395),
    n(REST, 151),
    n(G5, 466),
    n(REST, 70),
    n(FS5, 418),
    n(REST, 128),
    n(E5, 557),
    n(A5, 267),
    n(A5, 524),
    n(B5, 278),
    n(A5, 279),
    n(FS5, 278),
    n(A5, 534),
    n(A5, 279),
    n(A5, 523),
    n(B5, 279),
    n(A5, 278),
    n(FS5, 268),
    n(A5, 465),
    n(REST, 93),
    n(D5, 267),
    n(D5, 257),
    n(D5, 278),
    n(E5, 267),
    n(FS5, 825),
    n(D5, 267),
    n(FS5, 814),
    n(A5, 268),
    n(A5, 1093),
    n(D5, 267),
    n(D5, 279),
    n(D5, 267),
    n(E5, 267),
    n(FS5, 1093),
    n(D5, 268),
    n(D5, 267),
    n(D5, 278),
    n(E5, 268),
    n(FS5, 965),
    n(REST, 128),
    n(E5, 267),
    n(E5, 267),
    n(E5, 279),
    n(D5, 267),
    n(E5, 535),
    n(FS5, 418),
    n(REST, 140),
    n(A5, 395),
    n(REST, 140),
    n(G5, 476),
    n(REST, 70),
    n(FS5, 419),
    n(REST, 128),
    n(E5, 546),
    n(A5, 267),
    n(A5, 546),
    n(B5, 278),
    n(A5, 257),
    n(FS5, 278),
    n(A5, 534),
    n(A5, 291),
    n(A5, 533),
    n(B5, 257),
    n(A5, 278),
    n(FS5, 267),
    n(E5, 557),
    n(D5, 2370),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starts_on_a_note() {
        assert!(!MORNING_EXERCISE[0].is_rest());
    }

    #[test]
    fn runs_about_a_minute() {
        let ms = duration_ms(MORNING_EXERCISE);
        assert!((58_000..60_000).contains(&ms), "{ms} ms");
    }
}
