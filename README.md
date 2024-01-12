
<div class="oranda-hide">

# Yorch's Simple File Encryption/Decryption (YSFED)

</div>

Simple tool for protecting files with a password.

![Continuous integration](https://github.com/JorgeMayoral/ysfed/workflows/Continuous%20integration/badge.svg)
[![](https://img.shields.io/crates/v/ripnode.svg)](https://crates.io/crates/ysfed)

> [!CAUTION]
> This is a toy project, do not use it for anything serious, it is not secure. 

> [!WARNING]
> TWork in progress, API may change. 


<div class="oranda-hide">

## Installation

### Cargo

```sh
cargo install ysfed
```

</div>

## Usage

```sh
ysfed --file <FILE> --password <PASSWORD> <encrypt|decrypt>
```

### Commands

```sh
  encrypt    Encrypts a file
  decrypt    Decrypts a file
```

### Options

```sh
  -f, --file       File to encrypt/decrypt
  -p, --password   Password to encrypt/decrypt
  -o, --output     Output file
  -h, --help       Print help
  -V, --version    Print version
```

## Purpose

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
