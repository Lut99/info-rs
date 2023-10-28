//  SERIALIZABLE.rs
//    by Lut99
//
//  Created:
//    28 Oct 2023, 11:28:42
//  Last edited:
//    28 Oct 2023, 13:12:28
//  Auto updated?
//    Yes
//
//  Description:
//!   Defines the [`Serializable`], which conveniently implements
//!   functions to serialize and deserialize a type using serde (or other
//!   serializers).
//

use std::any::type_name;
use std::error;
use std::fmt::{Display, Formatter, Result as FResult};
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use crate::serializer::Serializer;





/***** ERRORS **** */
/// Defines errors that are occuring when using the [`Serializable`]-trait.
#[derive(Debug)]
pub enum Error<E> {
    /// Failed to create a new file.
    FileCreate { path: PathBuf, err: std::io::Error },
    /// Failed to open a new file.
    FileOpen { path: PathBuf, err: std::io::Error },

    /// Failed to serialize the type to a string.
    SerializeString { what: &'static str, err: E },
    /// Failed to serialize the type to a writer.
    SerializeWriter { what: &'static str, err: E },
    /// Failed to serialize the type to a file.
    SerializeFile { what: &'static str, path: PathBuf, err: E },

    /// Failed to deserialize the type from a string.
    DeserializeString { what: &'static str, err: E },
    /// Failed to deserialize the type from a reader.
    DeserializeReader { what: &'static str, err: E },
    /// Failed to deserialize the type from a file.
    DeserializeFile { what: &'static str, path: PathBuf, err: E },
}
impl<E: Display> Display for Error<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        use Error::*;
        match self {
            FileCreate { path, .. } => write!(f, "Failed to create output file '{}'", path.display()),
            FileOpen { path, .. } => write!(f, "Failed to open input file '{}'", path.display()),

            SerializeString { what, .. } => write!(f, "Failed to serialize {what} to a string"),
            SerializeWriter { what, .. } => {
                write!(f, "Failed to serialize {what} to the given writer")
            },
            SerializeFile { what, path, .. } => write!(f, "Failed to serialize {what} to file '{}'", path.display()),

            DeserializeString { what, .. } => {
                write!(f, "Failed to deserialize {what} from the given string")
            },
            DeserializeReader { what, .. } => {
                write!(f, "Failed to deserialize {what} from the given reader")
            },
            DeserializeFile { what, path, .. } => {
                write!(f, "Failed to deserialize {what} from file '{}'", path.display())
            },
        }
    }
}
impl<E: 'static + error::Error> error::Error for Error<E> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        use Error::*;
        match self {
            FileCreate { err, .. } => Some(err),
            FileOpen { err, .. } => Some(err),

            SerializeString { err, .. } => Some(err),
            SerializeWriter { err, .. } => Some(err),
            SerializeFile { err, .. } => Some(err),

            DeserializeString { err, .. } => Some(err),
            DeserializeReader { err, .. } => Some(err),
            DeserializeFile { err, .. } => Some(err),
        }
    }
}





/***** LIBRARY **** */
/// Conveniently implements functions to serialize- or deserialize a struct
/// using serde (or other serializers).
pub trait Serializable<T: Serializer<Target = Self>> {
    // Serializer backend aliases
    /// Serializes this object to a string.
    ///
    /// # Returns
    /// A string representing this object.
    ///
    /// # Errors
    /// This function may error with an [`Error::SerializeString`] if the
    /// backend serializer failed to serialize.
    #[inline]
    fn to_string(&self) -> Result<String, Error<T::Error>> {
        match T::to_string(self) {
            Ok(res) => Ok(res),
            Err(err) => Err(Error::SerializeString { what: type_name::<T::Target>(), err }),
        }
    }

    /// Serializes this object to a string, using a pretty backend if it's
    /// available.
    ///
    /// If not, then this is equivalent to calling
    /// [`Self::to_string()`](Serializable::to_string()).
    ///
    /// # Returns
    /// A string representing this object.
    ///
    /// # Errors
    /// This function may error with an [`Error::SerializeString`] if the
    /// backend serializer failed to serialize.
    #[inline]
    fn to_string_pretty(&self) -> Result<String, Error<T::Error>> {
        match T::to_string_pretty(self) {
            Ok(res) => Ok(res),
            Err(err) => Err(Error::SerializeString { what: type_name::<T::Target>(), err }),
        }
    }

    /// Serializes this object to the given writer.
    ///
    /// # Arguments
    /// - `writer`: A [`Write`]r that we will serialize to.
    ///
    /// # Errors
    /// This function may error with an [`Error::SerializeWriter`] if the
    /// backend serializer failed to serialize. This may also be because it
    /// failed to write to the `writer`.
    #[inline]
    fn to_writer(&self, writer: impl Write) -> Result<(), Error<T::Error>> {
        match T::to_writer(self, writer) {
            Ok(_) => Ok(()),
            Err(err) => Err(Error::SerializeWriter { what: type_name::<T::Target>(), err }),
        }
    }

    /// Serializes this object to the given writer, using a pretty backend if
    /// it's available.
    ///
    /// If not, then this is equivalent to calling
    /// [`Self::to_writer()`](Serializable::to_writer()).
    ///
    /// # Arguments
    /// - `writer`: A [`Write`]r that we will serialize to.
    ///
    /// # Errors
    /// This function may error with an [`Error::SerializeWriter`] if the
    /// backend serializer failed to serialize. This may also be because it
    /// failed to write to the `writer`.
    #[inline]
    fn to_writer_pretty(&self, writer: impl Write) -> Result<(), Error<T::Error>> {
        match T::to_writer_pretty(self, writer) {
            Ok(_) => Ok(()),
            Err(err) => Err(Error::SerializeWriter { what: type_name::<T::Target>(), err }),
        }
    }

    /// Deserializes this object from the given string.
    ///
    /// # Arguments
    /// - `raw`: The raw string that provides a serialized instantiation of
    ///   Self.
    ///
    /// # Returns
    /// A new Self that represents the deserialized counterpart.
    ///
    /// # Errors
    /// This function may error with an [`Error::DeserializeString`] if the
    /// backend deserializer failed to deserialize. This may be because the
    /// serialized representation was illegal for this type and backend.
    #[inline]
    fn from_str(raw: impl AsRef<str>) -> Result<Self, Error<T::Error>>
    where
        Self: Sized,
    {
        match T::from_str(raw) {
            Ok(res) => Ok(res),
            Err(err) => Err(Error::DeserializeString { what: type_name::<T::Target>(), err }),
        }
    }

    /// Deserializes this object from the given reader.
    ///
    /// # Arguments
    /// - `reader`: The [`Read`]er that provides a serialized instantiation of
    ///   Self.
    ///
    /// # Returns
    /// A new Self that represents the deserialized counterpart.
    ///
    /// # Errors
    /// This function may error with an [`Error::DeserializeString`] if the
    /// backend deserializer failed to deserialize. This may be because the
    /// backend failed to read from the given `reader`, or because the
    /// serialized representation was illegal for this type and backend.
    #[inline]
    fn from_reader(reader: impl Read) -> Result<Self, Error<T::Error>>
    where
        Self: Sized,
    {
        match T::from_reader(reader) {
            Ok(res) => Ok(res),
            Err(err) => Err(Error::DeserializeString { what: type_name::<T::Target>(), err }),
        }
    }



    // Convenience functions
    /// Convenience function for serializing this object to a file.
    ///
    /// # Arguments
    /// - `path`: The path to serialize this object to.
    ///
    /// # Errors
    /// This function may error with an [`Error::FileCreate`] if it failed to
    /// create a new file, or an [`Error::SerializeFile`] if the
    /// backend serializer failed to serialize. This may also be because it
    /// failed to write to the file.
    #[inline]
    fn to_path(&self, path: impl AsRef<Path>) -> Result<(), Error<T::Error>> {
        // Open the file as a writer
        let path: &Path = path.as_ref();
        let handle: File = match File::create(path) {
            Ok(handle) => handle,
            Err(err) => {
                return Err(Error::FileCreate { path: path.into(), err });
            },
        };

        // Pass to the writer impl
        match self.to_writer(handle) {
            Ok(_) => Ok(()),
            Err(Error::SerializeWriter { what, err }) => Err(Error::SerializeFile { what, path: path.into(), err }),
            Err(err) => Err(err),
        }
    }
    /// Convenience function for serializing this object to a file, using a pretty backend if it's available.
    ///
    /// If not, then this is equivalent to calling
    /// [`Self::to_path()`](Serializable::to_path()).
    ///
    /// # Arguments
    /// - `path`: The path to serialize this object to.
    ///
    /// # Errors
    /// This function may error with an [`Error::FileCreate`] if it failed to
    /// create a new file, or an [`Error::SerializeFile`] if the
    /// backend serializer failed to serialize. This may also be because it
    /// failed to write to the file.
    #[inline]
    fn to_path_pretty(&self, path: impl AsRef<Path>) -> Result<(), Error<T::Error>> {
        // Open the file as a writer
        let path: &Path = path.as_ref();
        let handle: File = match File::create(path) {
            Ok(handle) => handle,
            Err(err) => {
                return Err(Error::FileCreate { path: path.into(), err });
            },
        };

        // Pass to the writer impl
        match self.to_writer_pretty(handle) {
            Ok(_) => Ok(()),
            Err(Error::SerializeWriter { what, err }) => Err(Error::SerializeFile { what, path: path.into(), err }),
            Err(err) => Err(err),
        }
    }
    /// Convenience function for deserializing this object from a file.
    ///
    /// # Arguments
    /// - `path`: The path to deserialize this object from.
    ///
    /// # Returns
    /// A new Self that represents the deserialized object.
    ///
    /// # Errors
    /// This function may error with an [`Error::FileOpen`] if it failed to
    /// create a new file, or an [`Error::DeserializeFile`] if the
    /// backend serializer failed to serialize. This may also be because it
    /// failed to write to the file.
    #[inline]
    fn from_path(path: impl AsRef<Path>) -> Result<Self, Error<T::Error>>
    where
        Self: Sized,
    {
        // Open the file as a writer
        let path: &Path = path.as_ref();
        let handle: File = match File::open(path) {
            Ok(handle) => handle,
            Err(err) => {
                return Err(Error::FileOpen { path: path.into(), err });
            },
        };

        // Pass to the reader impl
        match Self::from_reader(handle) {
            Ok(res) => Ok(res),
            Err(Error::DeserializeReader { what, err }) => Err(Error::DeserializeFile { what, path: path.into(), err }),
            Err(err) => Err(err),
        }
    }

    /// Convenience function for serializing this object to a string using
    /// dynamic prettyness.
    ///
    /// # Arguments
    /// - `pretty`: Whether to use the pretty formatter or not.
    ///
    /// # Returns
    /// A string representing this object.
    ///
    /// # Errors
    /// This function may error with an [`Error::SerializeString`] if the
    /// backend serializer failed to serialize.
    #[inline]
    fn to_string_pretty_opt(&self, pretty: bool) -> Result<String, Error<T::Error>> {
        if pretty { self.to_string_pretty() } else { self.to_string() }
    }
    /// Convenience function for serializing this object to a writer using
    /// dynamic prettyness.
    ///
    /// # Arguments
    /// - `writer`: A [`Write`]r that we will serialize to.
    /// - `pretty`: Whether to use the pretty formatter or not.
    ///
    /// # Errors
    /// This function may error with an [`Error::SerializeWriter`] if the
    /// backend serializer failed to serialize. This may also be because it
    /// failed to write to the `writer`.
    #[inline]
    fn to_writer_pretty_opt(&self, writer: impl Write, pretty: bool) -> Result<(), Error<T::Error>> {
        if pretty { self.to_writer_pretty(writer) } else { self.to_writer(writer) }
    }
    /// Convenience function for serializing this object to a path using
    /// dynamic prettyness.
    ///
    /// # Arguments
    /// - `path`: The path to serialize this object to.
    /// - `pretty`: Whether to use the pretty formatter or not.
    ///
    /// # Errors
    /// This function may error with an [`Error::FileCreate`] if it failed to
    /// create a new file, or an [`Error::SerializeFile`] if the
    /// backend serializer failed to serialize. This may also be because it
    /// failed to write to the file.
    #[inline]
    fn to_path_pretty_opt(&self, path: impl AsRef<Path>, pretty: bool) -> Result<(), Error<T::Error>> {
        if pretty { self.to_path_pretty(path) } else { self.to_path(path) }
    }
}
