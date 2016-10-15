#!/bin/sh
set -e

#
# primer build script
#
# Copyright (c) 2016 Jonathan 'theJPster' Pallant <github@thejpster.org.uk>
#

PRIMER_TARGET=lm4f120
MODE=debug
#ARGS=-v for verbose mode
ARGS=
EXAMPLE=lm4fblink
export RUST_TARGET_PATH=`pwd`

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
xargo build --target=$PRIMER_TARGET $ARGS --example $EXAMPLE

echo "Converting elf -> bin..."
arm-none-eabi-objcopy -O binary ./target/$PRIMER_TARGET/$MODE/examples/$EXAMPLE ./target/$PRIMER_TARGET/$MODE/examples/$EXAMPLE.bin

echo "Examples available..."
ls -lh ./target/$PRIMER_TARGET/$MODE/examples/*

echo "Done!"
