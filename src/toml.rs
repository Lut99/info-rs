//  TOML.rs
//    by Lut99
//
//  Created:
//    28 Oct 2023, 13:05:57
//  Last edited:
//    28 Oct 2023, 13:12:31
//  Auto updated?
//    Yes
//
//  Description:
//!   Implements [`serializer::Serializer`] and cohorts for [`toml`].
//

use std::error::Error;
use std::fmt::{Display, Formatter, Result as FResult};
use std::io::{Read, Write};
use std::marker::PhantomData;

use serde::{Deserialize, Serialize};

use crate::serializer;


/***** ERRORS *****/
/// Defines errors that occur when using the [`TomlSerializer`].
#[cfg(feature = "serde-toml")]
#[derive(Debug)]
pub enum TomlError {
    /// Failed to write to the given writer.
    Write { err: std::io::Error },
    /// Failed to read from the given reader.
    Read { err: std::io::Error },
    /// Failed to serialize the object to TOML.
    Serialize { err: toml::ser::Error },
    /// Failed to deserialize the object from TOML.
    Deserialize { err: toml::de::Error },
}
#[cfg(feature = "serde-toml")]
impl Display for TomlError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        use TomlError::*;
        match self {
            Write { .. } => write!(f, "Failed to write to given writer"),
            Read { .. } => write!(f, "Failed to read from given reader"),
            Serialize { .. } => write!(f, "Failed to serialize to TOML"),
            Deserialize { .. } => write!(f, "Failed to deserialize from TOML"),
        }
    }
}
#[cfg(feature = "serde-toml")]
impl Error for TomlError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        use TomlError::*;
        match self {
            Write { err } => Some(err),
            Read { err } => Some(err),
            Serialize { err } => Some(err),
            Deserialize { err } => Some(err),
        }
    }
}





/***** LIBRARY *****/
/// Implements a [`serializer::Serializer`] for [`toml`].
#[derive(Clone, Copy, Debug)]
pub struct TomlSerializer<T>(PhantomData<T>);

impl<T: for<'de> Deserialize<'de> + Serialize> serializer::Serializer for TomlSerializer<T> {
    type Error = TomlError;
    type Target = T;

    #[inline]
    fn to_string(value: &Self::Target) -> Result<String, Self::Error> { toml::to_string(value).map_err(|err| TomlError::Serialize { err }) }

    #[inline]
    fn to_string_pretty(value: &Self::Target) -> Result<String, Self::Error> {
        toml::to_string_pretty(value).map_err(|err| TomlError::Serialize { err })
    }

    #[inline]
    fn to_writer(value: &Self::Target, mut writer: impl Write) -> Result<(), Self::Error> {
        // Write to string first
        let raw: String = match toml::to_string(value) {
            Ok(raw) => raw,
            Err(err) => return Err(TomlError::Serialize { err }),
        };

        // Then write to the writer
        match writer.write_all(raw.as_bytes()) {
            Ok(_) => Ok(()),
            Err(err) => Err(TomlError::Write { err }),
        }
    }

    #[inline]
    fn to_writer_pretty(value: &Self::Target, mut writer: impl Write) -> Result<(), Self::Error> {
        // Write to string first
        let raw: String = match toml::to_string_pretty(value) {
            Ok(raw) => raw,
            Err(err) => return Err(TomlError::Serialize { err }),
        };

        // Then write to the writer
        match writer.write_all(raw.as_bytes()) {
            Ok(_) => Ok(()),
            Err(err) => Err(TomlError::Write { err }),
        }
    }

    #[inline]
    fn from_str(raw: impl AsRef<str>) -> Result<Self::Target, Self::Error> {
        toml::from_str(raw.as_ref()).map_err(|err| TomlError::Deserialize { err })
    }

    #[inline]
    fn from_reader(mut reader: impl Read) -> Result<Self::Target, Self::Error> {
        // Simply read the whole reader
        let mut raw: String = String::new();
        if let Err(err) = reader.read_to_string(&mut raw) {
            return Err(TomlError::Read { err });
        }

        // Now deserialize using the string edition
        match toml::from_str(&raw) {
            Ok(res) => Ok(res),
            Err(err) => Err(TomlError::Deserialize { err }),
        }
    }
}
