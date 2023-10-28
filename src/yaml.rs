//  YAML.rs
//    by Lut99
//
//  Created:
//    28 Oct 2023, 13:04:05
//  Last edited:
//    28 Oct 2023, 13:07:35
//  Auto updated?
//    Yes
//
//  Description:
//!   Implements [`serializer::Serializer`] and cohorts for [`serde_yaml`].
//

use std::io::{Read, Write};
use std::marker::PhantomData;

use serde::{Deserialize, Serialize};

use crate::serializer;


/***** LIBRARY *****/
/// Implements a [`serializer::Serializer`] for [`serde_yaml`].
///
/// Note that this serializer has no pretty version available. As such,
/// [`serializer::Serializer::to_string_pretty()`] and [`serializer::Serializer::to_writer_pretty()`]
/// are simply aliases for [`serializer::Serializer::to_string()`] and
/// [`serializer::Serializer::to_writer()`], respectively.
#[derive(Clone, Copy, Debug)]
pub struct Serializer<T>(PhantomData<T>);

impl<T: for<'de> Deserialize<'de> + Serialize> serializer::Serializer for Serializer<T> {
    type Error = serde_yaml::Error;
    type Target = T;

    #[inline]
    fn to_string(value: &Self::Target) -> Result<String, Self::Error> { serde_yaml::to_string(value) }

    #[inline]
    fn to_writer(value: &Self::Target, writer: impl Write) -> Result<(), Self::Error> {
        serde_yaml::to_writer(writer, value)
    }

    #[inline]
    fn from_str(raw: impl AsRef<str>) -> Result<Self::Target, Self::Error> { serde_yaml::from_str(raw.as_ref()) }

    #[inline]
    fn from_reader(reader: impl Read) -> Result<Self::Target, Self::Error> { serde_yaml::from_reader(reader) }
}
