# gvas

The gvas crate is a Rust library that allows parsing of gvas save files.

## Save files

A gvas save file is a binary file format used by the Unreal Engine 4 (UE4) game
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

The example code provided below demonstrates how to use the gvas crate to read
a gvas save file. The code first reads the contents of the gvas .sav file into
a byte vector. Then, a Cursor is created from the byte vector to allow reading
from the vector as if it were a file. Finally, the GvasFile struct's read()
method is used to parse the data from the cursor and produce a GvasFile struct.
The GvasFile struct represents the parsed data from the gvas save file and can
be used to read and modify the data.

```rust
use gvas::GvasFile;
use std::{
    fs::File,
    io::{Cursor, Read},
    path::Path,
};

let mut file = File::open("save.sav").unwrap();
let mut data = Vec::new();
file.read_to_end(&mut data).unwrap();

let mut cursor = Cursor::new(data);
let gvas_file = GvasFile::read(&mut cursor);

println!("{:#?}", gvas_file);
```

The [tests directory](https://github.com/localcc/gvas/tree/main/tests) contains
several tests that demonstrate how to use the crate to read and write gvas
files.

## Contributing

Please see the [CONTRIBUTING](CONTRIBUTING.md) document for guidelines on how
to contribute to this project.

## License

This library is distributed under the terms of the MIT license. See the
[LICENSE](LICENSE) file for details.
