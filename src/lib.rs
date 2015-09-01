//! XDR runtime encoding/decoding
//!
//! This crate provides runtime support for encoding and decoding XDR
//! data. It is intended to be used with code generated by the
//! "xdrgen" crate, but it can also be used with hand-written code.
//!
//! It provides two key traits - `Pack` and `Unpack` - which all
//! encodable types must implement. It also provides the helper
//! functions `pack()` and `unpack()` to simplify the API.
#![feature(slice_patterns, box_patterns)]
#![crate_type = "lib"]

extern crate byteorder;

pub use std::io::{Write, Read};

mod xdr;
pub use xdr::{Result, Error, Pack, Unpack, pack, unpack};
