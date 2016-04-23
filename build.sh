#!/bin/sh
set -e
cargo build --target=thumbv7em-none-eabi
echo "Converting elf -> bin"
arm-none-eabi-objcopy -O binary ./target/thumbv7em-none-eabi/debug/bare-metal-arm-rust ./target/thumbv7em-none-eabi/debug/bare-metal-arm-rust.bin
ls -l ./target/thumbv7em-none-eabi/debug/bare-metal-arm-rust*
