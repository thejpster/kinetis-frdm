# Rust on the Stellaris Launchpad

A bare metal example program written in Rust (https://rust-lang.org) for the Stellaris Launchpad (LM4F120 dev board)

The idea is that useful functionality will be moved out into separate crates.

## Requirements

* rustc nightly
* xargo (run `cargo install xargo`)
* arm-none-eabi-gcc
* arm-none-eabi-ar
* arm-none-eabi-objcopy

## Compile and upload

```bash
xargo build --example launchpad_blink
arm-none-eabi-objcopy -O binary target/thumbv7em-none-eabihf/debug/examples/launchpad_blink target/thumbv7em-none-eabihf/debug/examples/launchpad_blink.bin
sudo lm4flash target/thumbv7em-none-eabihf/debug/examples/launchpad_blink.bin
```

## You can also debug
```
$ sudo openocd -f /usr/share/openocd/scripts/board/ek-lm4f120xl.cfg
$ arm-none-eabi-gdb ./target/thumbv7em-none-eabihf/debug/examples/launchpad_blink
(gdb) target remote localhost:3333
(gdb) monitor reset halt
(gdb) continue
```

## What works:

* UART works, using the on-board UART-to-USB bridge (115200 bps, 8N1)
* PLL runs at 66.7MHz
* SysTick works at 4MHz, providing a timer a currently use for the busy-waits
* GPIO works - you can control the on-board RGB LED
* Timer works - you can drive GPIOs (including the LED) with PWM
* Panic handler works - it quickly flashes the red LED if it panics or hits a hardfault
