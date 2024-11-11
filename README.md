# Scraper337

> [!WARNING]
> THIS TOOL IS A WORK IN PROGRESS! USE AT YOUR OWN RISK!

This tool searches through raw drive data for common file headers and attempts to extract the file data. If an extracted file is invalid, then it will be ignored.

You can find the extracted files in the "extract" directory.

This tool uses chunking to process a portion of the drive at once. This means that extraction is incremental, and you can observe changes as the program runs!

## Building (Linux)

To build this tool, you will need the [rust](https://www.rust-lang.org/tools/install) toolchain:
```console
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
Next, clone the repo and build the binary:

```console
$ git clone https://github.com/void-scape/scraper337
$ cd scraper337
$ cargo build --release
```

Finally, run the binary with the target drive specified. Note that, in order to access the drive's raw data, the program needs to run with privilege:

```console
$ sudo ./target/release/scraper337 -d /dev/sda
```

For more info see the help:

```console
$ ./target/release/scraper337 --help
```
