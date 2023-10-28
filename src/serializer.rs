//  SERIALIZER.rs
//    by Lut99
//
//  Created:
//    28 Oct 2023, 10:21:11
//  Last edited:
//    28 Oct 2023, 13:11:24
//  Auto updated?
//    Yes
//
//  Description:
//!   Defines the [`Serializer`]-trait (and various library-provided
//!   `impl`s) that abstracts away which serde serializer (or other
//!   serializer!) we're using.
//

use std::error::Error;
use std::io::{Read, Write};


/***** LIBRARY **** */
/// Defines a trait that abstracts over the possible serializers.
pub trait Serializer {
    type Target;
    type Error: Error;

    /// Serializes the given value to a string in accordance with the backend
    /// implementation.
    ///
    /// # Arguments
    /// - `value`: The value to serialize.
    ///
    /// # Returns
    /// A string with the serialized value.
    ///
    /// # Errors
    /// This function may error if the given value was not serializable in its
    /// current state.
    fn to_string(value: &Self::Target) -> Result<String, Self::Error>;
    /// Serializes the given value to a string in accordance with the backend
    /// implementation's pretty serialization.
    ///
    /// Not all backends are expected to define a meaningful difference ([YAML](https://yaml.org)), for example.
    /// If so, then the default implementation can be used, which is then simply
    /// an alias for [`Self::to_string()`](Serializer::to_string()).
    ///
    /// # Arguments
    /// - `value`: The value to serialize.
    ///
    /// # Returns
    /// A string with the serialized value.
    ///
    /// # Errors
    /// This function may error if the given value was not serializable in its
    /// current state.
    #[inline]
    fn to_string_pretty(value: &Self::Target) -> Result<String, Self::Error> { Self::to_string(value) }

    /// Serializes the given value to the given writer in accordance with the
    /// backend implementation.
    ///
    /// # Arguments
    /// - `value`: The value to serialize.
    /// - `writer`: The [`Write`]r to serialize to.
    ///
    /// # Errors
    /// This function may error if the given value was not serializable in its
    /// current state, or if it failed to write to the given `writer`.
    fn to_writer(value: &Self::Target, writer: impl Write) -> Result<(), Self::Error>;

    /// Serializes the given value to the given writer in accordance with the
    /// backend implementation.
    ///
    /// Not all backends are expected to define a meaningful difference ([YAML](https://yaml.org)), for example.
    /// If so, then the default implementation can be used, which is then simply
    /// an alias for [`Self::to_writer()`](Serializer::to_writer()).
    ///
    /// # Arguments
    /// - `value`: The value to serialize.
    /// - `writer`: The [`Write`]r to serialize to.
    ///
    /// # Errors
    /// This function may error if the given value was not serializable in its
    /// current state, or if it failed to write to the given `writer`.
    #[inline]
    fn to_writer_pretty(value: &Self::Target, writer: impl Write) -> Result<(), Self::Error> { Self::to_writer(value, writer) }

    /// Deserializes the given string as a representation for the target type in
    /// the backend format.
    ///
    /// # Arguments
    /// - `raw`: The string that contains the serialized representation of the
    ///   target.
    ///
    /// # Returns
    /// The deserialized target.
    ///
    /// # Errors
    /// This function may error if the given `raw` is not a valid representation
    /// for a target in the backend format.
    fn from_str(raw: impl AsRef<str>) -> Result<Self::Target, Self::Error>;
    /// Deserializes the contents of the given reader as a representation for
    /// the target type in the backend format.
    ///
    /// # Arguments
    /// - `reader`: The [`Read`]er that contains the serialized representation
    ///   of the target.
    ///
    /// # Returns
    /// The deserialized target.
    ///
    /// # Errors
    /// This function may error if the given `raw` is not a valid representation
    /// for a target in the backend format, or if it failed to read from the
    /// given `reader`.
    fn from_reader(reader: impl Read) -> Result<Self::Target, Self::Error>;
}



/// Defines a complement to the [`Serializer`] that implements it asynchronously.
///
/// Note that support by backends for this varies. [`serde`](https://serde.rs)-related backends,
/// for example, do not, and hence the file is read in memory in one go asynchronously and then
/// parsed synchronously.
#[cfg(feature = "async-tokio")]
pub trait SerializerAsync: Serializer {}
