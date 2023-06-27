//! A silly little library for printing colored text to the terminal
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
//! // print colored text
//! use kolorz::get_colorscheme;
//!
//! fn main() {
//!     let mocha = get_colorscheme("catppuccin mocha");
//!     println!("{}", mocha.red("This is red"));
//! }
//! ```
//!
//! ```rust
//! // get an array of all available colorschemes
//! use kolorz::get_all_colorschemes;
//!
//! fn main() {
//!     let colorschemes = get_all_colorschemes();
//! }
//! ```
//!
//! ## The following colorschemes are available:
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
//! ## The following colors are available on all of the colorschemes:
//!
//! - red
//! - purple
//! - blue
//! - orange
//! - yellow
//! - white

pub fn get_all_colorschemes() -> [&'static str; 12] {
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

pub fn get_colorscheme(str: &str) -> crate::Color {
    match str {
        "catppuccin latte" => Color {
            red: (210, 15, 57),
            purple: (136, 57, 239),
            blue: (30, 102, 245),
            green: (64, 160, 43),
            orange: (254, 100, 11),
            yellow: (223, 142, 29),
            white: (204, 208, 218),
        },
        "catppuccin frappe" => Color {
            red: (231, 130, 132),
            purple: (202, 158, 230),
            blue: (140, 170, 238),
            green: (166, 209, 137),
            orange: (239, 159, 118),
            yellow: (229, 200, 144),
            white: (198, 208, 245),
        },
        "catppuccin macchiato" => Color {
            red: (237, 135, 150),
            purple: (198, 160, 246),
            blue: (138, 173, 244),
            green: (166, 218, 149),
            orange: (245, 169, 127),
            yellow: (238, 212, 159),
            white: (202, 211, 245),
        },
        "catppuccin mocha" => Color {
            red: (243, 139, 168),
            purple: (203, 166, 247),
            blue: (137, 180, 250),
            green: (166, 227, 161),
            orange: (250, 179, 135),
            yellow: (249, 226, 175),
            white: (205, 214, 244),
        },
        "dracula" => Color {
            red: (255, 85, 85),
            purple: (189, 147, 249),
            blue: (139, 233, 253),
            green: (80, 250, 123),
            orange: (255, 184, 108),
            yellow: (241, 250, 140),
            white: (248, 248, 242),
        },
        "nord" => Color {
            red: (191, 97, 106),
            purple: (180, 142, 173),
            blue: (136, 192, 208),
            green: (163, 190, 140),
            orange: (208, 135, 112),
            yellow: (235, 203, 139),
            white: (236, 239, 244),
        },
        "gruvbox" => Color {
            red: (251, 73, 52),
            purple: (211, 134, 155),
            blue: (131, 165, 152),
            green: (184, 187, 38),
            orange: (254, 128, 25),
            yellow: (250, 189, 47),
            white: (235, 219, 178),
        },
        "onedark" => Color {
            red: (224, 108, 117),
            purple: (198, 120, 221),
            blue: (97, 175, 239),
            green: (152, 195, 121),
            orange: (209, 154, 102),
            yellow: (229, 192, 123),
            white: (171, 178, 191),
        },
        "tokyonight" => Color {
            red: (247, 118, 142),
            purple: (173, 142, 230),
            blue: (122, 162, 247),
            green: (158, 206, 106),
            orange: (255, 158, 100),
            yellow: (224, 175, 104),
            white: (169, 177, 214),
        },
        "ayu" => Color {
            red: (255, 51, 51),
            purple: (212, 191, 255),
            blue: (115, 208, 255),
            green: (186, 230, 126),
            orange: (255, 167, 89),
            yellow: (255, 238, 153),
            white: (203, 204, 198),
        },
        "palenight" => Color {
            red: (255, 85, 114),
            purple: (199, 146, 234),
            blue: (96, 173, 236),
            green: (195, 232, 141),
            orange: (240, 113, 120),
            yellow: (255, 203, 107),
            white: (255, 254, 254),
        },
        "gogh" => Color {
            red: (255, 85, 114),
            purple: (245, 128, 255),
            blue: (117, 161, 255),
            green: (98, 222, 132),
            orange: (240, 113, 120),
            yellow: (255, 203, 107),
            white: (255, 254, 254),
        },
        _ => Color {
            red: (243, 139, 168),
            purple: (203, 166, 247),
            blue: (137, 180, 250),
            green: (166, 227, 161),
            orange: (250, 179, 135),
            yellow: (249, 226, 175),
            white: (205, 214, 244),
        },
    }
}

#[derive(Clone, Copy)]
pub struct Color {
    red: (u8, u8, u8),
    purple: (u8, u8, u8),
    blue: (u8, u8, u8),
    green: (u8, u8, u8),
    orange: (u8, u8, u8),
    yellow: (u8, u8, u8),
    white: (u8, u8, u8),
}

impl Color {
    pub fn red(&self, str: impl std::fmt::Display + Into<String>) -> String {
        return format!(
            "\x1b[38;2;{};{};{}m{}\x1b[0m",
            self.red.0, self.red.1, self.red.2, str
        );
    }
    pub fn purple(&self, str: impl std::fmt::Display + Into<String>) -> String {
        return format!(
            "\x1b[38;2;{};{};{}m{}\x1b[0m",
            self.purple.0, self.purple.1, self.purple.2, str
        );
    }
    pub fn blue(&self, str: impl std::fmt::Display + Into<String>) -> String {
        return format!(
            "\x1b[38;2;{};{};{}m{}\x1b[0m",
            self.blue.0, self.blue.1, self.blue.2, str
        );
    }
    pub fn green(&self, str: impl std::fmt::Display + Into<String>) -> String {
        return format!(
            "\x1b[38;2;{};{};{}m{}\x1b[0m",
            self.green.0, self.green.1, self.green.2, str
        );
    }
    pub fn orange(&self, str: impl std::fmt::Display + Into<String>) -> String {
        return format!(
            "\x1b[38;2;{};{};{}m{}\x1b[0m",
            self.orange.0, self.orange.1, self.orange.2, str
        );
    }
    pub fn yellow(&self, str: impl std::fmt::Display + Into<String>) -> String {
        return format!(
            "\x1b[38;2;{};{};{}m{}\x1b[0m",
            self.yellow.0, self.yellow.1, self.yellow.2, str
        );
    }
    pub fn white(&self, str: impl std::fmt::Display + Into<String>) -> String {
        return format!(
            "\x1b[38;2;{};{};{}m{}\x1b[0m",
            self.white.0, self.white.1, self.white.2, str
        );
    }
}
