//  LIB.rs
//    by Lut99
// 
//  Created:
//    24 Oct 2023, 22:36:08
//  Last edited:
//    25 Oct 2023, 22:20:54
//  Auto updated?
//    Yes
// 
//  Description:
//!   <Todo>
// 

use std::error;
use std::fmt::{Debug, Display, Formatter, Result as FResult};
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};


/***** ERRORS *****/
/// Defines errors that originate from [`Info`]s.
/// 
/// See the functions of [`Info`] for examples when these errors may occur.
#[derive(Debug)]
pub enum Error<E: Debug> {
    /// Failed to create the output file.
    FileCreate { path: PathBuf, err: std::io::Error },
    /// Failed to open the input file.
    FileOpen { path: PathBuf, err: std::io::Error },
    /// Failed to write the output file.
    FileWrite { path: PathBuf, err: std::io::Error },
    /// Failed to flush the output file.
    FileFlush { path: PathBuf, err: std::io::Error },
    /// Failed to read the input file.
    FileRead { path: PathBuf, err: std::io::Error },
    /// Failed to serialize the config to a given file.
    FileSerialize { path: PathBuf, err: E },
    /// Failed to deserialize a file to the config.
    FileDeserialize { path: PathBuf, err: E },

    /// Failed to serialize the config to a given writer.
    WriterSerialize { err: E },
    /// Failed to deserialize a reader to the config.
    ReaderDeserialize { err: E },

    /// Failed to serialize the config to a string.
    StringSerialize { err: E },
    /// Failed to deserialize a string to the config.
    StringDeserialize { err: E },
}
impl<E: error::Error> Display for Error<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        use Error::*;
        match self {
            FileCreate { path, .. }     => write!(f, "Failed to create output file '{}'", path.display()),
            FileOpen { path, .. }       => write!(f, "Failed to open input file '{}'", path.display()),
            FileWrite { path, .. }      => write!(f, "Failed to write output file '{}'", path.display()),
            FileFlush { path, .. }      => write!(f, "Failed to flush output file '{}'", path.display()),
            FileRead { path, .. }       => write!(f, "Failed to read input file '{}'", path.display()),
            FileSerialize { path, .. }  => write!(f, "Failed to serialize to output file '{}'", path.display()),
            FileDeserialize{ path, .. } => write!(f, "Failed to deserialize from input file '{}'", path.display()),

            WriterSerialize { .. }  => write!(f, "Failed to serialize to a writer"),
            ReaderDeserialize{ .. } => write!(f, "Failed to deserialize from a reader"),
            
            StringSerialize { .. }  => write!(f, "Failed to serialize to string"),
            StringDeserialize{ .. } => write!(f, "Failed to deserialize from string"),
        }
    }
}
impl<E: 'static + error::Error> error::Error for Error<E> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        use Error::*;
        match self {
            FileCreate { err, .. }      => Some(err),
            FileOpen { err, .. }        => Some(err),
            FileWrite { err, .. }       => Some(err),
            FileFlush { err, .. }       => Some(err),
            FileRead { err, .. }        => Some(err),
            FileSerialize { err, .. }   => Some(err),
            FileDeserialize { err, .. } => Some(err),

            WriterSerialize { err }   => Some(err),
            ReaderDeserialize { err } => Some(err),

            StringSerialize { err }   => Some(err),
            StringDeserialize { err } => Some(err),
        }
    }
}





