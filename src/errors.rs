use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnknownKolorSchemeError {
    name: String,
}

impl UnknownKolorSchemeError {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl fmt::Display for UnknownKolorSchemeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unknown kolor scheme: '{}'", self.name)
    }
}

impl std::error::Error for UnknownKolorSchemeError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvalidHexCodeError {
    code: String,
}

impl InvalidHexCodeError {
    pub fn new(code: impl Into<String>) -> Self {
        Self { code: code.into() }
    }

    /// Returns the invalid hex code
    pub fn code(&self) -> &str {
        &self.code
    }
}

impl fmt::Display for InvalidHexCodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid hex color code: '{}'", self.code)
    }
}

impl std::error::Error for InvalidHexCodeError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InvalidColorNumberError {
    number: usize,
}

impl InvalidColorNumberError {
    pub const MAX_COLOR_NUMBER: usize = 6;

    pub fn new(number: usize) -> Self {
        Self { number }
    }

    /// Returns the invalid color number
    pub fn number(&self) -> usize {
        self.number
    }
}

impl fmt::Display for InvalidColorNumberError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "invalid color number: {} (must be 0-{})",
            self.number,
            Self::MAX_COLOR_NUMBER
        )
    }
}

impl std::error::Error for InvalidColorNumberError {}
