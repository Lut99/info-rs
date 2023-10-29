//  SERIALIZER.rs
//    by Lut99
//
//  Created:
//    28 Oct 2023, 10:21:11
//  Last edited:
//    29 Oct 2023, 14:51:54
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
///
/// # Example
/// For an example of a Serializer implementation, see the source code for the [dummy serializer](crate::dummy::Serializer).
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
    ///
    /// # Examples
    /// ```rust
    /// # use std::path::PathBuf;
    /// use serializable::dummy::Serializer;
    /// use serializable::Serializer as _;
    ///
    /// // The dummy parser can be used for any type implementing `FromStr` and `ToString`
    /// assert_eq!(Serializer::to_string(&42u8).unwrap(), "42");
    /// assert_eq!(Serializer::to_string(&String::from("42")).unwrap(), "42");
    /// assert_eq!(Serializer::to_string(&true).unwrap(), "true");
    /// ```
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
    ///
    /// # Examples
    /// ```rust
    /// # use std::path::PathBuf;
    /// use serializable::dummy::Serializer;
    /// use serializable::Serializer as _;
    ///
    /// // The dummy parser can be used for any type implementing `FromStr` and `ToString`
    /// assert_eq!(Serializer::to_string_pretty(&42u8).unwrap(), "Dummy<42>");
    /// assert_eq!(Serializer::to_string_pretty(&String::from("42")).unwrap(), "Dummy<42>");
    /// assert_eq!(Serializer::to_string_pretty(&true).unwrap(), "Dummy<true>");
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
    ///
    /// # Examples
    /// ```rust
    /// # use std::path::PathBuf;
    /// use serializable::dummy::Serializer;
    /// use serializable::Serializer as _;
    ///
    /// let mut buf: [u8; 4] = [0; 4];
    ///
    /// // The dummy parser can be used for any type implementing `FromStr` and `ToString`
    /// Serializer::to_writer(&42u8, &mut buf[..]).unwrap();
    /// assert_eq!(String::from_utf8_lossy(&buf[..2]), "42");
    ///
    /// Serializer::to_writer(&String::from("42"), &mut buf[..]).unwrap();
    /// assert_eq!(String::from_utf8_lossy(&buf[..2]), "42");
    ///
    /// Serializer::to_writer(&true, &mut buf[..]).unwrap();
    /// assert_eq!(String::from_utf8_lossy(&buf), "true");
    ///
    /// // Errors when writing are propagated
    /// assert!(matches!(
    ///     Serializer::to_writer(&String::from("Hello, there!"), &mut buf[..]),
    ///     Err(serializable::dummy::Error::Write { .. })
    /// ));
    /// ```
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
    ///
    /// # Examples
    /// ```rust
    /// # use std::path::PathBuf;
    /// use serializable::dummy::Serializer;
    /// use serializable::Serializer as _;
    ///
    /// let mut buf: [u8; 11] = [0; 11];
    ///
    /// // The dummy parser can be used for any type implementing `FromStr` and `ToString`
    /// Serializer::to_writer_pretty(&42u8, &mut buf[..]).unwrap();
    /// assert_eq!(String::from_utf8_lossy(&buf[..9]), "Dummy<42>");
    ///
    /// Serializer::to_writer_pretty(&String::from("42"), &mut buf[..]).unwrap();
    /// assert_eq!(String::from_utf8_lossy(&buf[..9]), "Dummy<42>");
    ///
    /// Serializer::to_writer_pretty(&true, &mut buf[..]).unwrap();
    /// assert_eq!(String::from_utf8_lossy(&buf), "Dummy<true>");
    ///
    /// // Errors when writing are propagated
    /// assert!(matches!(
    ///     Serializer::to_writer_pretty(&String::from("Hello, there!"), &mut buf[..]),
    ///     Err(serializable::dummy::Error::Write { .. })
    /// ));
    /// ```
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
    ///
    /// # Examples
    /// ```rust
    /// use serializable::dummy::Serializer;
    /// use serializable::Serializer as _;
    ///
    /// // The dummy parser can be used for any type implementing `FromStr` and `ToString`
    /// assert_eq!(Serializer::<u8>::from_str("42").unwrap(), 42);
    /// assert_eq!(Serializer::<String>::from_str("42").unwrap(), "42");
    /// assert_eq!(Serializer::<bool>::from_str("true").unwrap(), true);
    /// ```
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
    ///
    /// # Examples
    /// ```rust
    /// use serializable::dummy::Serializer;
    /// use serializable::Serializer as _;
    ///
    /// // The dummy parser can be used for any type implementing `FromStr` and `ToString`
    /// assert_eq!(Serializer::<u8>::from_reader("42".as_bytes()).unwrap(), 42);
    /// assert_eq!(Serializer::<String>::from_reader("42".as_bytes()).unwrap(), "42");
    /// assert_eq!(Serializer::<bool>::from_reader("true".as_bytes()).unwrap(), true);
    /// ```
    fn from_reader(reader: impl Read) -> Result<Self::Target, Self::Error>;
}



