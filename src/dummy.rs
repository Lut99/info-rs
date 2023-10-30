//  DUMMY.rs
//    by Lut99
//
//  Created:
//    29 Oct 2023, 11:59:19
//  Last edited:
//    30 Oct 2023, 12:30:22
//  Auto updated?
//    Yes
//
//  Description:
//!   Defines a simple dummy serializer/deserializer that is used in the
//!   doctests.
//

use std::error;
use std::fmt::{Debug, Display, Formatter, Result as FResult};
use std::marker::PhantomData;

use crate::serializer;


/***** ERRORS *****/
/// Defines errors that occur when using the dummy [`Serializer`].
#[derive(Debug)]
pub enum Error {
    /// Failed to write to the given writer.
    Write { err: std::io::Error },
    /// Failed to read from the given reader.
    Read { err: std::io::Error },
    /// Failed to flush the given reader.
    #[cfg(feature = "async-tokio")]
    Flush { err: std::io::Error },
}
impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        use Error::*;
        match self {
            Write { .. } => write!(f, "Failed to write to given writer"),
            Read { .. } => write!(f, "Failed to read from given reader"),
            #[cfg(feature = "async-tokio")]
            Flush { .. } => write!(f, "Failed to flush the given writer"),
        }
    }
}
impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        use Error::*;
        match self {
            Write { err } => Some(err),
            Read { err } => Some(err),
            #[cfg(feature = "async-tokio")]
            Flush { err } => Some(err),
        }
    }
}





/***** LIBRARY *****/
/// Defines a dummy serializer that serializes to a constant value and "deserializes" by calling the target's [`Default`]-implementation.
///
/// Mostly used in examples and (doc)tests.
///
/// # Examples
/// ```rust
/// use serializable::dummy::Serializer;
/// use serializable::Serializable;
///
/// #[derive(Debug, Default, Eq, PartialEq)]
/// struct HelloWorld {
///     hello: String,
///     world: String,
/// }
/// impl Serializable<Serializer<HelloWorld>> for HelloWorld {}
///
/// assert_eq!(
///     <HelloWorld as Serializable<_>>::to_string(&HelloWorld {
///         hello: "Hello".into(),
///         world: "World".into(),
///     })
///     .unwrap(),
///     "<dummy_text>"
/// );
///
/// assert_eq!(HelloWorld::from_str("<dummy_text>").unwrap(), HelloWorld {
///     hello: "".into(),
///     world: "".into(),
/// })
/// ```
#[derive(Clone, Copy, Debug)]
pub struct Serializer<T>(PhantomData<T>);

impl<T: Default> serializer::Serializer for Serializer<T> {
    type Error = Error;
    type Target = T;

    fn to_string(_value: &Self::Target) -> Result<String, Self::Error> { Ok("<dummy_text>".into()) }

    fn to_string_pretty(_value: &Self::Target) -> Result<String, Self::Error> { Ok("Dummy Text".into()) }

    fn to_writer(value: &Self::Target, mut writer: impl std::io::Write) -> Result<(), Self::Error> {
        writer.write_all(Self::to_string(value)?.as_bytes()).map_err(|err| Error::Write { err })
    }

    fn to_writer_pretty(value: &Self::Target, mut writer: impl std::io::Write) -> Result<(), Self::Error> {
        writer.write_all(Self::to_string_pretty(value)?.as_bytes()).map_err(|err| Error::Write { err })
    }

    fn from_str(_raw: impl AsRef<str>) -> Result<Self::Target, Self::Error> { Ok(Self::Target::default()) }

    fn from_reader(mut reader: impl std::io::Read) -> Result<Self::Target, Self::Error> {
        // Read from the reader first...
        let mut raw: String = String::new();
        if let Err(err) = reader.read_to_string(&mut raw) {
            return Err(Error::Read { err });
        }

        // ...and then deserialize
        Self::from_str(&raw)
    }
}

#[cfg(feature = "async-tokio")]
#[async_trait::async_trait]
impl<T: Send + Sync + Default> serializer::SerializerAsync for Serializer<T> {
    async fn to_writer_async(value: &Self::Target, mut writer: impl Send + std::marker::Unpin + tokio::io::AsyncWrite) -> Result<(), Self::Error> {
        use tokio::io::AsyncWriteExt as _;
        writer.write_all(<Self as serializer::Serializer>::to_string(value)?.as_bytes()).await.map_err(|err| Error::Write { err })?;
        writer.flush().await.map_err(|err| Error::Flush { err })
    }

    async fn to_writer_pretty_async(
        value: &Self::Target,
        mut writer: impl Send + std::marker::Unpin + tokio::io::AsyncWrite,
    ) -> Result<(), Self::Error> {
        use tokio::io::AsyncWriteExt as _;
        writer.write_all(<Self as serializer::Serializer>::to_string_pretty(value)?.as_bytes()).await.map_err(|err| Error::Write { err })?;
        writer.flush().await.map_err(|err| Error::Flush { err })
    }

    async fn from_reader_async(mut reader: impl Send + std::marker::Unpin + tokio::io::AsyncRead) -> Result<Self::Target, Self::Error> {
        use tokio::io::AsyncReadExt as _;

        // Read from the reader first...
        let mut raw: String = String::new();
        if let Err(err) = reader.read_to_string(&mut raw).await {
            return Err(Error::Read { err });
        }

        // ...and then deserialize
        <Self as serializer::Serializer>::from_str(&raw)
    }
}
