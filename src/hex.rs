use crate::{Kolor, KoloredText};

pub trait HexKolorize {
    fn kolorize(&self, color: &str) -> KoloredText;
}

impl<T> HexKolorize for T where T: std::fmt::Display + Into<String>  {
    fn kolorize(&self, color: &str) -> KoloredText {
        let hex = color.replace("#", "").to_lowercase();

        let hex_num = usize::from_str_radix(&hex, 16).unwrap();
        let kolor = (
            (hex_num >> 16) as u8,
            ((hex_num >> 8) & 0x00FF) as u8,
            (hex_num & 0x0000_00FF) as u8,
        );
        Kolor::kolorize(self.to_string(), kolor)
    }
}
