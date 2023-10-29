//  DUMMY.rs
//    by Lut99
//
//  Created:
//    29 Oct 2023, 11:59:19
//  Last edited:
//    29 Oct 2023, 17:38:22
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
use std::str::FromStr;
use std::string::ToString;

use crate::serializer;


/***** ERRORS *****/
/// Defines errors that occur when using the dummy [`Serializer`].
#[derive(Debug)]
pub enum Error<E> {
    /// Failed to write to the given writer.
    Write { err: std::io::Error },
    /// Failed to read from the given reader.
    Read { err: std::io::Error },
    /// Failed to deserialize the given type.
    Deserialize { err: E },
}
impl<E: Display> Display for Error<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        use Error::*;
        match self {
            Write { .. } => write!(f, "Failed to write to given writer"),
            Read { .. } => write!(f, "Failed to read from given reader"),
            Deserialize { .. } => write!(f, "Failed to deserialize as FromStr"),
        }
    }
}
impl<E: 'static + error::Error> error::Error for Error<E> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        use Error::*;
        match self {
            Write { err } => Some(err),
            Read { err } => Some(err),
            Deserialize { err } => Some(err),
        }
    }
}





/***** LIBRARY *****/
/// Defines a dummy serializer that serializes naively using a type's [`ToString`]-implementation for serialization, and [`FromStr`] for deserialization.
///
/// Mostly used in examples and (doc)tests.
///
/// # Examples
/// ```rust
/// use serializable::dummy::Serializer;
/// use serializable::Serializable;
///
/// #[derive(Debug, Eq, PartialEq)]
/// struct HelloWorld {
///     hello: String,
///     world: String,
/// }
/// impl std::str::FromStr for HelloWorld {
///     type Err = std::convert::Infallible;
///
///     fn from_str(value: &str) -> Result<Self, Self::Err> {
///         if let Some(pos) = value.find(',') {
///             Ok(Self { hello: value[..pos].into(), world: value[pos + 1..].into() })
///         } else {
///             Ok(Self { hello: value.into(), world: "".into() })
///         }
///     }
/// }
/// impl ToString for HelloWorld {
///     fn to_string(&self) -> String { format!("{},{}", self.hello, self.world) }
/// }
/// impl Serializable<Serializer<HelloWorld>> for HelloWorld {}
///
/// assert_eq!(
///     <HelloWorld as Serializable<_>>::to_string(&HelloWorld {
///         hello: "Hello".into(),
///         world: "World".into(),
///     })
///     .unwrap(),
///     "Hello,World"
/// );
///
/// assert_eq!(HelloWorld::from_str("Goodbye,Planet").unwrap(), HelloWorld {
///     hello: "Goodbye".into(),
///     world: "Planet".into(),
/// })
/// ```
#[derive(Clone, Copy, Debug)]
pub struct Serializer<T>(PhantomData<T>);

impl<T: FromStr<Err = E> + ToString, E: 'static + error::Error> serializer::Serializer for Serializer<T> {
    type Error = Error<E>;
    type Target = T;

    fn to_string(value: &Self::Target) -> Result<String, Self::Error> { Ok(value.to_string()) }

    fn to_string_pretty(value: &Self::Target) -> Result<String, Self::Error> { Ok(format!("Dummy<{}>", value.to_string())) }

    fn to_writer(value: &Self::Target, mut writer: impl std::io::Write) -> Result<(), Self::Error> {
        writer.write_all(Self::to_string(value)?.as_bytes()).map_err(|err| Error::Write { err })
    }

    fn to_writer_pretty(value: &Self::Target, mut writer: impl std::io::Write) -> Result<(), Self::Error> {
        writer.write_all(Self::to_string_pretty(value)?.as_bytes()).map_err(|err| Error::Write { err })
    }

    fn from_str(raw: impl AsRef<str>) -> Result<Self::Target, Self::Error> {
        Self::Target::from_str(raw.as_ref()).map_err(|err| Error::Deserialize { err })
    }

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
impl<T: Send + Sync + FromStr<Err = E> + ToString, E: 'static + Send + error::Error> serializer::SerializerAsync for Serializer<T> {
    async fn to_writer_async(value: &Self::Target, mut writer: impl Send + std::marker::Unpin + tokio::io::AsyncWrite) -> Result<(), Self::Error> {
        use tokio::io::AsyncWriteExt as _;
        writer.write_all(<Self as serializer::Serializer>::to_string(value)?.as_bytes()).await.map_err(|err| Error::Write { err })
    }

    async fn to_writer_pretty_async(
        value: &Self::Target,
        mut writer: impl Send + std::marker::Unpin + tokio::io::AsyncWrite,
    ) -> Result<(), Self::Error> {
        use tokio::io::AsyncWriteExt as _;
        writer.write_all(<Self as serializer::Serializer>::to_string_pretty(value)?.as_bytes()).await.map_err(|err| Error::Write { err })
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
