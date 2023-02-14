PinePhone Development Repository
================================
### Carsten Thue-Bludworth, 2023

This repository contains code and configuration for use with a Pine64 PinePhone. This is a work-in-progress; a playground for experimenting with embedded-linux development.

### Notes on framebuffer development
* I played with changing the way consoles are sent to the framebuffer with `systemctl <disable|stop> getty@tty<0|1>.service`
* `tput civis > /dev/tty1` disables the cursor. How to re-enable?
* Disable kernel logging to the console with `dmesg -D`; re-enable with `dmesg -E`
* Monitor battery status with `upower -d`
