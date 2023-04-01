# Cannon Raw Image Parser

This Rust program can parse Canon RAW file format. Currently supports the CRW format only. Implementation is based on the CRW standard information available [here](https://exiftool.org/canon_raw.html).

It's a work in progress and only reads and prints the file header information right now.

Usage:
```
$ cargo run -- ./image_file.crw

Header is Header { endianness: LE, header_length: 26, signature: HEAPCCDR, crw_version: 65538, reserved: [0, 0] }
Body is DirectoryBlock { value_data len: 8325954, dir_count: 3, dir_entries: 3, other_data len: 0, dir_start: 8325954 }
```
