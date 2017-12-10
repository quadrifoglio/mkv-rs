//! Error handling functionality.

use std::fmt::Display;

error_chain! {
    errors {
        UnexpectedEof {
            description("Unexpected Enf of File")
        }

        ElementNotFound(el: u64) {
            description("Element not found"),
            display("Element '0x{:X}' not found", el)
        }

        UnexpectedElement(expected: u64, found: u64) {
            description("Unexpected element"),
            display("Unexpected element: was expecting '0x{:X}', but got '0x{:X}'", expected, found)
        }

        InvalidElementValue(el: u64, val: String) {
            description("Got an invalid value for an element"),
            display("Invalid value for element '0x{:X}': {}", el, val)
        }
    }

    foreign_links {
        Io(::std::io::Error);
        Ebml(::ebml::error::Error);
    }
}

/// Construct an `UnexpectedEof` error.
pub fn unexpected_eof() -> Error {
    Error::from(ErrorKind::UnexpectedEof)
}

/// Construct an `ElementNotFound` error.
pub fn not_found(el: u64) -> Error {
    Error::from(ErrorKind::ElementNotFound(el))
}

/// Construct an `UnexpectedElement` error.
pub fn unexpected(expected: u64, got: u64) -> Error {
    Error::from(ErrorKind::UnexpectedElement(expected, got))
}

/// Construct an `InvalidElementValue` error.
pub fn invalid_value<D: Display>(el: u64, val: D) -> Error {
    Error::from(ErrorKind::InvalidElementValue(el, format!("{}", val)))
}
