# Minecrevy IO Buffers

This crate provides functions for reading/writing [Minecraft protocol data types](https://wiki.vg/Protocol#Data_types) to/from IO buffers.

## Dependency

```toml
[dependencies]
minecrevy_io_buf = "<version #>"
```

## Features

- `blocking`: Provides these functions on `std::io::Read/Write`.
- `async-tokio`: Provides these functions on `tokio::io::AsyncRead/Write`.

## Packets

This crates also provides low-level operations for reading and writing Minecraft packets, in the form of the `RawPacket` struct:

```rust
fn main() {
    let mut buf = [0; 4096];
    // read from a socket or something else into buf
    
    let RawPacket { _id, _body } = Cursor::new(&buf).read_packet()
        .expect("malformed packet");
    // now we'd usually branch based on id, and then parse the body.
}
```
