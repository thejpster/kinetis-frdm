#!/bin/sh
set -e
PRIMER_TARGET=lm4f120
xargo build --target=$PRIMER_TARGET
echo "Converting elf -> bin"
arm-none-eabi-objcopy -O binary ./target/$PRIMER_TARGET/debug/bare-metal-arm-rust ./target/$PRIMER_TARGET/debug/bare-metal-arm-rust.bin
ls -l ./target/$PRIMER_TARGET/debug/bare-metal-arm-rust*