/// Defines a complement to the [`Serializer`] that implements reader- and writer-related functions asynchronously.
///
/// Note that support by backends for this varies. [`serde`](https://serde.rs)-related backends,
/// for example, do not, and hence the file is read in memory in one go asynchronously and then
/// parsed synchronously.
#[cfg(feature = "async-tokio")]
#[async_trait::async_trait]
pub trait SerializerAsync: Serializer
where
    Self::Target: Send + Sync,
{
    /// Serializes the given value to the given writer asynchronously in accordance with the backend implementation.
    ///
    /// # Arguments
    /// - `value`: The value to serialize.
    /// - `writer`: The [`Write`]r to serialize to.
    ///
    /// # Errors
    /// This function may error if the given value was not serializable in its
    /// current state, or if it failed to write to the given `writer`.
    ///
    /// # Examples
    /// ```rust
    /// # use std::path::PathBuf;
    /// use serializable::dummy::Serializer;
    /// use serializable::SerializerAsync as _;
    ///
    /// # tokio_test::block_on(async {
    /// // The dummy parser can be used for any type implementing `FromStr` and `ToString`
    /// let mut buf: Vec<u8> = Vec::new();
    /// Serializer::to_writer_async(&42u8, &mut buf).await.unwrap();
    /// assert_eq!(String::from_utf8_lossy(&buf), "42");
    ///
    /// let mut buf: Vec<u8> = Vec::new();
    /// Serializer::to_writer_async(&String::from("42"), &mut buf).await.unwrap();
    /// assert_eq!(String::from_utf8_lossy(&buf), "42");
    ///
    /// let mut buf: Vec<u8> = Vec::new();
    /// Serializer::to_writer_async(&true, &mut buf).await.unwrap();
    /// assert_eq!(String::from_utf8_lossy(&buf), "true");
    /// # });
    /// ```
    async fn to_writer_async(value: &Self::Target, writer: impl Send + std::marker::Unpin + tokio::io::AsyncWrite) -> Result<(), Self::Error>;
    /// Serializes the given value to the given writer asynchronously in accordance with the
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
    ///
    /// # Examples
    /// ```rust
    /// # use std::path::PathBuf;
    /// use serializable::dummy::Serializer;
    /// use serializable::SerializerAsync as _;
    ///
    /// # tokio_test::block_on(async {
    /// // The dummy parser can be used for any type implementing `FromStr` and `ToString`
    /// let mut buf: Vec<u8> = Vec::new();
    /// Serializer::to_writer_pretty_async(&42u8, &mut buf).await.unwrap();
    /// assert_eq!(String::from_utf8_lossy(&buf), "Dummy<42>");
    ///
    /// let mut buf: Vec<u8> = Vec::new();
    /// Serializer::to_writer_pretty_async(&String::from("42"), &mut buf).await.unwrap();
    /// assert_eq!(String::from_utf8_lossy(&buf), "Dummy<42>");
    ///
    /// let mut buf: Vec<u8> = Vec::new();
    /// Serializer::to_writer_pretty_async(&true, &mut buf).await.unwrap();
    /// assert_eq!(String::from_utf8_lossy(&buf), "Dummy<true>");
    /// # });
    /// ```
    #[inline]
    async fn to_writer_pretty_async(value: &Self::Target, writer: impl Send + std::marker::Unpin + tokio::io::AsyncWrite) -> Result<(), Self::Error> {
        Self::to_writer_async(value, writer).await
    }

    /// Deserializes the contents of the given reader asynchronously as a representation for
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
    ///
    /// # Examples
    /// ```rust
    /// use serializable::dummy::Serializer;
    /// use serializable::SerializerAsync as _;
    ///
    /// # tokio_test::block_on(async {
    /// // The dummy parser can be used for any type implementing `FromStr` and `ToString`
    /// assert_eq!(Serializer::<u8>::from_reader_async("42".as_bytes()).await.unwrap(), 42);
    /// assert_eq!(Serializer::<String>::from_reader_async("42".as_bytes()).await.unwrap(), "42");
    /// assert_eq!(Serializer::<bool>::from_reader_async("true".as_bytes()).await.unwrap(), true);
    /// # });
    /// ```
    async fn from_reader_async(reader: impl Send + std::marker::Unpin + tokio::io::AsyncRead) -> Result<Self::Target, Self::Error>;
}
