//! A silly little library for printing kolored text to the terminal
//!
//! ## Installation
//!
//! Add kolorz to your project's `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
#![doc = concat!("kolorz = \"", env!("CARGO_PKG_VERSION"), "\"")]
//! ```
//!
//! ## Basic Usage
//!
//! ```rust
//! use kolorz::Kolor;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let mocha = Kolor::new("catppuccin mocha")?;
//!     println!("{}", mocha.red("This is red"));
//!     Ok(())
//! }
//! ```
//!
//! ## The following kolorschemes are available:
//!
//! - catppuccin latte
//! - catppuccin frappe
//! - catppuccin macchiato
//! - catppuccin mocha
//! - dracula
//! - nord
//! - gruvbox dark
//! - gruvbox light
//! - onedark
//! - tokyonight
//! - ayu
//! - palenight
//! - gogh
//! - biscuit dark
//! - biscuit light
//!
//! ## The following colors are available on all of the kolorschemes:
//!
//! - red           (0)
//! - purple        (1)
//! - blue          (2)
//! - green         (3)
//! - orange        (4)
//! - yellow        (5)
//! - text          (6)
//! - random (picks a random color from above)
//! - numbered (allows the user to pick a kolor by number)
//!
//! ## Kustom Kolorz are also available
//!
//! ```rust
//! use kolorz::HexKolorize;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     println!("{}", "This is peach".kolorize("#fab387")?);
//!     Ok(())
//! }
//! ```
//!
//! ```rust
//! // custom kolorz from RGB
//! use kolorz::RGBKolorize;
//!
//! fn main() {
//!     println!("{}", "This is red".kolorize((235, 160, 172)));
//! }
//! ```

pub mod errors;
pub mod hex;
pub mod rgb;

pub use errors::{InvalidColorNumberError, InvalidHexCodeError, UnknownKolorSchemeError};
pub use hex::HexKolorize;
use rand::Rng;
pub use rgb::RGBKolorize;
use std::fmt::Display;
use std::io::IsTerminal;

/// Determines if colors should be used based on the CLICOLORS spec.
///
/// Priority (highest to lowest):
/// 1. `NO_COLOR` env var - if set, colors are disabled
/// 2. `CLICOLOR_FORCE` env var - if set, colors are always enabled
/// 3. `CLICOLOR` env var / default - colors enabled only if stdout is a TTY
///
/// See <https://bixense.com/clicolors/> for the full specification.
pub fn colors_enabled() -> bool {
    if std::env::var_os("NO_COLOR").is_some() {
        return false;
    }

    if std::env::var_os("CLICOLOR_FORCE").is_some() {
        return true;
    }

    std::io::stdout().is_terminal()
}

pub type RGB = (u8, u8, u8);

#[derive(Clone, Copy, Debug)]
pub struct Kolor {
    red: RGB,
    purple: RGB,
    blue: RGB,
    green: RGB,
    orange: RGB,
    yellow: RGB,
    text: RGB,
}

#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub enum KolorScheme {
    CatppuccinLatte,
    CatppuccinFrappe,
    CatppuccinMacchiato,
    CatppuccinMocha,
    Dracula,
    Nord,
    GruvboxDark,
    GruvboxLight,
    OneDark,
    TokyoNight,
    Ayu,
    PaleNight,
    Gogh,
    BiscuitDark,
    BiscuitLight,
}

impl Default for KolorScheme {
    fn default() -> Self {
        Self::CatppuccinMocha
    }
}

impl TryFrom<&str> for KolorScheme {
    type Error = UnknownKolorSchemeError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "catppuccin latte" => Ok(Self::CatppuccinLatte),
            "catppuccin frappe" => Ok(Self::CatppuccinFrappe),
            "catppuccin macchiato" => Ok(Self::CatppuccinMacchiato),
            "catppuccin mocha" => Ok(Self::CatppuccinMocha),
            "dracula" => Ok(Self::Dracula),
            "nord" => Ok(Self::Nord),
            "gruvbox dark" => Ok(Self::GruvboxDark),
            "gruvbox light" => Ok(Self::GruvboxLight),
            "onedark" => Ok(Self::OneDark),
            "tokyonight" => Ok(Self::TokyoNight),
            "ayu" => Ok(Self::Ayu),
            "palenight" => Ok(Self::PaleNight),
            "gogh" => Ok(Self::Gogh),
            "biscuit dark" => Ok(Self::BiscuitDark),
            "biscuit light" => Ok(Self::BiscuitLight),
            _ => Err(UnknownKolorSchemeError::new(s)),
        }
    }
}

