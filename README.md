# Rust on the Stellaris Launchpad

A bare metal example of blink written in rust for the Stellaris Launchpad (LM4F120 dev board)
## Requirements

* rustc nightly
* cargo
* arm-none-eabi-gcc
* arm-none-eabi-ar
* arm-none-eabi-objcopy

## Compile and upload

```bash
cargo build --target thumbv7em-none-eabi
arm-none-eabi-objcopy -O binary target/thumbv7em-none-eabi/debug/blink blink.bin
sudo lm4flash blink.bin
```

## You can also debug
```
$ sudo openocd -f /usr/share/openocd/scripts/board/ek-lm4f120xl.cfg
$ arm-none-eabi-gdb ./target/thumbv7em-none-eabi/debug/blink
(gdb) target remote localhost:3333
(gdb) monitor reset halt
(gdb) continue
```
