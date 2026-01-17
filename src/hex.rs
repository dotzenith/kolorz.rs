use crate::{Kolor, KoloredText};

pub trait HexKolorize {
    fn kolorize(&self, color: &str) -> KoloredText;
}

impl<T> HexKolorize for T
where
    T: std::fmt::Display,
{
    fn kolorize(&self, color: &str) -> KoloredText {
        let hex = color.replace("#", "").to_lowercase();

        let Some(hex_num) = usize::from_str_radix(&hex, 16).ok() else {
            return KoloredText::uncolored(self.to_string());
        };
        let kolor = (
            (hex_num >> 16) as u8,
            ((hex_num >> 8) & 0x00FF) as u8,
            (hex_num & 0x0000_00FF) as u8,
        );
        Kolor::kolorize(self.to_string(), kolor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_with_hash() {
        unsafe { std::env::set_var("CLICOLOR_FORCE", "1") };
        let output = "text".kolorize("#ff5500").to_string();
        assert!(output.contains("255;85;0"));
    }

    #[test]
    fn hex_without_hash() {
        unsafe { std::env::set_var("CLICOLOR_FORCE", "1") };
        let output = "text".kolorize("ff5500").to_string();
        assert!(output.contains("255;85;0"));
    }

    #[test]
    fn hex_lowercase() {
        unsafe { std::env::set_var("CLICOLOR_FORCE", "1") };
        let output = "text".kolorize("#aabbcc").to_string();
        assert!(output.contains("170;187;204"));
    }

    #[test]
    fn hex_uppercase() {
        unsafe { std::env::set_var("CLICOLOR_FORCE", "1") };
        let output = "text".kolorize("#AABBCC").to_string();
        assert!(output.contains("170;187;204"));
    }

    #[test]
    fn hex_mixed_case() {
        unsafe { std::env::set_var("CLICOLOR_FORCE", "1") };
        let output = "text".kolorize("#AaBbCc").to_string();
        assert!(output.contains("170;187;204"));
    }

    #[test]
    fn hex_invalid_returns_uncolored() {
        unsafe { std::env::set_var("CLICOLOR_FORCE", "1") };
        let output = "text".kolorize("gggggg").to_string();
        assert_eq!(output, "text");
    }

    #[test]
    fn hex_invalid_with_hash_returns_uncolored() {
        unsafe { std::env::set_var("CLICOLOR_FORCE", "1") };
        let output = "text".kolorize("#nothex").to_string();
        assert_eq!(output, "text");
    }

    #[test]
    fn hex_produces_correct_rgb() {
        unsafe { std::env::set_var("CLICOLOR_FORCE", "1") };
        let output = "x".kolorize("#ff8000").to_string();
        assert!(output.contains("255;128;0"));
    }

    #[test]
    fn hex_black() {
        unsafe { std::env::set_var("CLICOLOR_FORCE", "1") };
        let output = "x".kolorize("#000000").to_string();
        assert!(output.contains("0;0;0"));
    }

    #[test]
    fn hex_white() {
        unsafe { std::env::set_var("CLICOLOR_FORCE", "1") };
        let output = "x".kolorize("#ffffff").to_string();
        assert!(output.contains("255;255;255"));
    }
}