impl From<KolorScheme> for Kolor {
    fn from(kolorscheme: KolorScheme) -> Self {
        match kolorscheme {
            KolorScheme::CatppuccinLatte => Kolor {
                red: (210, 15, 57),
                purple: (136, 57, 239),
                blue: (30, 102, 245),
                green: (64, 160, 43),
                orange: (254, 100, 11),
                yellow: (223, 142, 29),
                text: (204, 208, 218),
            },
            KolorScheme::CatppuccinFrappe => Kolor {
                red: (231, 130, 132),
                purple: (202, 158, 230),
                blue: (140, 170, 238),
                green: (166, 209, 137),
                orange: (239, 159, 118),
                yellow: (229, 200, 144),
                text: (198, 208, 245),
            },
            KolorScheme::CatppuccinMacchiato => Kolor {
                red: (237, 135, 150),
                purple: (198, 160, 246),
                blue: (138, 173, 244),
                green: (166, 218, 149),
                orange: (245, 169, 127),
                yellow: (238, 212, 159),
                text: (202, 211, 245),
            },
            KolorScheme::CatppuccinMocha => Kolor {
                red: (243, 139, 168),
                purple: (203, 166, 247),
                blue: (137, 180, 250),
                green: (166, 227, 161),
                orange: (250, 179, 135),
                yellow: (249, 226, 175),
                text: (205, 214, 244),
            },
            KolorScheme::Dracula => Kolor {
                red: (255, 85, 85),
                purple: (189, 147, 249),
                blue: (139, 233, 253),
                green: (80, 250, 123),
                orange: (255, 184, 108),
                yellow: (241, 250, 140),
                text: (248, 248, 242),
            },
            KolorScheme::Nord => Kolor {
                red: (191, 97, 106),
                purple: (180, 142, 173),
                blue: (136, 192, 208),
                green: (163, 190, 140),
                orange: (208, 135, 112),
                yellow: (235, 203, 139),
                text: (236, 239, 244),
            },
            KolorScheme::GruvboxDark => Kolor {
                red: (251, 73, 52),
                purple: (211, 134, 155),
                blue: (131, 165, 152),
                green: (184, 187, 38),
                orange: (254, 128, 25),
                yellow: (250, 189, 47),
                text: (235, 219, 178),
            },
            KolorScheme::GruvboxLight => Kolor {
                red: (204, 36, 29),
                purple: (211, 134, 155),
                blue: (69, 133, 136),
                green: (184, 187, 38),
                orange: (254, 128, 25),
                yellow: (215, 153, 33),
                text: (60, 56, 54),
            },
            KolorScheme::OneDark => Kolor {
                red: (224, 108, 117),
                purple: (198, 120, 221),
                blue: (97, 175, 239),
                green: (152, 195, 121),
                orange: (209, 154, 102),
                yellow: (229, 192, 123),
                text: (171, 178, 191),
            },
            KolorScheme::TokyoNight => Kolor {
                red: (247, 118, 142),
                purple: (173, 142, 230),
                blue: (122, 162, 247),
                green: (158, 206, 106),
                orange: (255, 158, 100),
                yellow: (224, 175, 104),
                text: (169, 177, 214),
            },
            KolorScheme::Ayu => Kolor {
                red: (255, 51, 51),
                purple: (212, 191, 255),
                blue: (115, 208, 255),
                green: (186, 230, 126),
                orange: (255, 167, 89),
                yellow: (255, 238, 153),
                text: (203, 204, 198),
            },
            KolorScheme::PaleNight => Kolor {
                red: (255, 85, 114),
                purple: (199, 146, 234),
                blue: (96, 173, 236),
                green: (195, 232, 141),
                orange: (240, 113, 120),
                yellow: (255, 203, 107),
                text: (255, 254, 254),
            },
            KolorScheme::Gogh => Kolor {
                red: (255, 85, 114),
                purple: (245, 128, 255),
                blue: (117, 161, 255),
                green: (98, 222, 132),
                orange: (240, 113, 120),
                yellow: (255, 203, 107),
                text: (255, 254, 254),
            },
            KolorScheme::BiscuitDark => Kolor {
                red: (207, 34, 56),
                purple: (123, 61, 121),
                blue: (124, 138, 126),
                green: (150, 143, 107),
                orange: (240, 104, 66),
                yellow: (227, 137, 69),
                text: (255, 233, 199),
            },
            KolorScheme::BiscuitLight => Kolor {
                red: (174, 71, 80),
                purple: (131, 84, 107),
                blue: (122, 127, 127),
                green: (151, 145, 111),
                orange: (198, 105, 93),
                yellow: (205, 145, 101),
                text: (255, 233, 199),
            },
        }
    }
}

