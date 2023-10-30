# info-rs
Provides a helper trait for [serde](https://serde.rs) types that makes working with them slightly nicer.


## Installation
To use this library, simply add it to your `Cargo.toml` as a [git-dependency](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#specifying-dependencies-from-git-repositories):
```toml
serializer = { git = "https://github.com/Lut99/serializer-rs" }
```

Typically, you will want to enable one of the possible features (see below for a complete list). This can be done using the `features`-key:
```toml
serializer = { git = "https://github.com/Lut99/serializer-rs", features = ["async-tokio", "serde-json"] }
```

You can also optionally commit yourself to a specific version using the `tag`-key:
```toml
serializer = { git = "https://github.com/Lut99/serializer-rs", tag = "v1.0.0" }
```

### Generating documentation
To access the code documentation, either use your IDE's builtin support if you're using [`rust-analyzer`](https://rust-analyzer.github.io/), or build the HTML version yourself using:
```bash
cargo doc --no-deps --open --package serializer
```
in the root of your repository.


## Usage
To use the library, simply implement the `Serializable`-trait for one of your types using a `Serializer` of your choice.

The library provides optional serializers for [`serde_json`](https://github.com/serde-rs/json), [`serde_yaml`](https://github.com/dtolnay/serde-yaml) and [`toml`](https://github.com/toml-rs/toml). There is also a dummy serializer, which doesn't serialize or deserialize any content (mostly used in tests).

For example, if you enabled the `serde-json`-feature, you can:
```rust
use serde::{Deserialize, Serialize};
use serializable::json::Serializer;
use serializable::Serializable;

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
struct HelloWorld {
    hello: String,
    world: String,
}
impl Serializable<Serializer<HelloWorld>> for HelloWorld {}

assert_eq!(
    HelloWorld { hello: "Hello".into(), world: "World".into() }.to_string().unwrap(),
    "{\"hello\":\"Hello\",\"world\":\"World\"}"
);

assert_eq!(
    HelloWorld::from_str("{\"hello\":\"Goodbye\",\"world\":\"Planet\"}").unwrap(),
    HelloWorld { hello: "Goodbye".into(), world: "Planet".into() }
)
```

See the docs for a complete overview of functions in the `Serializer`-trait.

### `async`-API
This library also offers an async API, through the `async-tokio` feature. As the name implies, this uses the [`tokio`](https://tokio.rs/) backend.

This provides the `SerializableAsync`-trait, which implements `std::io::Read`- and `std::io::Write`-related functions for `tokio::io::AsyncRead`- and `tokio::io::AsyncWrite`-types instead. It is automatically implemented for any type implementing `Serializable`.

For example:
```rust
use serde::{Deserialize, Serialize};
use serializable::json::Serializer;
use serializable::{Serializable, SerializableAsync as _};

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
struct HelloWorld {
    hello: String,
    world: String,
}
impl Serializable<Serializer<HelloWorld>> for HelloWorld {}

let mut buf: Vec<u8> = Vec::new();
HelloWorld { hello: "Hello".into(), world: "World".into() }.to_writer_async(&mut buf).await.unwrap();
assert_eq!(
    String::from_utf8_lossy(&buf),
    "{\"hello\":\"Hello\",\"world\":\"World\"}"
);

assert_eq!(
    HelloWorld::from_reader_async("{\"hello\":\"Goodbye\",\"world\":\"Planet\"}".as_bytes()).await.unwrap(),
    HelloWorld { hello: "Goodbye".into(), world: "Planet".into() }
)
```

### Custom `Serializer`s
You can also implement your own serializers using the `Serializer` and `SerializerAsync` traits (the latter of which is only available when the `async-tokio`-feature is enabled). For an example of how to do so, check the [dummy serializer](./src/dummy.rs)


## Features
This create has the following features:
- `async-tokio`: Enables the `SerializableAsync` and `SerializerAsync` traits for `async` contexts. Both of these are based on [`tokio`](https://tokio.rs/) as a backend.
- `serde-json`: Provides the `serializable::json::Serializer`, which can be used to implement `Serializable` for [`serde`](https://serde.rs)-compatible types to serialize/deserialize to [JSON](https://json.org). Based on [`serde_json`](https://github.com/serde-rs/json).
- `serde-yaml`: Provides the `serializable::yaml::Serializer`, which can be used to implement `Serializable` for [`serde`](https://serde.rs)-compatible types to serialize/deserialize to [YAML](https://yaml.org). Based on [`serde_yaml`](https://github.com/dtolnay/serde-yaml).
- `serde-toml`: Provides the `serializable::toml::Serializer`, which can be used to implement `Serializable` for [`serde`](https://serde.rs)-compatible types to serialize/deserialize to [TOML](https://toml.io). Based on [`toml`](https://github.com/toml-rs/toml).


## License
The GNU Public License v3 applies to this project. See [LICENSE](./LICENSE) for more details.

## Contributing
If you like to contribute to this project, welcome! Feel free to [raise an issue](https://github.com/Lut99/serializable-rs/issues) or [create a pull request](https://github.com/Lut99/serializable-rs/pulls) if you want to.

Note that this is a hobby project, and as such, I _may_ not accept your suggestions, even if they're really good ;)
