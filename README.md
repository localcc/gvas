# gvas

The gvas crate is a Rust library that allows parsing of gvas save files.

## Documentation

Crate documentation is published to
[docs.rs/gvas](https://docs.rs/gvas/).

## Save files

A gvas save file is a binary file format used by the Unreal Engine 4/5 (UE4/UE5) game
engine to store persistent data such as player progress, game settings, and
other game-related information.

## Usage

The crate can be added to a Rust project as a dependency by running the command
`cargo add gvas`.

## Serde Support

This crate supports serde deserialization and serialization. To use serde with
gvas, the serde feature must be enabled by running
`cargo add gvas --features serde`.

## Examples

The example code below demonstrates how to use the gvas crate to read a gvas
save file. The GvasFile struct's read() method is used to parse the data from
the file and produce a GvasFile struct.

```rust
use gvas::GvasFile;
use std::fs::File;

let mut file = File::open("save.sav")?;
let gvas_file = GvasFile::read(&mut file, GameVersion::Default);

println!("{:#?}", gvas_file);
```

The [tests directory](https://github.com/localcc/gvas/tree/main/tests) contains
several tests that demonstrate how to use the crate to read and write gvas
files.

## Status

[![Coverage Status](https://coveralls.io/repos/github/localcc/gvas/badge.svg?branch=main)](https://coveralls.io/github/localcc/gvas?branch=main)

## Contributing

Please see the [CONTRIBUTING](CONTRIBUTING.md) document for guidelines on how
to contribute to this project.

## License

This library is distributed under the terms of the MIT license. See the
[LICENSE](LICENSE) file for details.
