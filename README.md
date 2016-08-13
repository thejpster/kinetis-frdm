# Rust on the Stellaris Launchpad

A bare metal example program written in Rust (https://rust-lang.org) for the Stellaris Launchpad (LM4F120 dev board)

## Requirements

* rustc nightly
* xargo (https://github.com/japaric/xargo)
* arm-none-eabi-gcc
* arm-none-eabi-ar
* arm-none-eabi-objcopy

## Compile and upload

```bash
xargo build --target lm4f120
arm-none-eabi-objcopy -O binary target/lm4f120/debug/bare-metal-arm-rust target/lm4f120/debug/bare-metal-arm-rust.bin
sudo lm4flash target/lm4f120/debug/bare-metal-arm-rust.bin
```

## You can also debug
```
$ sudo openocd -f /usr/share/openocd/scripts/board/ek-lm4f120xl.cfg
$ arm-none-eabi-gdb ./target/lm4f120/debug/bare-metal-arm-rust
(gdb) target remote localhost:3333
(gdb) monitor reset halt
(gdb) continue
```

## What works:

* UART works, using the on-board UART-to-USB bridge (115200 bps, 8N1)
* PLL runs at 66.7MHz
* SysTick works at 4MHz, providing a timer a currently use for the busy-waits
* GPIO works - you can control the on-board RGB LED
* Panic handler works - it quickly flashes the red LED if it panics or hits a hardfault

## What doesn't work

* Release mode hardfaults on startup, unless you step through the debugger in which case it doesn't :/
