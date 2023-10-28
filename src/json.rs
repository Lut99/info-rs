//  JSON.rs
//    by Lut99
//
//  Created:
//    28 Oct 2023, 13:02:40
//  Last edited:
//    28 Oct 2023, 13:11:30
//  Auto updated?
//    Yes
//
//  Description:
//!   Implements [`serializer::Serializer`] and cohorts for [`serde_json`].
//

use std::io::{Read, Write};
use std::marker::PhantomData;

use serde::{Deserialize, Serialize};

use crate::serializer;


/***** LIBRARY *****/
/// Implements a [`serializer::Serializer`] for [`serde_json`].
#[derive(Clone, Copy, Debug)]
pub struct Serializer<T>(PhantomData<T>);

impl<T: for<'de> Deserialize<'de> + Serialize> serializer::Serializer for Serializer<T> {
    type Error = serde_json::Error;
    type Target = T;

    #[inline]
    fn to_string(value: &Self::Target) -> Result<String, Self::Error> { serde_json::to_string(value) }

    #[inline]
    fn to_string_pretty(value: &Self::Target) -> Result<String, Self::Error> { serde_json::to_string_pretty(value) }

    #[inline]
    fn to_writer(value: &Self::Target, writer: impl Write) -> Result<(), Self::Error> { serde_json::to_writer(writer, value) }

    #[inline]
    fn to_writer_pretty(value: &Self::Target, writer: impl Write) -> Result<(), Self::Error> { serde_json::to_writer_pretty(writer, value) }

    #[inline]
    fn from_str(raw: impl AsRef<str>) -> Result<Self::Target, Self::Error> { serde_json::from_str(raw.as_ref()) }

    #[inline]
    fn from_reader(reader: impl Read) -> Result<Self::Target, Self::Error> { serde_json::from_reader(reader) }
}
