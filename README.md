# gvas

This crate was written to parse UE4 gvas save files.

# Usage

To use this crate add it as a dependency to your `Cargo.toml`

```toml
gvas = "0.3.0"
```

# Serde

This crate supports serde deserialization and serialization.
To use it enable the `serde` feature.

# Examples

```rust
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

For more examples look in the [tests](https://github.com/localcc/gvas/tree/main/tests) directory.
