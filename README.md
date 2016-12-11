# Rust on the Freescale Kinetis FRDM-KE06Z

A bare metal example program written in Rust (https://rust-lang.org) for the Freescale Kinetis FRDM-KE06Z (KE06Z dev board)

The idea is that useful functionality will be moved out into separate crates.

## Requirements

* rustc nightly
* xargo
* arm-none-eabi-gcc
* arm-none-eabi-ar
* arm-none-eabi-objcopy

## Geting set up

```bash
rustup install nightly
rustup component add rust-src
cargo install xargo
git clone https://github.com/thejpster/kinetis-frdm
cd ./kinetis-frdm
rustup override set nightly
./build.sh
```

## Compile and upload

```bash
xargo build --example kinetis_blink
arm-none-eabi-objcopy -O binary target/thumbv6m-none-eabi/debug/examples/kinetis_blink target/thumbv6m-none-eabi/debug/examples/kinetis_blink.bin
cp target/thumbv6m-none-eabi/debug/examples/kinetis_blink.bin /path/to/mass/storage
```

## What works:

* Nothing
