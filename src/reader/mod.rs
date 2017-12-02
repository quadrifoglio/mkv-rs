//! This module contains the functionality to read and parse MKV elements.

macro_rules! find_child {
    ($parent:ident, $child:expr) => {
        if let Some(res) = $parent.find($child) {
            res
        } else {
            bail!($crate::error::ErrorKind::ElementNotFound($child));
        }
    }
}

macro_rules! find_child_uint {
    ($parent:ident, $child:expr) => {
        find_child!($parent, $child).data().to_unsigned_int()?
    }
}

macro_rules! find_child_utf8 {
    ($parent:ident, $child:expr) => {
        find_child!($parent, $child).data().to_utf8()?
    }
}

macro_rules! find_child_uint_or {
    ($parent:ident, $child:expr, $default:expr) => {
        if let Some(value) = $parent.find($child) {
            value.data().to_unsigned_int()?
        } else {
            $default
        }
    }
}

macro_rules! find_child_float_or {
    ($parent:ident, $child:expr, $default:expr) => {
        if let Some(value) = $parent.find($child) {
            value.data().to_float()?
        } else {
            $default
        }
    }
}

pub mod segment;
pub mod tracks;