/***** LIBRARY *****/
/// Defines a serializable struct that we typically use for structs that are directly read and written to disk.
/// 
/// While you can implement this trait yourself, it's much more efficient to enable one of the auto-implementation features like `serde-json` or `serde-yaml` and implement this trait automatically using an alias.
/// 
/// Also see the `async-tokio`-feature for an async counterpart w.r.t. path deserialization/serialization.
/// 
/// # Example
/// ```rust
/// # use std::fs;
/// # use std::io::{Read, Write};
/// use info::Info;
/// 
/// #[derive(Debug)]
/// enum HelloWorldError {
/// #     Writer { err: std::io::Error },
/// #     Reader { err: std::io::Error },
/// #     Illegal { raw: String },
///     // ...
/// };
/// impl std::fmt::Display for HelloWorldError {
/// #     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
/// #         match self {
/// #             Self::Writer { err }  => write!(f, "Failed to read from given reader: {err}"),
/// #             Self::Reader { err }  => write!(f, "Failed to read from given reader: {err}"),
/// #             Self::Illegal { raw } => write!(f, "Given input '{raw}' is not 'hello_world' or 'Hello, world!'"),
/// #         }
/// #     }
///     // ...
/// }
/// impl std::error::Error for HelloWorldError {}
/// 
/// #[derive(Debug, Eq, PartialEq)]
/// struct HelloWorld;
/// impl Info for HelloWorld {
///     type Error = HelloWorldError;
/// 
///     fn to_string(&self, pretty: bool) -> Result<String, info::Error<Self::Error>> {
///         if pretty {
///             Ok("Hello, world!".into())
///         } else {
///             Ok("hello_world".into())
///         }
///     }
/// 
///     fn to_writer(&self, mut writer: impl Write, _pretty: bool) -> Result<(), info::Error<Self::Error>> {
///         writer.write_all("Hello, world!".as_bytes())
///             .map_err(|err| info::Error::WriterSerialize { err: HelloWorldError::Writer { err } })
///     }
/// 
///     fn from_string(raw: impl AsRef<str>) -> Result<Self, info::Error<Self::Error>> {
///         let raw: &str = raw.as_ref();
///         if raw == "hello_world" || raw == "Hello, world!" {
///             Ok(Self)
///         } else {
///             Err(info::Error::StringDeserialize { err: HelloWorldError::Illegal { raw: raw.into() } })
///         }
///     }
/// 
///     fn from_reader(mut reader: impl Read) -> Result<Self, info::Error<Self::Error>> {
///         // Attempt to read the string
///         let mut buf: [u8; 13] = [0; 13];
///         if let Err(err) = reader.read_exact(&mut buf) { return Err(info::Error::ReaderDeserialize { err: HelloWorldError::Reader { err } }); }
/// 
///         // Match it
///         if String::from_utf8_lossy(&buf) == "Hello, world!" {
///             Ok(Self)
///         } else {
///             Err(info::Error::ReaderDeserialize { err: HelloWorldError::Illegal { raw: String::from_utf8_lossy(&buf).into() } })
///         }
///     }
/// }
/// 
/// assert_eq!(HelloWorld::from_string(HelloWorld.to_string(true).unwrap()).unwrap(), HelloWorld);
/// assert_eq!(HelloWorld::from_string(HelloWorld.to_string(false).unwrap()).unwrap(), HelloWorld);
/// 
/// let path = std::env::temp_dir().join("example.txt");
/// HelloWorld.to_path(&path, true).unwrap();
/// assert_eq!(fs::read_to_string(path).unwrap(), "Hello, world!");
/// 
/// let path = std::env::temp_dir().join("example.txt");
/// fs::write(&path, "Hello, world!").unwrap();
/// assert_eq!(HelloWorld::from_path(path).unwrap(), HelloWorld);
/// ```
pub trait Info: Sized {
    /// The types of errors that may be thrown by the serialization function(s).
    /// 
    /// Note that this error is always embedded in the main [`Error`].
    type Error : error::Error;


