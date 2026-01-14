use crate::{Kolor, KoloredText, RGB};

pub trait RGBKolorize {
    fn kolorize(&self, color: RGB) -> KoloredText;
}

impl<T> RGBKolorize for T where T: std::fmt::Display + Into<String>  {
    fn kolorize(&self, color: RGB) -> KoloredText {
        Kolor::kolorize(self.to_string(), color)
    }
}
