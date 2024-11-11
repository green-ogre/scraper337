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

Finally, run the binary with the target drive specified:

> [!NOTE]
> In order to access the drive's raw data, the program needs to run with privilege. However, this tool does NOT write to the drive.

```console
$ sudo ./target/release/scraper337 -d /dev/sda
```

For more info see the help:

```console
$ ./target/release/scraper337 --help
```

## Why do you need **sudo** privilege?

A mounted file system can be accessed through the /media directory on a Linux system without **sudo**. So why do we use a device (/dev) instead of a mounted file system (/media)?

The device instantiated for the drive by Linux acts just like an ordinary file, allowing us to read directly from its memory. This is NOT possible with a mounted file system, which restricts access to things like `files` and `directories`.

But __any__ access of the device means __complete access__ of the device. In other words, if we open a drive, we may be reading some sensitive data, hence the **sudo**.