    // Child-provided
    /// Serializes this Info to a string.
    /// 
    /// # Arguments
    /// - `pretty`: If true, then it will be serialized using a pretty version of the backend (if available).
    /// 
    /// # Returns
    /// A new String that represents this info but serialized.
    /// 
    /// # Errors
    /// This function may error if the serialization failed.
    /// 
    /// # Example
    /// ```rust
    /// use info::Info;
    /// 
    /// struct HelloWorld;
    /// impl Info for HelloWorld {
    ///     type Error = std::convert::Infallible;
    /// 
    ///     fn to_string(&self, pretty: bool) -> Result<String, info::Error<Self::Error>> {
    ///         if pretty {
    ///             Ok("Hello, world!".into())
    ///         } else {
    ///             Ok("hello_world".into())
    ///         }
    ///     }
    /// 
    ///     // ...
    /// #     fn to_writer(&self, _writer: impl std::io::Write, _pretty: bool) -> Result<(), info::Error<Self::Error>> {
    /// #         todo!();
    /// #     }
    /// #     fn from_string(_raw: impl AsRef<str>) -> Result<Self, info::Error<Self::Error>> {
    /// #         todo!();
    /// #     }
    /// #     fn from_reader(_reader: impl std::io::Read) -> Result<Self, info::Error<Self::Error>> {
    /// #         todo!();
    /// #     }
    /// }
    /// 
    /// assert_eq!(HelloWorld.to_string(true).unwrap(), "Hello, world!");
    /// assert_eq!(HelloWorld.to_string(false).unwrap(), "hello_world");
    /// ```
    fn to_string(&self, pretty: bool) -> Result<String, Error<Self::Error>>;
    /// Serializes this Info to a given writer.
    /// 
    /// # Arguments
    /// - `writer`: The [`Write`]r to write the serialized representation to.
    /// - `pretty`: If true, then it will be serialized using a pretty version of the backend (if available).
    /// 
    /// # Errors
    /// This function may error if the serialization failed or if we failed to write to the given writer.
    /// 
    /// # Example
    /// ```rust
    /// # use std::io::Write;
    /// use info::Info;
    /// 
    /// struct HelloWorld;
    /// impl Info for HelloWorld {
    ///     type Error = std::io::Error;
    /// 
    ///     fn to_writer(&self, mut writer: impl Write, pretty: bool) -> Result<(), info::Error<Self::Error>> {
    ///         if pretty {
    ///             writer.write_all("Hello, world!".as_bytes())
    ///                 .map_err(|err| info::Error::WriterSerialize { err })
    ///         } else {
    ///             writer.write_all("hello_world".as_bytes())
    ///                 .map_err(|err| info::Error::WriterSerialize { err })
    ///         }
    ///     }
    /// 
    ///     // ...
    /// #     fn to_string(&self, _pretty: bool) -> Result<String, info::Error<Self::Error>> {
    /// #         todo!();
    /// #     }
    /// #     fn from_string(_raw: impl AsRef<str>) -> Result<Self, info::Error<Self::Error>> {
    /// #         todo!();
    /// #     }
    /// #     fn from_reader(_reader: impl std::io::Read) -> Result<Self, info::Error<Self::Error>> {
    /// #         todo!();
    /// #     }
    /// }
    /// 
    /// let mut buf: [u8; 13] = [0; 13];
    /// HelloWorld.to_writer(&mut buf[..], true).unwrap();
    /// assert_eq!(String::from_utf8_lossy(&buf), "Hello, world!");
    /// 
    /// let mut buf: [u8; 11] = [0; 11];
    /// HelloWorld.to_writer(&mut buf[..], false).unwrap();
    /// assert_eq!(String::from_utf8_lossy(&buf), "hello_world");
    /// 
    /// let mut buf: [u8; 0] = [];
    /// assert!(matches!(HelloWorld.to_writer(&mut buf[..], false), Err(info::Error::WriterSerialize { .. })));
    /// ```
    fn to_writer(&self, writer: impl Write, pretty: bool) -> Result<(), Error<Self::Error>>;

