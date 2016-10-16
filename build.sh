#!/bin/bash
set -e

#
# launchpad build script
#
# Copyright (c) 2016 Jonathan 'theJPster' Pallant <github@thejpster.org.uk>
#

MODE=debug
#ARGS=-v for verbose mode
ARGS=

if [ "$1" == "--release" ];
then
    MODE=release
    ARGS=--release
fi

echo "Running xargo..."
xargo build $ARGS --example launchpad_blink

echo "Converting elf -> bin..."
arm-none-eabi-objcopy -O binary ./target/thumbv7em-none-eabihf/$MODE/examples/launchpad_blink ./target/thumbv7em-none-eabihf/$MODE/examples/launchpad_blink.bin

echo "Examples available..."
ls -lh ./target/thumbv7em-none-eabihf/$MODE/examples/*

echo "Done!"
