#!/bin/sh
set -e
AR_lm4f120=arm-none-eabi-ar CC_lm4f120=arm-none-eabi-gcc xargo build --target=lm4f120
echo "Converting elf -> bin"
arm-none-eabi-objcopy -O binary ./target/$PRIMER_TARGET/debug/bare-metal-arm-rust ./target/$PRIMER_TARGET/debug/bare-metal-arm-rust.bin
ls -l ./target/$PRIMER_TARGET/debug/bare-metal-arm-rust*