    /// Deserializes the given string to an instance of ourselves.
    /// 
    /// # Arguments
    /// - `raw`: The raw string to deserialize.
    /// 
    /// # Returns
    /// A new instance of `Self` with its contents read from the given raw string.
    /// 
    /// # Errors
    /// This function may fail if the input string was invalid for this object.
    /// 
    /// # Example
    /// ```rust
    /// use info::Info;
    /// 
    /// #[derive(Debug)]
    /// struct HelloWorldError(String);
    /// impl std::fmt::Display for HelloWorldError {
    ///     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ///         write!(f, "Given input '{}' is not 'hello_world' or 'Hello, world!'", self.0)
    ///     }
    /// }
    /// impl std::error::Error for HelloWorldError {}
    /// 
    /// #[derive(Debug, Eq, PartialEq)]
    /// struct HelloWorld;
    /// impl Info for HelloWorld {
    ///     type Error = HelloWorldError;
    /// 
    ///     fn from_string(raw: impl AsRef<str>) -> Result<Self, info::Error<Self::Error>> {
    ///         let raw: &str = raw.as_ref();
    ///         if raw == "hello_world" || raw == "Hello, world!" {
    ///             Ok(Self)
    ///         } else {
    ///             Err(info::Error::StringDeserialize { err: HelloWorldError(raw.into()) })
    ///         }
    ///     }
    /// 
    ///     // ...
    /// #     fn to_string(&self, _pretty: bool) -> Result<String, info::Error<Self::Error>> {
    /// #         todo!();
    /// #     }
    /// #     fn to_writer(&self, _writer: impl std::io::Write, _pretty: bool) -> Result<(), info::Error<Self::Error>> {
    /// #         todo!();
    /// #     }
    /// #     fn from_reader(_reader: impl std::io::Read) -> Result<Self, info::Error<Self::Error>> {
    /// #         todo!();
    /// #     }
    /// }
    /// 
    /// assert_eq!(HelloWorld::from_string("hello_world").unwrap(), HelloWorld);
    /// assert_eq!(HelloWorld::from_string("Hello, world!").unwrap(), HelloWorld);
    /// assert!(matches!(HelloWorld::from_string("foo"), Err(info::Error::StringDeserialize { err: HelloWorldError(_) })));
    /// ```
    fn from_string(raw: impl AsRef<str>) -> Result<Self, Error<Self::Error>>;
    /// Deserializes the contents of the given reader to an instance of ourselves.
    /// 
    /// # Arguments
    /// - `reader`: The [`Read`]er who's contents to deserialize.
    /// 
    /// # Returns
    /// A new instance of `Self` with its contents read from the given reader.
    /// 
    /// # Errors
    /// This function may fail if we failed to read from the reader or if its contents were invalid for this object.
    /// 
    /// # Example
    /// ```rust
    /// # use std::io::Read;
    /// use info::Info;
    /// 
    /// #[derive(Debug)]
    /// enum HelloWorldError {
    ///     Reader { err: std::io::Error },
    ///     Illegal { raw: String },
    /// };
    /// impl std::fmt::Display for HelloWorldError {
    ///     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ///         match self {
    ///             Self::Reader { err }  => write!(f, "Failed to read from given reader: {err}"),
    ///             Self::Illegal { raw } => write!(f, "Given input '{raw}' is not 'hello_world' or 'Hello, world!'"),
    ///         }
    ///     }
    /// }
    /// impl std::error::Error for HelloWorldError {}
    /// 
    /// #[derive(Debug, Eq, PartialEq)]
    /// struct HelloWorld;
    /// impl Info for HelloWorld {
    ///     type Error = HelloWorldError;
    /// 
    ///     fn from_reader(mut reader: impl Read) -> Result<Self, info::Error<Self::Error>> {
    ///         // Attempt to read the string
    ///         let mut buf: [u8; 13] = [0; 13];
    ///         if let Err(err) = reader.read_exact(&mut buf) { return Err(info::Error::ReaderDeserialize { err: HelloWorldError::Reader { err } }); }
    /// 
    ///         // Match it
    ///         if String::from_utf8_lossy(&buf) == "Hello, world!" {
    ///             Ok(Self)
    ///         } else {
    ///             Err(info::Error::ReaderDeserialize { err: HelloWorldError::Illegal { raw: String::from_utf8_lossy(&buf).into() } })
    ///         }
    ///     }
    /// 
    ///     // ...
    /// #     fn to_string(&self, _pretty: bool) -> Result<String, info::Error<Self::Error>> {
    /// #         todo!();
    /// #     }
    /// #     fn to_writer(&self, _writer: impl std::io::Write, _pretty: bool) -> Result<(), info::Error<Self::Error>> {
    /// #         todo!();
    /// #     }
    /// #     fn from_string(_raw: impl AsRef<str>) -> Result<Self, info::Error<Self::Error>> {
    /// #         todo!();
    /// #     }
    /// }
    /// 
    /// assert_eq!(HelloWorld::from_reader("Hello, world!".as_bytes()).unwrap(), HelloWorld);
    /// assert!(matches!(HelloWorld::from_reader("foo".as_bytes()), Err(info::Error::ReaderDeserialize { err: HelloWorldError::Reader { .. } })));
    /// assert!(matches!(HelloWorld::from_reader("foofoofoofoof".as_bytes()), Err(info::Error::ReaderDeserialize { err: HelloWorldError::Illegal { .. } })));
    /// ```
    fn from_reader(reader: impl Read) -> Result<Self, Error<Self::Error>>;


