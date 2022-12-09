use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RandomnessFromStrErr {
    InvalidInputLength {
        /// Input length in bytes
        n: usize,
    },
    InvalidHexCharacter {
        c: char,
        index: usize,
    },
}

impl fmt::Display for RandomnessFromStrErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RandomnessFromStrErr::InvalidInputLength { n } => {
                write!(
                    f,
                    "Expected 64 hex characters but got an input of {n} bytes"
                )
            }
            RandomnessFromStrErr::InvalidHexCharacter { c, index } => {
                write!(f, "Invalid character {:?} at position {}", c, index)
            }
        }
    }
}

/// Takes a hex string and decodes it. Input must be 64 hex characters long (32 bytes).
pub fn randomness_from_str(input: impl AsRef<str>) -> Result<[u8; 32], RandomnessFromStrErr> {
    let input = input.as_ref();
    if input.len() != 64 {
        return Err(RandomnessFromStrErr::InvalidInputLength { n: input.len() });
    }

    let mut out: [u8; 32] = Default::default();
    hex::decode_to_slice(input, &mut out).map_err(|err| match err {
        hex::FromHexError::InvalidHexCharacter { c, index } => {
            RandomnessFromStrErr::InvalidHexCharacter { c, index }
        }
        // Those two cases cannot happen as long as we check the input length independently
        hex::FromHexError::OddLength => unreachable!(),
        hex::FromHexError::InvalidStringLength => unreachable!(),
    })?;
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn randomness_from_str_works() {
        let r =
            randomness_from_str("0000000000000000000000000000000000000000000000000000000000000000")
                .unwrap();
        assert_eq!(r, [0u8; 32]);
        let r =
            randomness_from_str("0101010101010101010101010101010101010101010101010101010101010101")
                .unwrap();
        assert_eq!(r, [1u8; 32]);

        // node
        // Uint8Array.from(Buffer.from("9e8e26615f51552aa3b18b6f0bcf0dae5afbe30321e8d7ea7fa51ebeb1d8fe62", "hex"))
        let r =
            randomness_from_str("9e8e26615f51552aa3b18b6f0bcf0dae5afbe30321e8d7ea7fa51ebeb1d8fe62")
                .unwrap();
        assert_eq!(
            r,
            [
                158, 142, 38, 97, 95, 81, 85, 42, 163, 177, 139, 111, 11, 207, 13, 174, 90, 251,
                227, 3, 33, 232, 215, 234, 127, 165, 30, 190, 177, 216, 254, 98
            ]
        );

        // wrong input length (30 bytes)
        let err =
            randomness_from_str("26615f51552aa3b18b6f0bcf0dae5afbe30321e8d7ea7fa51ebeb1d8fe62")
                .unwrap_err();
        assert_eq!(
            err.to_string(),
            "Expected 64 hex characters but got an input of 60 bytes"
        );

        // wrong input length and characters
        let err = randomness_from_str("whatever 🤷‍♂️").unwrap_err();
        assert_eq!(
            err.to_string(),
            "Expected 64 hex characters but got an input of 22 bytes"
        );
    }
}
