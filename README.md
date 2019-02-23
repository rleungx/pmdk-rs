# pmdk-rs (WIP)

[![Build Status](https://travis-ci.org/rleungx/pmdk-rs.svg?branch=master)](https://travis-ci.org/rleungx/pmdk-rs)
[![Build status](https://ci.appveyor.com/api/projects/status/9yg194tyy7ccfhrg/branch/master?svg=true)](https://ci.appveyor.com/project/rleungx/pmdk-rs/branch/master)

Rust API wrapping the Persistent Memory Development Kit library.

## Building
The compiled library will go to the `target` directory.
```sh
$ git clone https://github.com/rleungx/pmdk-rs.git
$ cd pmdk-rs
$ cargo build
```
Note that you must to have the pmdk library installed. For more information, see https://github.com/pmem/pmdk.

## License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.
