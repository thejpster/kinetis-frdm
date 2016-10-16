#!/bin/bash
set -e

#
# launchpad build script
#
# Copyright (c) 2016 Jonathan 'theJPster' Pallant <github@thejpster.org.uk>
#

echo "Running xargo debug..."
xargo build --example launchpad_blink

echo "Running xargo release..."
xargo build --release --example launchpad_blink

echo "Running xargo docs..."
xargo doc

echo "Converting elf -> bin..."
arm-none-eabi-objcopy -O binary ./target/thumbv7em-none-eabihf/debug/examples/launchpad_blink ./target/thumbv7em-none-eabihf/debug/examples/launchpad_blink.bin
arm-none-eabi-objcopy -O binary ./target/thumbv7em-none-eabihf/release/examples/launchpad_blink ./target/thumbv7em-none-eabihf/release/examples/launchpad_blink.bin

echo "Examples available..."
ls -lh ./target/thumbv7em-none-eabihf/*/examples/*

echo "Done!"