impl Kolor {
    pub fn new<T: TryInto<KolorScheme>>(scheme: T) -> Result<Self, T::Error> {
        Ok(Kolor::from(scheme.try_into()?))
    }
    pub fn kolorize(text: impl std::fmt::Display, color: RGB) -> KoloredText {
        KoloredText::new(text.to_string(), color)
    }
    pub fn random(&self, text: impl std::fmt::Display) -> KoloredText {
        let mut rng = rand::thread_rng();

        match rng.gen_range(0..=6) as usize {
            0 => self.red(text),
            1 => self.purple(text),
            2 => self.blue(text),
            3 => self.green(text),
            4 => self.orange(text),
            5 => self.yellow(text),
            6 => self.text(text),
            _ => unreachable!(),
        }
    }

    pub fn numbered(
        &self,
        text: impl std::fmt::Display,
        num: usize,
    ) -> Result<KoloredText, InvalidColorNumberError> {
        match num {
            0 => Ok(self.red(text)),
            1 => Ok(self.purple(text)),
            2 => Ok(self.blue(text)),
            3 => Ok(self.green(text)),
            4 => Ok(self.orange(text)),
            5 => Ok(self.yellow(text)),
            6 => Ok(self.text(text)),
            _ => Err(InvalidColorNumberError::new(num)),
        }
    }
    pub fn red(&self, text: impl std::fmt::Display) -> KoloredText {
        Self::kolorize(text, self.red)
    }
    pub fn purple(&self, text: impl std::fmt::Display) -> KoloredText {
        Self::kolorize(text, self.purple)
    }
    pub fn blue(&self, text: impl std::fmt::Display) -> KoloredText {
        Self::kolorize(text, self.blue)
    }
    pub fn green(&self, text: impl std::fmt::Display) -> KoloredText {
        Self::kolorize(text, self.green)
    }
    pub fn orange(&self, text: impl std::fmt::Display) -> KoloredText {
        Self::kolorize(text, self.orange)
    }
    pub fn yellow(&self, text: impl std::fmt::Display) -> KoloredText {
        Self::kolorize(text, self.yellow)
    }
    pub fn text(&self, text: impl std::fmt::Display) -> KoloredText {
        Self::kolorize(text, self.text)
    }
}

#[derive(Clone, Debug)]
pub struct KoloredText {
    text: String,
    color: RGB,
}

impl KoloredText {
    pub fn new(text: String, color: RGB) -> Self {
        KoloredText { text, color }
    }
}