    // Globally deduced
    /// Serializes this Info to a file at the given path.
    /// 
    /// # Arguments
    /// - `path`: The path where to write the file to.
    /// - `pretty`: If true, then it will be serialized using a pretty version of the backend (if available).
    /// 
    /// # Errors
    /// This function may error if the serialization failed or if we failed to create and/or write to the file.
    /// 
    /// # Example
    /// ```rust
    /// # use std::fs;
    /// use info::Info;
    /// 
    /// struct HelloWorld;
    /// impl Info for HelloWorld {
    ///     // ...
    /// #     type Error = std::io::Error;
    /// #     fn to_string(&self, _pretty: bool) -> Result<String, info::Error<Self::Error>> {
    /// #         todo!();
    /// #     }
    /// #     fn to_writer(&self, mut writer: impl std::io::Write, _pretty: bool) -> Result<(), info::Error<Self::Error>> {
    /// #         writer.write_all("Hello, world!".as_bytes())
    /// #             .map_err(|err| info::Error::WriterSerialize { err })
    /// #     }
    /// #     fn from_string(_raw: impl AsRef<str>) -> Result<Self, info::Error<Self::Error>> {
    /// #         todo!();
    /// #     }
    /// #     fn from_reader(_reader: impl std::io::Read) -> Result<Self, info::Error<Self::Error>> {
    /// #         todo!();
    /// #     }
    /// }
    /// 
    /// let path = std::env::temp_dir().join("example.txt");
    /// HelloWorld.to_path(&path, true).unwrap();
    /// assert_eq!(fs::read_to_string(path).unwrap(), "Hello, world!");
    /// ```
    fn to_path(&self, path: impl AsRef<Path>, pretty: bool) -> Result<(), Error<Self::Error>> {
        let path: &Path = path.as_ref();

        // Attempt to create the new file
        let handle: File = match File::create(path) {
            Ok(handle) => handle,
            Err(err)   => { return Err(Error::FileCreate { path: path.into(), err }); },
        };

        // Write it using the child function, wrapping the error that may occur
        match self.to_writer(handle, pretty) {
            Ok(_)                               => Ok(()),
            Err(Error::WriterSerialize { err }) => Err(Error::FileSerialize { path: path.into(), err }),
            Err(err)                            => Err(err),
        }
    }

    /// Deserializes this Config from the file at the given path.
    /// 
    /// # Arguments
    /// - `path`: The path where to read the file from.
    /// 
    /// # Returns
    /// A new instance of `Self` with its contents read from the given file.
    /// 
    /// # Errors
    /// This function may fail if we failed to open/read from the file or if its contents were invalid for this object.
    /// 
    /// # Example
    /// ```rust
    /// # use std::fs;
    /// use info::Info;
    /// 
    /// #[derive(Debug, Eq, PartialEq)]
    /// struct HelloWorld;
    /// impl Info for HelloWorld {
    ///     // ...
    /// #     type Error = std::io::Error;
    /// #     fn to_string(&self, _pretty: bool) -> Result<String, info::Error<Self::Error>> {
    /// #         todo!();
    /// #     }
    /// #     fn to_writer(&self, _writer: impl std::io::Write, _pretty: bool) -> Result<(), info::Error<Self::Error>> {
    /// #         todo!();
    /// #     }
    /// #     fn from_string(_raw: impl AsRef<str>) -> Result<Self, info::Error<Self::Error>> {
    /// #         todo!();
    /// #     }
    /// #     fn from_reader(mut reader: impl std::io::Read) -> Result<Self, info::Error<Self::Error>> {
    /// #         // Attempt to read the string
    /// #         let mut buf: [u8; 13] = [0; 13];
    /// #         if let Err(err) = reader.read_exact(&mut buf) { return Err(info::Error::ReaderDeserialize { err }); }
    /// # 
    /// #         // Match it
    /// #         if String::from_utf8_lossy(&buf) == "Hello, world!" {
    /// #             Ok(Self)
    /// #         } else {
    /// #             Err(info::Error::ReaderDeserialize { err: std::io::Error::from(std::io::ErrorKind::Other) })
    /// #         }
    /// #     }
    /// }
    /// 
    /// let path = std::env::temp_dir().join("example.txt");
    /// fs::write(&path, "Hello, world!").unwrap();
    /// assert_eq!(HelloWorld::from_path(path).unwrap(), HelloWorld);
    /// ```
    fn from_path(path: impl AsRef<Path>) -> Result<Self, Error<Self::Error>> {
        let path: &Path = path.as_ref();

        // Attempt to open the given file
        let handle: File = match File::open(path) {
            Ok(handle) => handle,
            Err(err)   => { return Err(Error::FileOpen { path: path.into(), err }); },
        };

        // Write it using the child function, wrapping the error that may occur
        match Self::from_reader(handle) {
            Ok(config)                            => Ok(config),
            Err(Error::ReaderDeserialize { err }) => Err(Error::FileDeserialize { path: path.into(), err }),
            Err(err)                              => Err(err),
        }
    }
}



