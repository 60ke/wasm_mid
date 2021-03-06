//! Externs library to interact with Ethereum-like network

#![cfg_attr(not(feature="std"), no_std)]

extern crate wasm_std;

mod ext;
mod storage;

pub use ext::*;
pub use storage::*;
