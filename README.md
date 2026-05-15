# IQ-Convert: Conversion Tool for IQ Files

`iqconvert` is a [FutureSDR](https://www.futuresdr.org/), rust-based cli tool
for converting IQ-Data from one format into another one. It helps you when
working with iq data from different sources and origins

> [!TIP]
> Main Repository: [codeberg.org/akira25/iqconvert](https://codeberg.org/akira25/iqconvert)

## Work in Progress

This tool is a work in progress. It does not support all conversions between
all formats at the moment. If you need another conversion, feel free to add it
to the code and hand in a merge request, or open an issue.

## Building

To build this tool, you need a recent [rust-toolchain](https://rustup.rs/) and
cargo:

```sh
$ git clone https://codeberg.org/akira25/iqconvert.git
$ cd iqconvert
$ cargo build --release
```

## Usage Examples

Converting a [SatNOGS](https://network.satnogs.org/) IQ-File to replay it in
[gqrx](https://www.gqrx.dk/):

```sh
$ ./iqconvert satnogs_iq.raw -i ci16_le -t cf32_le -o satnogs_iq_13978190_436775000_57600_fc.raw
```

Please mind, that `gqrx` uses a naming convention for IQ files. To seamlessly
replay your IQ samples, you should follow that convention:

```sh
yourname_{Freq. in Hz}_{bandwidth in hz}_fc.raw
```
