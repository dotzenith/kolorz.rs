use crate::{Kolor, KoloredText, RGB};

pub trait RGBKolorize {
    fn kolorize(&self, color: RGB) -> KoloredText;
}

impl<T> RGBKolorize for T
where
    T: std::fmt::Display + Into<String>,
{
    fn kolorize(&self, color: RGB) -> KoloredText {
        Kolor::kolorize(self.to_string(), color)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rgb_kolorize() {
        let output = "test".kolorize((100, 150, 200)).to_string();
        assert!(output.contains("100;150;200"));
    }

    #[test]
    fn rgb_min_values() {
        let output = "t".kolorize((0, 0, 0)).to_string();
        assert!(output.contains("0;0;0"));
    }

    #[test]
    fn rgb_max_values() {
        let output = "t".kolorize((255, 255, 255)).to_string();
        assert!(output.contains("255;255;255"));
    }

    #[test]
    fn rgb_mixed_values() {
        let output = "t".kolorize((255, 0, 128)).to_string();
        assert!(output.contains("255;0;128"));
    }

    #[test]
    fn rgb_output_format() {
        let output = "hello".kolorize((10, 20, 30)).to_string();
        assert!(output.starts_with("\x1b[38;2;"));
        assert!(output.ends_with("\x1b[0m"));
        assert!(output.contains("hello"));
    }
}
