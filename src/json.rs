//  JSON.rs
//    by Lut99
//
//  Created:
//    28 Oct 2023, 13:02:40
//  Last edited:
//    29 Oct 2023, 14:17:33
//  Auto updated?
//    Yes
//
//  Description:
//!   Implements [`serializer::Serializer`] and cohorts for [`serde_json`].
//

use std::error;
use std::fmt::{Display, Formatter, Result as FResult};
use std::io::{Read, Write};
use std::marker::PhantomData;

use serde::{Deserialize, Serialize};

use crate::serializer;


/***** ERRORS *****/
/// Defines errors that occur when using the JSON [`Serializer`].
#[derive(Debug)]
pub enum Error {
    /// Failed to write to the given writer.
    #[cfg(feature = "async-tokio")]
    Write { err: std::io::Error },
    /// Failed to read from the given reader.
    #[cfg(feature = "async-tokio")]
    Read { err: std::io::Error },
    /// Failed to serialize the object to JSON.
    Serialize { err: serde_json::Error },
    /// Failed to deserialize the object from JSON.
    Deserialize { err: serde_json::Error },
}
impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        use Error::*;
        match self {
            #[cfg(feature = "async-tokio")]
            Write { .. } => write!(f, "Failed to write to given writer"),
            #[cfg(feature = "async-tokio")]
            Read { .. } => write!(f, "Failed to read from given reader"),
            Serialize { .. } => write!(f, "Failed to serialize to JSON"),
            Deserialize { .. } => write!(f, "Failed to deserialize from JSON"),
        }
    }
}
impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        use Error::*;
        match self {
            #[cfg(feature = "async-tokio")]
            Write { err } => Some(err),
            #[cfg(feature = "async-tokio")]
            Read { err } => Some(err),
            Serialize { err } => Some(err),
            Deserialize { err } => Some(err),
        }
    }
}





/***** LIBRARY *****/
/// Implements a [`serializer::Serializer`] for [`serde_json`].
#[derive(Clone, Copy, Debug)]
pub struct Serializer<T>(PhantomData<T>);

impl<T: for<'de> Deserialize<'de> + Serialize> serializer::Serializer for Serializer<T> {
    type Error = Error;
    type Target = T;

    #[inline]
    fn to_string(value: &Self::Target) -> Result<String, Self::Error> { serde_json::to_string(value).map_err(|err| Error::Serialize { err }) }

    #[inline]
    fn to_string_pretty(value: &Self::Target) -> Result<String, Self::Error> {
        serde_json::to_string_pretty(value).map_err(|err| Error::Serialize { err })
    }

    #[inline]
    fn to_writer(value: &Self::Target, writer: impl Write) -> Result<(), Self::Error> {
        serde_json::to_writer(writer, value).map_err(|err| Error::Serialize { err })
    }

    #[inline]
    fn to_writer_pretty(value: &Self::Target, writer: impl Write) -> Result<(), Self::Error> {
        serde_json::to_writer_pretty(writer, value).map_err(|err| Error::Serialize { err })
    }

    #[inline]
    fn from_str(raw: impl AsRef<str>) -> Result<Self::Target, Self::Error> {
        serde_json::from_str(raw.as_ref()).map_err(|err| Error::Serialize { err })
    }

    #[inline]
    fn from_reader(reader: impl Read) -> Result<Self::Target, Self::Error> { serde_json::from_reader(reader).map_err(|err| Error::Serialize { err }) }
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
        match writer.write_all(raw.as_bytes()).await {
            Ok(_) => Ok(()),
            Err(err) => Err(Error::Write { err }),
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
        match writer.write_all(raw.as_bytes()).await {
            Ok(_) => Ok(()),
            Err(err) => Err(Error::Write { err }),
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
