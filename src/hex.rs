use crate::errors::InvalidHexCodeError;
use crate::{Kolor, KoloredText};

pub trait HexKolorize {
    fn kolorize(&self, color: &str) -> Result<KoloredText, InvalidHexCodeError>;
}

impl<T> HexKolorize for T
where
    T: std::fmt::Display,
{
    fn kolorize(&self, color: &str) -> Result<KoloredText, InvalidHexCodeError> {
        let hex = color.replace("#", "").to_lowercase();

        let hex_num =
            usize::from_str_radix(&hex, 16).map_err(|_| InvalidHexCodeError::new(color))?;
        let kolor = (
            (hex_num >> 16) as u8,
            ((hex_num >> 8) & 0x00FF) as u8,
            (hex_num & 0x0000_00FF) as u8,
        );
        Ok(Kolor::kolorize(self.to_string(), kolor))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_with_hash() {
        assert!("text".kolorize("#ff5500").is_ok());
    }

    #[test]
    fn hex_without_hash() {
        assert!("text".kolorize("ff5500").is_ok());
    }

    #[test]
    fn hex_lowercase() {
        assert!("text".kolorize("#aabbcc").is_ok());
    }

    #[test]
    fn hex_uppercase() {
        assert!("text".kolorize("#AABBCC").is_ok());
    }

    #[test]
    fn hex_mixed_case() {
        assert!("text".kolorize("#AaBbCc").is_ok());
    }

    #[test]
    fn hex_invalid_characters() {
        let err = "text".kolorize("gggggg").unwrap_err();
        assert_eq!(err.code(), "gggggg");
    }

    #[test]
    fn hex_invalid_with_hash() {
        let err = "text".kolorize("#nothex").unwrap_err();
        assert_eq!(err.code(), "#nothex");
    }

    #[test]
    fn hex_produces_correct_rgb() {
        let output = "x".kolorize("#ff8000").unwrap().to_string();
        assert!(output.contains("255;128;0"));
    }

    #[test]
    fn hex_black() {
        let output = "x".kolorize("#000000").unwrap().to_string();
        assert!(output.contains("0;0;0"));
    }

    #[test]
    fn hex_white() {
        let output = "x".kolorize("#ffffff").unwrap().to_string();
        assert!(output.contains("255;255;255"));
    }
}