impl Display for KoloredText {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if colors_enabled() {
            // Write prefix, text, suffix separately to avoid formatting issues
            write!(
                f,
                "\x1b[38;2;{};{};{}m",
                self.color.0, self.color.1, self.color.2
            )?;
            self.text.fmt(f)?;
            f.write_str("\x1b[0m")
        } else {
            self.text.fmt(f)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod kolor_scheme {
        use super::*;

        #[test]
        fn parse_valid_schemes() {
            assert!(KolorScheme::try_from("catppuccin mocha").is_ok());
            assert!(KolorScheme::try_from("catppuccin latte").is_ok());
            assert!(KolorScheme::try_from("catppuccin frappe").is_ok());
            assert!(KolorScheme::try_from("catppuccin macchiato").is_ok());
            assert!(KolorScheme::try_from("dracula").is_ok());
            assert!(KolorScheme::try_from("nord").is_ok());
            assert!(KolorScheme::try_from("gruvbox dark").is_ok());
            assert!(KolorScheme::try_from("gruvbox light").is_ok());
            assert!(KolorScheme::try_from("onedark").is_ok());
            assert!(KolorScheme::try_from("tokyonight").is_ok());
            assert!(KolorScheme::try_from("ayu").is_ok());
            assert!(KolorScheme::try_from("palenight").is_ok());
            assert!(KolorScheme::try_from("gogh").is_ok());
            assert!(KolorScheme::try_from("biscuit dark").is_ok());
            assert!(KolorScheme::try_from("biscuit light").is_ok());
        }

        #[test]
        fn parse_invalid_scheme() {
            let err = KolorScheme::try_from("not-a-scheme").unwrap_err();
            assert_eq!(err.name(), "not-a-scheme");
        }

        #[test]
        fn default_scheme() {
            assert!(matches!(
                KolorScheme::default(),
                KolorScheme::CatppuccinMocha
            ));
        }
    }

    mod kolor {
        use super::*;

        #[test]
        fn new_with_valid_str() {
            assert!(Kolor::new("catppuccin mocha").is_ok());
            assert!(Kolor::new("dracula").is_ok());
        }

        #[test]
        fn new_with_invalid_str() {
            assert!(Kolor::new("invalid").is_err());
        }

        #[test]
        fn new_with_enum() {
            assert!(Kolor::new(KolorScheme::Dracula).is_ok());
            assert!(Kolor::new(KolorScheme::Nord).is_ok());
        }

        #[test]
        fn numbered_valid_range() {
            let kolor = Kolor::new("dracula").unwrap();
            for i in 0..=6 {
                assert!(kolor.numbered("test", i).is_ok());
            }
        }

        #[test]
        fn numbered_out_of_range() {
            let kolor = Kolor::new("dracula").unwrap();
            let err = kolor.numbered("test", 7).unwrap_err();
            assert_eq!(err.number(), 7);

            let err = kolor.numbered("test", 100).unwrap_err();
            assert_eq!(err.number(), 100);
        }

        #[test]
        fn random_returns_kolored_text() {
            unsafe { std::env::set_var("CLICOLOR_FORCE", "1") };
            let kolor = Kolor::new("dracula").unwrap();
            let output = kolor.random("test").to_string();
            assert!(output.starts_with("\x1b[38;2;"));
            assert!(output.ends_with("\x1b[0m"));
        }
    }

    mod kolored_text {
        use super::*;

        #[test]
        fn format_includes_ansi_codes() {
            unsafe { std::env::set_var("CLICOLOR_FORCE", "1") };
            let kolor = Kolor::new("catppuccin mocha").unwrap();
            let output = kolor.red("hello").to_string();

            assert!(output.starts_with("\x1b[38;2;"));
            assert!(output.ends_with("\x1b[0m"));
            assert!(output.contains("hello"));
        }

        #[test]
        fn kolorize_with_rgb_values() {
            unsafe { std::env::set_var("CLICOLOR_FORCE", "1") };
            let output = Kolor::kolorize("test", (255, 128, 0)).to_string();
            assert!(output.contains("255;128;0"));
        }

        #[test]
        fn each_color_method_works() {
            unsafe { std::env::set_var("CLICOLOR_FORCE", "1") };
            let kolor = Kolor::new("dracula").unwrap();

            assert!(kolor.red("x").to_string().contains("255;85;85"));
            assert!(kolor.purple("x").to_string().contains("189;147;249"));
            assert!(kolor.blue("x").to_string().contains("139;233;253"));
            assert!(kolor.green("x").to_string().contains("80;250;123"));
            assert!(kolor.orange("x").to_string().contains("255;184;108"));
            assert!(kolor.yellow("x").to_string().contains("241;250;140"));
            assert!(kolor.text("x").to_string().contains("248;248;242"));
        }

        #[test]
        fn no_color_disables_ansi() {
            unsafe { std::env::set_var("NO_COLOR", "1") };
            unsafe { std::env::remove_var("CLICOLOR_FORCE") };
            let kolor = Kolor::new("dracula").unwrap();
            let output = kolor.red("hello").to_string();

            assert_eq!(output, "hello");
            assert!(!output.contains("\x1b["));
            unsafe { std::env::remove_var("NO_COLOR") };
        }
    }
}