/// Asynchronous version of the [`Info`] that uses [`tokio`] for the asynchronous file reading.
/// 
/// Note that the serde part is actually never asynchronous, so there's no need to re-implement anything; only the path-parts are automatically implemented.
/// 
/// # Example
/// ```rust
/// # use std::fs;
/// use info::{Info, InfoAsync as _};
/// 
/// #[derive(Debug, Eq, PartialEq)]
/// struct HelloWorld;
/// impl Info for HelloWorld {
///     // ...
/// #     type Error = std::io::Error;
/// #     fn to_string(&self, _pretty: bool) -> Result<String, info::Error<Self::Error>> {
/// #         Ok("Hello, world!".into())
/// #     }
/// #     fn to_writer(&self, mut writer: impl std::io::Write, _pretty: bool) -> Result<(), info::Error<Self::Error>> {
/// #         todo!();
/// #     }
/// #     fn from_string(raw: impl AsRef<str>) -> Result<Self, info::Error<Self::Error>> {
/// #         if raw.as_ref() == "Hello, world!" {
/// #             Ok(Self)
/// #         } else {
/// #             panic!("Input is not 'Hello, world!'");
/// #         }
/// #     }
/// #     fn from_reader(mut reader: impl std::io::Read) -> Result<Self, info::Error<Self::Error>> {
/// #         todo!();
/// #     }
/// }
/// 
/// # tokio_test::block_on(async {
/// let path = std::env::temp_dir().join("example1.txt");
/// HelloWorld.to_path_async(&path, true).await.unwrap();
/// assert_eq!(fs::read_to_string(path).unwrap(), "Hello, world!");
/// 
/// let path = std::env::temp_dir().join("example2.txt");
/// fs::write(&path, "Hello, world!").unwrap();
/// assert_eq!(HelloWorld::from_path_async(path).await.unwrap(), HelloWorld);
/// # });
/// ```
#[cfg(feature = "async-tokio")]
#[async_trait::async_trait]
pub trait InfoAsync: Info {
    // Globally deduced
    /// Serializes this Info to a file at the given path, with the writing part done asynchronously.
    /// 
    /// Note that the parsing-part cannot be done asynchronously.
    /// 
    /// # Arguments
    /// - `path`: The path where to write the file to.
    /// - `pretty`: If true, then it will be serialized using a pretty version of the backend (if available).
    /// 
    /// # Returns
    /// A new instance of `Self` with its contents read from the given file.
    /// 
    /// # Errors
    /// This function may error if the serialization failed or if we failed to create and/or write to the file.
    /// 
    /// # Example
    /// ```rust
    /// # use std::fs;
    /// use info::{Info, InfoAsync as _};
    /// 
    /// struct HelloWorld;
    /// impl Info for HelloWorld {
    ///     // ...
    /// #     type Error = std::io::Error;
    /// #     fn to_string(&self, _pretty: bool) -> Result<String, info::Error<Self::Error>> {
    /// #         Ok("Hello, world!".into())
    /// #     }
    /// #     fn to_writer(&self, _writer: impl std::io::Write, _pretty: bool) -> Result<(), info::Error<Self::Error>> {
    /// #         todo!();
    /// #     }
    /// #     fn from_string(_raw: impl AsRef<str>) -> Result<Self, info::Error<Self::Error>> {
    /// #         todo!();
    /// #     }
    /// #     fn from_reader(_reader: impl std::io::Read) -> Result<Self, info::Error<Self::Error>> {
    /// #         todo!();
    /// #     }
    /// }
    /// 
    /// # tokio_test::block_on(async {
    /// let path = std::env::temp_dir().join("example.txt");
    /// HelloWorld.to_path_async(&path, true).await.unwrap();
    /// assert_eq!(fs::read_to_string(path).unwrap(), "Hello, world!");
    /// # });
    /// ```
    async fn to_path_async(&self, path: impl Send + AsRef<Path>, pretty: bool) -> Result<(), Error<Self::Error>> {
        let path: &Path = path.as_ref();

        // Serialize ourselves to a string
        let raw: String = match self.to_string(pretty) {
            Ok(raw)                             => raw,
            Err(Error::WriterSerialize { err }) => { return Err(Error::FileSerialize { path: path.into(), err }); },
            Err(err)                            => { return Err(err); },
        };

        // Attempt to create the new file; then write to it
        let mut handle: tokio::fs::File = match tokio::fs::File::create(path).await {
            Ok(handle) => handle,
            Err(err)   => { return Err(Error::FileCreate { path: path.into(), err }); },
        };
        if let Err(err) = <tokio::fs::File as tokio::io::AsyncWriteExt>::write_all(&mut handle, raw.as_bytes()).await {
            return Err(Error::FileWrite { path: path.into(), err });
        }

        // Don't forget to flush the writer!
        if let Err(err) = <tokio::fs::File as tokio::io::AsyncWriteExt>::flush(&mut handle).await {
            return Err(Error::FileFlush { path: path.into(), err });
        }
        drop(handle);

        // Alright, that's it, signing off...
        Ok(())
    }

