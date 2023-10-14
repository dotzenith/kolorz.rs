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
//! // print kolored text
//! use kolorz::Kolor;
//!
//! fn main() {
//!     let mocha = Kolor::new("catppuccin mocha");
//!     println!("{}", mocha.red("This is red"));
//! }
//! ```
//!
//! ```rust
//! // get an array of all available kolorscheme names
//! use kolorz::Kolor;
//!
//! fn main() {
//!     let kolorschemes = Kolor::get_all_kolorschemes();
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
//! - gruvbox
//! - onedark
//! - tokyonight
//! - ayu
//! - palenight
//! - gogh
//!
//! ## The following colors are available on all of the kolorschemes:
//!
//! - red
//! - purple
//! - blue
//! - orange
//! - yellow
//! - text (usually white for dark kolorschemes)

#[derive(Clone, Copy)]
pub struct Kolor {
    red: (u8, u8, u8),
    purple: (u8, u8, u8),
    blue: (u8, u8, u8),
    green: (u8, u8, u8),
    orange: (u8, u8, u8),
    yellow: (u8, u8, u8),
    text: (u8, u8, u8),
}

impl Kolor {
    pub fn new(str: &str) -> crate::Kolor {
        match str {
            "catppuccin latte" => Kolor {
                red: (210, 15, 57),
                purple: (136, 57, 239),
                blue: (30, 102, 245),
                green: (64, 160, 43),
                orange: (254, 100, 11),
                yellow: (223, 142, 29),
                text: (204, 208, 218),
            },
            "catppuccin frappe" => Kolor {
                red: (231, 130, 132),
                purple: (202, 158, 230),
                blue: (140, 170, 238),
                green: (166, 209, 137),
                orange: (239, 159, 118),
                yellow: (229, 200, 144),
                text: (198, 208, 245),
            },
            "catppuccin macchiato" => Kolor {
                red: (237, 135, 150),
                purple: (198, 160, 246),
                blue: (138, 173, 244),
                green: (166, 218, 149),
                orange: (245, 169, 127),
                yellow: (238, 212, 159),
                text: (202, 211, 245),
            },
            "catppuccin mocha" => Kolor {
                red: (243, 139, 168),
                purple: (203, 166, 247),
                blue: (137, 180, 250),
                green: (166, 227, 161),
                orange: (250, 179, 135),
                yellow: (249, 226, 175),
                text: (205, 214, 244),
            },
            "dracula" => Kolor {
                red: (255, 85, 85),
                purple: (189, 147, 249),
                blue: (139, 233, 253),
                green: (80, 250, 123),
                orange: (255, 184, 108),
                yellow: (241, 250, 140),
                text: (248, 248, 242),
            },
            "nord" => Kolor {
                red: (191, 97, 106),
                purple: (180, 142, 173),
                blue: (136, 192, 208),
                green: (163, 190, 140),
                orange: (208, 135, 112),
                yellow: (235, 203, 139),
                text: (236, 239, 244),
            },
            "gruvbox" => Kolor {
                red: (251, 73, 52),
                purple: (211, 134, 155),
                blue: (131, 165, 152),
                green: (184, 187, 38),
                orange: (254, 128, 25),
                yellow: (250, 189, 47),
                text: (235, 219, 178),
            },
            "onedark" => Kolor {
                red: (224, 108, 117),
                purple: (198, 120, 221),
                blue: (97, 175, 239),
                green: (152, 195, 121),
                orange: (209, 154, 102),
                yellow: (229, 192, 123),
                text: (171, 178, 191),
            },
            "tokyonight" => Kolor {
                red: (247, 118, 142),
                purple: (173, 142, 230),
                blue: (122, 162, 247),
                green: (158, 206, 106),
                orange: (255, 158, 100),
                yellow: (224, 175, 104),
                text: (169, 177, 214),
            },
            "ayu" => Kolor {
                red: (255, 51, 51),
                purple: (212, 191, 255),
                blue: (115, 208, 255),
                green: (186, 230, 126),
                orange: (255, 167, 89),
                yellow: (255, 238, 153),
                text: (203, 204, 198),
            },
            "palenight" => Kolor {
                red: (255, 85, 114),
                purple: (199, 146, 234),
                blue: (96, 173, 236),
                green: (195, 232, 141),
                orange: (240, 113, 120),
                yellow: (255, 203, 107),
                text: (255, 254, 254),
            },
            "gogh" => Kolor {
                red: (255, 85, 114),
                purple: (245, 128, 255),
                blue: (117, 161, 255),
                green: (98, 222, 132),
                orange: (240, 113, 120),
                yellow: (255, 203, 107),
                text: (255, 254, 254),
            },
            _ => Kolor {
                red: (243, 139, 168),
                purple: (203, 166, 247),
                blue: (137, 180, 250),
                green: (166, 227, 161),
                orange: (250, 179, 135),
                yellow: (249, 226, 175),
                text: (205, 214, 244),
            },
        }
    }
    pub fn get_all_kolorschemes() -> [&'static str; 12] {
        [
            "catppuccin latte",
            "catppuccin frappe",
            "catppuccin macchiato",
            "catppuccin mocha",
            "dracula",
            "nord",
            "gruvbox",
            "onedark",
            "tokynight",
            "ayu",
            "palenight",
            "gogh",
        ]
    }
    fn kolorize(str: impl std::fmt::Display + Into<String>, colors: (u8, u8, u8))  -> String {
        format!("\x1b[38;2;{};{};{}m{}\x1b[0m", colors.0, colors.1, colors.2, str)
    }
    pub fn red(&self, str: impl std::fmt::Display + Into<String>) -> String {
        Self::kolorize(str, self.red)
    }
    pub fn purple(&self, str: impl std::fmt::Display + Into<String>) -> String {
        Self::kolorize(str, self.purple)
    }
    pub fn blue(&self, str: impl std::fmt::Display + Into<String>) -> String {
        Self::kolorize(str, self.blue)
    }
    pub fn green(&self, str: impl std::fmt::Display + Into<String>) -> String {
        Self::kolorize(str, self.green)
    }
    pub fn orange(&self, str: impl std::fmt::Display + Into<String>) -> String {
        Self::kolorize(str, self.orange)
    }
    pub fn yellow(&self, str: impl std::fmt::Display + Into<String>) -> String {
        Self::kolorize(str, self.yellow)
    }
    pub fn text(&self, str: impl std::fmt::Display + Into<String>) -> String {
        Self::kolorize(str, self.text)
    }
}
