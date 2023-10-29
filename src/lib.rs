//  LIB.rs
//    by Lut99
//
//  Created:
//    24 Oct 2023, 22:36:08
//  Last edited:
//    29 Oct 2023, 11:58:46
//  Auto updated?
//    Yes
//
//  Description:
//!   Provides helper traits for [serde](https://serde.rs) types that makes
//!   working with them slightly nicer.
//

// Declare the submodules
pub mod dummy;
#[cfg(feature = "serde-json")]
pub mod json;
pub mod serializable;
pub mod serializer;
#[cfg(feature = "serde-toml")]
pub mod toml;
#[cfg(feature = "serde-yaml")]
pub mod yaml;

// Bring some of that into the crate namespace
pub use serializable::{Error, Serializable};
pub use serializer::Serializer;
#[cfg(feature = "async-tokio")]
pub use serializer::SerializerAsync;
