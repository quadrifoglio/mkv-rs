//! mkv-rs
//!
//! ## Description
//!
//! This library aims to provide a simple and ergonomic way to work with Matroska files (MKV/WebM).

#[macro_use]
extern crate error_chain;

mod error;
mod reader;

pub use reader::Reader;

#[cfg(test)]
mod tests;
