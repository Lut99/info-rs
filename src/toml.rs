//  TOML.rs
//    by Lut99
//
//  Created:
//    28 Oct 2023, 13:05:57
//  Last edited:
//    30 Oct 2023, 12:32:57
//  Auto updated?
//    Yes
//
//  Description:
//!   Implements [`serializer::Serializer`] and cohorts for [`toml`].
//

use std::error;
use std::fmt::{Display, Formatter, Result as FResult};
use std::io::{Read, Write};
use std::marker::PhantomData;

use serde::{Deserialize, Serialize};

use crate::serializer;


/***** ERRORS *****/
/// Defines errors that occur when using the TOML [`Serializer`].
#[derive(Debug)]
pub enum Error {
    /// Failed to write to the given writer.
    Write { err: std::io::Error },
    /// Failed to read from the given reader.
    Read { err: std::io::Error },
    /// Failed to flush the given writer.
    #[cfg(feature = "async-tokio")]
    Flush { err: std::io::Error },
    /// Failed to serialize the object to TOML.
    Serialize { err: toml::ser::Error },
    /// Failed to deserialize the object from TOML.
    Deserialize { err: toml::de::Error },
}
#[cfg(feature = "serde-toml")]
impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        use Error::*;
        match self {
            Write { .. } => write!(f, "Failed to write to given writer"),
            Read { .. } => write!(f, "Failed to read from given reader"),
            #[cfg(feature = "async-tokio")]
            Flush { .. } => write!(f, "Failed to flush the given writer"),
            Serialize { .. } => write!(f, "Failed to serialize to TOML"),
            Deserialize { .. } => write!(f, "Failed to deserialize from TOML"),
        }
    }
}
#[cfg(feature = "serde-toml")]
impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        use Error::*;
        match self {
            Write { err } => Some(err),
            Read { err } => Some(err),
            #[cfg(feature = "async-tokio")]
            Flush { err } => Some(err),
            Serialize { err } => Some(err),
            Deserialize { err } => Some(err),
        }
    }
}





/***** LIBRARY *****/
/// Implements a [`serializer::Serializer`] for [`toml`].
///
/// # Examples
/// ```rust
/// use serde::{Deserialize, Serialize};
/// use serializable::toml::Serializer;
/// use serializable::Serializable;
///
/// #[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
/// struct HelloWorld {
///     hello: String,
///     world: String,
/// }
/// impl Serializable<Serializer<HelloWorld>> for HelloWorld {}
///
/// assert_eq!(
///     HelloWorld { hello: "Hello".into(), world: "World".into() }.to_string().unwrap(),
///     "hello = \"Hello\"\nworld = \"World\"\n"
/// );
///
/// assert_eq!(
///     HelloWorld::from_str("hello = \"Goodbye\"\nworld = \"Planet\"\n").unwrap(),
///     HelloWorld { hello: "Goodbye".into(), world: "Planet".into() }
/// )
/// ```
#[derive(Clone, Copy, Debug)]
pub struct Serializer<T>(PhantomData<T>);

impl<T: for<'de> Deserialize<'de> + Serialize> serializer::Serializer for Serializer<T> {
    type Error = Error;
    type Target = T;

    #[inline]
    fn to_string(value: &Self::Target) -> Result<String, Self::Error> { toml::to_string(value).map_err(|err| Error::Serialize { err }) }

    #[inline]
    fn to_string_pretty(value: &Self::Target) -> Result<String, Self::Error> { toml::to_string_pretty(value).map_err(|err| Error::Serialize { err }) }

    #[inline]
    fn to_writer(value: &Self::Target, mut writer: impl Write) -> Result<(), Self::Error> {
        // Write to string first
        let raw: String = match toml::to_string(value) {
            Ok(raw) => raw,
            Err(err) => return Err(Error::Serialize { err }),
        };

        // Then write to the writer
        match writer.write_all(raw.as_bytes()) {
            Ok(_) => Ok(()),
            Err(err) => Err(Error::Write { err }),
        }
    }

    #[inline]
    fn to_writer_pretty(value: &Self::Target, mut writer: impl Write) -> Result<(), Self::Error> {
        // Write to string first
        let raw: String = match toml::to_string_pretty(value) {
            Ok(raw) => raw,
            Err(err) => return Err(Error::Serialize { err }),
        };

        // Then write to the writer
        match writer.write_all(raw.as_bytes()) {
            Ok(_) => Ok(()),
            Err(err) => Err(Error::Write { err }),
        }
    }

    #[inline]
    fn from_str(raw: impl AsRef<str>) -> Result<Self::Target, Self::Error> { toml::from_str(raw.as_ref()).map_err(|err| Error::Deserialize { err }) }

    #[inline]
    fn from_reader(mut reader: impl Read) -> Result<Self::Target, Self::Error> {
        // Simply read the whole reader
        let mut raw: String = String::new();
        if let Err(err) = reader.read_to_string(&mut raw) {
            return Err(Error::Read { err });
        }

        // Now deserialize using the string edition
        match toml::from_str(&raw) {
            Ok(res) => Ok(res),
            Err(err) => Err(Error::Deserialize { err }),
        }
    }
}

#[cfg(feature = "async-tokio")]
#[async_trait::async_trait]
impl<T: Send + Sync + for<'de> Deserialize<'de> + Serialize> serializer::SerializerAsync for Serializer<T> {
    #[inline]
    async fn to_writer_async(value: &Self::Target, mut writer: impl Send + std::marker::Unpin + tokio::io::AsyncWrite) -> Result<(), Self::Error> {
        use tokio::io::AsyncWriteExt;

        // Serialize ourselves to a string first
        let raw: String = <Self as serializer::Serializer>::to_string(value)?;

        // Now write to the writer
        if let Err(err) = writer.write_all(raw.as_bytes()).await {
            return Err(Error::Write { err });
        }
        // ...making sure its flushed
        match writer.flush().await {
            Ok(_) => Ok(()),
            Err(err) => Err(Error::Flush { err }),
        }
    }

    #[inline]
    async fn to_writer_pretty_async(
        value: &Self::Target,
        mut writer: impl Send + std::marker::Unpin + tokio::io::AsyncWrite,
    ) -> Result<(), Self::Error> {
        use tokio::io::AsyncWriteExt;

        // Serialize ourselves to a string first
        let raw: String = <Self as serializer::Serializer>::to_string_pretty(value)?;

        // Now write to the writer
        if let Err(err) = writer.write_all(raw.as_bytes()).await {
            return Err(Error::Write { err });
        }
        // ...making sure its flushed
        match writer.flush().await {
            Ok(_) => Ok(()),
            Err(err) => Err(Error::Flush { err }),
        }
    }

    #[inline]
    async fn from_reader_async(mut reader: impl Send + std::marker::Unpin + tokio::io::AsyncRead) -> Result<T, Self::Error> {
        use tokio::io::AsyncReadExt;

        // Read the entire buffer first
        let mut raw: String = String::new();
        if let Err(err) = reader.read_to_string(&mut raw).await {
            return Err(Error::Read { err });
        }

        // Then deserialize as string
        <Self as serializer::Serializer>::from_str(&raw)
    }
}
