//! mkv-rs
//!
//! ## Description
//!
//! This library aims to provide a simple and ergonomic way to work with Matroska files (MKV/WebM).

#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate ebml;

pub mod error;
pub mod elements;
pub mod structures;
pub mod reader;

pub use reader::Reader;

#[cfg(test)]
mod tests;
