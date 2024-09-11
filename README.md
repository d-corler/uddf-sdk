# UDDF SDK

Rust crate for reading and writing UDDF files.

UDDF file format is a standard for dive logs. More information can be found [here](https://www.streit.cc/extern/uddf_v321/en/index.html).

The idea behind this crate is to provide a simple and easy-to-use API to convert proprietary dive logs to UDDF format and vice versa.

## Supported

| Provider | File format | Support parsing    | Support serializing |
| -------- | ----------- | ------------------ | ------------------- |
| Garmin   | Fit         | :white_check_mark: | :x:                 |

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
uddf = "0.1"
```

Or install it from the command line:

```sh
cargo install uddf-sdk
```

Run the example:

```sh
cargo run --example garmin
```