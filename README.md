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
cargo install xargo
rustup install nightly
git clone https://github.com/thejpster/launchpad-rs.git
git checkout kinetis
cd ./launchpad-rs
rustup override set nightly
rustup component add rust-src
```

## Compile and upload

```bash
xargo build --example kinetis_blink
arm-none-eabi-objcopy -O binary target/thumbv6m-none-eabi/debug/examples/kinetis_blink target/thumbv6m-none-eabi/debug/examples/kinetis_blink.bin
cp target/thumbv6m-none-eabi/debug/examples/kinetis_blink.bin /path/to/mass/storage
```

## What works:

* Nothing
