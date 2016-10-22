#!/bin/bash
set -e

DEBUG_PATH=./target/thumbv7em-none-eabihf/debug/examples/launchpad_blink
RELEASE_PATH=${DEBUG_PATH/debug/release}

#
# launchpad build script
#
# Copyright (c) 2016 Jonathan 'theJPster' Pallant <github@thejpster.org.uk>
#

echo "Running xargo debug..."
xargo build --example launchpad_blink
arm-none-eabi-size -B -x ${DEBUG_PATH}

echo "Running xargo release..."
xargo build --release --example launchpad_blink
arm-none-eabi-size -B -x ${RELEASE_PATH}

echo "Running xargo docs..."
xargo doc

echo "Converting elf -> bin..."
arm-none-eabi-objcopy -O binary ${DEBUG_PATH} ${DEBUG_PATH}.bin
arm-none-eabi-objcopy -O binary ${RELEASE_PATH} ${RELEASE_PATH}.bin


echo "Examples available..."
ls -lh ./target/thumbv7em-none-eabihf/*/examples/*

echo "Done!"
