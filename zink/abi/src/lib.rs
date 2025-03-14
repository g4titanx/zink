//! Zink ABI implementation
//!
//! Currently just a wrapper of solidity ABI.

mod abi;
pub mod result;
pub mod selector;
#[cfg(feature = "encoding")]
mod encoding;
#[cfg(feature = "encoding")]
pub use encoding::{encode, decode, AbiEncode, AbiDecode, is_dynamic_type, DecodeError};

pub use abi::Abi;

#[cfg(feature = "selector")]
pub use selector::keccak256;
