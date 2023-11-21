use crate::{Kolor, KoloredText};

pub trait RGBKolorize {
    fn kolorize(&self, color: (u8, u8, u8)) -> KoloredText;
}

impl<T> RGBKolorize for T where T: std::fmt::Display + Into<String>  {
    fn kolorize(&self, color: (u8, u8, u8)) -> KoloredText {
        Kolor::kolorize(self.to_string(), color)
    }
}
