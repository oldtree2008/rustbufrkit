# RustBufrKit

A project for learning [Rust](https://www.rust-lang.org/) by implementing a 
WMO [BUFR](https://en.wikipedia.org/wiki/BUFR) decoder.

The code currently does not decode the data section (section 4). Hence it is only
useful for reporting metadata. It can also be used to lookup BUFR descriptors 
(`rustbufrkit lookup 309052`). Sequence descriptors will be expanded.

The goal is to learn Rust. So it is not expected to be as feature complete as 
[PyBufrKit](https://github.com/ywangd/pybufrkit).
