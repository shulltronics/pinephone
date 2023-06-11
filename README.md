PinePhone Development Repository
================================
### Carsten Thue-Bludworth, 2023

This repository contains code and configuration for use with a Pine64 PinePhone. This is a work-in-progress; a playground for experimenting with embedded-linux development.

### Directory structure
The `circuitpython` directory has code examples for running CircuitPython on the Pinephone. I have added initial support (more to come!) to `Adafruit_Python_PlatformDetect` and `Adafruit_Blinka` for the Pinephone, which currently includes pin mappings for the front RGB LEDs. The Pinephone screen can also be controlled in CircuitPython via my `displayio_fbdev` repository, which provides a CircuitPython `Display` via the Linux framebuffer driver.

The `rust` repo contains some Rust code that show probably be removed, as it already exists in the `displayio_fbdev` repo.

### Notes on framebuffer development
* I played with changing the way consoles are sent to the framebuffer with `systemctl <disable|stop> getty@tty<0|1>.service`
* `tput civis > /dev/tty1` disables the cursor. How to re-enable?
* Disable kernel logging to the console with `dmesg -D`; re-enable with `dmesg -E`
* Monitor battery status with `upower -d`
