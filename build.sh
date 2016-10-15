#!/bin/sh
set -e

#
# primer build script
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

if ! which xargo > /dev/null;
then
	echo "Xargo not in path. Is this travis?"
	if [ -d /home/travis/.cargo/bin ];
	then
		echo "Yes - setting path for travis"
		export PATH=/home/travis/.cargo/bin:$PATH
	else
		echo "Xargo not found. Try 'cargo install xargo'"
		exit 1
	fi
fi

echo "Running xargo..."
xargo build $ARGS --example launchpad_blink

echo "Converting elf -> bin..."
arm-none-eabi-objcopy -O binary ./target/thumbv7em-none-eabihf/$MODE/examples/launchpad_blink ./target/thumbv7em-none-eabihf/$MODE/examples/launchpad_blink.bin

echo "Examples available..."
ls -lh ./target/thumbv7em-none-eabihf/$MODE/examples/*

echo "Done!"