    /// Deserializes this Config from the file at the given path, with the reading part done asynchronously.
    /// 
    /// Note that the parsing-part cannot be done asynchronously.
    /// 
    /// # Arguments
    /// - `path`: The path where to read the file from.
    /// 
    /// # Errors
    /// This function may fail if we failed to open/read from the file or if its contents were invalid for this object.
    /// 
    /// # Example
    /// ```rust
    /// # use std::fs;
    /// use info::{Info, InfoAsync as _};
    /// 
    /// #[derive(Debug, Eq, PartialEq)]
    /// struct HelloWorld;
    /// impl Info for HelloWorld {
    ///     // ...
    /// #     type Error = std::io::Error;
    /// #     fn to_string(&self, _pretty: bool) -> Result<String, info::Error<Self::Error>> {
    /// #         todo!();
    /// #     }
    /// #     fn to_writer(&self, _writer: impl std::io::Write, _pretty: bool) -> Result<(), info::Error<Self::Error>> {
    /// #         todo!();
    /// #     }
    /// #     fn from_string(raw: impl AsRef<str>) -> Result<Self, info::Error<Self::Error>> {
    /// #         if raw.as_ref() == "Hello, world!" {
    /// #             Ok(Self)
    /// #         } else {
    /// #             panic!("Input is not 'Hello, world!'");
    /// #         }
    /// #     }
    /// #     fn from_reader(_reader: impl std::io::Read) -> Result<Self, info::Error<Self::Error>> {
    /// #         todo!();
    /// #     }
    /// }
    /// 
    /// # tokio_test::block_on(async {
    /// let path = std::env::temp_dir().join("example.txt");
    /// fs::write(&path, "Hello, world!").unwrap();
    /// assert_eq!(HelloWorld::from_path_async(path).await.unwrap(), HelloWorld);
    /// # });
    /// ```
    async fn from_path_async(path: impl Send + AsRef<Path>) -> Result<Self, Error<Self::Error>> {
        let path: &Path = path.as_ref();

        // Read the file to a string
        let raw: String = {
            // Attempt to open the given file
            let mut handle: tokio::fs::File = match tokio::fs::File::open(path).await {
                Ok(handle) => handle,
                Err(err)   => { return Err(Error::FileOpen { path: path.into(), err }); },
            };

            // Read everything to a string
            let mut raw: String = String::new();
            if let Err(err) = <tokio::fs::File as tokio::io::AsyncReadExt>::read_to_string(&mut handle, &mut raw).await { return Err(Error::FileRead { path: path.into(), err }); }
            raw
        };

        // Write it using the child function, wrapping the error that may occur
        match Self::from_string(raw) {
            Ok(config)                            => Ok(config),
            Err(Error::ReaderDeserialize { err }) => Err(Error::FileDeserialize  { path: path.into(), err }),
            Err(err)                              => Err(err),
        }
    }
}
#[cfg(feature = "async-tokio")]
impl<T: Info> InfoAsync for T {}



