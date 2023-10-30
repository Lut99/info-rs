# Changelog for `serializer`
This file keeps track of changes to the `serializer` crate.

Note that this project uses [semantic versioning](https://semver.org). As such, we will indicate \[**breaking changes**\] to changes that are breaking.


## v0.1.0 - 2023-10-30
### Added
- Initial release.
- The `Serializable`-trait.
- The `SerializableAsync`-trait under the `async-tokio`-feature.
- The `Serializer`-trait.
- The `SerializerAsync`-trait under the `async-tokio`-feature.
- A dummy serializer.
- A JSON serializer under the `serde-json`-feature.
- A TOML serializer under the `serde-toml`-feature.
- A YAML serializer under the `serde-yaml`-feature.
