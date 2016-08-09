#!/bin/sh
set -e
PRIMER_TARGET=lm4f120
if ! which xargo;
then
	echo "Xargo not in path. Is this travis?"
	if [ -d /home/travis/.cargo/bin ];
	then
		echo "Yes - setting path for travis"
		export PATH=/home/travis/.cargo/bin:$PATH
	else
		echo "Xargo not found"
		exit 1
	fi
fi
xargo build --target=$PRIMER_TARGET -v
echo "Converting elf -> bin"
arm-none-eabi-objcopy -O binary ./target/$PRIMER_TARGET/debug/bare-metal-arm-rust ./target/$PRIMER_TARGET/debug/bare-metal-arm-rust.bin
ls -l ./target/$PRIMER_TARGET/debug/bare-metal-arm-rust*