/// A marker trait that will let the compiler implement [`Info`] for this object using the [`serde_yaml`] backend.
/// 
/// # Example
/// ```rust
/// use serde::{Deserialize, Serialize};
/// use info::{Info as _, JsonInfo};
/// 
/// #[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
/// struct HelloWorld {
///     world: String,
/// }
/// impl<'de> JsonInfo<'de> for HelloWorld {}
/// 
/// let text: &str = r#"
/// {
///     "world": "world"
/// }
/// "#;
/// assert_eq!(HelloWorld::from_string(text).unwrap(), HelloWorld { world: "world".into() });
/// ```
#[cfg(feature = "serde-json")]
pub trait JsonInfo<'de>: serde::Deserialize<'de> + serde::Serialize {}
#[cfg(feature = "serde-json")]
impl<T: serde::de::DeserializeOwned + serde::Serialize + for<'de> JsonInfo<'de>> Info for T {
    type Error = serde_json::Error;


    fn to_string(&self, pretty: bool) -> Result<String, Error<Self::Error>> {
        if pretty {
            match serde_json::to_string(self) {
                Ok(raw)  => Ok(raw),
                Err(err) => Err(Error::StringSerialize { err }),
            }
        } else {
            match serde_json::to_string_pretty(self) {
                Ok(raw)  => Ok(raw),
                Err(err) => Err(Error::StringSerialize { err }),
            }
        }
    }
    fn to_writer(&self, writer: impl Write, pretty: bool) -> Result<(), Error<Self::Error>> {
        if pretty {
            match serde_json::to_writer(writer, self) {
                Ok(raw)  => Ok(raw),
                Err(err) => Err(Error::WriterSerialize { err }),
            }
        } else {
            match serde_json::to_writer_pretty(writer, self) {
                Ok(raw)  => Ok(raw),
                Err(err) => Err(Error::WriterSerialize { err }),
            }
        }
    }

    fn from_string(raw: impl AsRef<str>) -> Result<Self, Error<Self::Error>> {
        match serde_json::from_str(raw.as_ref()) {
            Ok(config) => Ok(config),
            Err(err)   => Err(Error::WriterSerialize { err }),
        }
    }
    fn from_reader(reader: impl Read) -> Result<Self, Error<Self::Error>> {
        match serde_json::from_reader(reader) {
            Ok(config) => Ok(config),
            Err(err)   => Err(Error::WriterSerialize { err }),
        }
    }
}

/// A type alias for the [`Error`] for a [`JsonInfo`].
#[cfg(feature = "serde-json")]
pub type JsonError = Error<serde_json::Error>;



/// A marker trait that will let the compiler implement [`Info`] for this object using the [`serde_yaml`] backend.
/// 
/// # Example
/// ```rust
/// use serde::{Deserialize, Serialize};
/// use info::{Info as _, YamlInfo};
/// 
/// #[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
/// struct HelloWorld {
///     world: String,
/// }
/// impl<'de> YamlInfo<'de> for HelloWorld {}
/// 
/// let text: &str = r#"
/// world: "world"
/// "#;
/// assert_eq!(HelloWorld::from_string(text).unwrap(), HelloWorld { world: "world".into() });
/// ```
#[cfg(feature = "serde-yaml")]
pub trait YamlInfo<'de>: serde::Deserialize<'de> + serde::Serialize {}
#[cfg(feature = "serde-yaml")]
impl<T: serde::de::DeserializeOwned + serde::Serialize + for<'de> YamlInfo<'de>> Info for T {
    type Error = serde_yaml::Error;


    fn to_string(&self, _pretty: bool) -> Result<String, Error<Self::Error>> {
        match serde_yaml::to_string(self) {
            Ok(raw)  => Ok(raw),
            Err(err) => Err(Error::StringSerialize { err }),
        }
    }
    fn to_writer(&self, writer: impl Write, _pretty: bool) -> Result<(), Error<Self::Error>> {
        match serde_yaml::to_writer(writer, self) {
            Ok(raw)  => Ok(raw),
            Err(err) => Err(Error::WriterSerialize { err }),
        }
    }

    fn from_string(raw: impl AsRef<str>) -> Result<Self, Error<Self::Error>> {
        match serde_yaml::from_str(raw.as_ref()) {
            Ok(config) => Ok(config),
            Err(err)   => Err(Error::WriterSerialize { err }),
        }
    }
    fn from_reader(reader: impl Read) -> Result<Self, Error<Self::Error>> {
        match serde_yaml::from_reader(reader) {
            Ok(config) => Ok(config),
            Err(err)   => Err(Error::WriterSerialize { err }),
        }
    }
}

/// A type alias for the [`Error`] for a [`YamlInfo`].
#[cfg(feature = "serde-yaml")]
pub type YamlError = Error<serde_yaml::Error>;
