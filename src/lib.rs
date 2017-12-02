//! # mkv-rs
//!
//! Basic implementation of the MKV (Matroska) video format for the Rust Programming Language. 

#[macro_use]
extern crate error_chain;

extern crate ebml;

pub mod error;
pub mod elements;
pub mod reader;

#[cfg(test)]
mod tests;
