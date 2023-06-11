"""
This is a test script for the pin functionality
of Pine64's Pinephone in CircuitPython.

The user/group running this script needs rw access
to the /dev/gpiochip* files
(either through udev rules or running with sudo)
"""
import board
import digitalio
import time

# Setup LED pins
r = digitalio.DigitalInOut(board.RED_LED)
g = digitalio.DigitalInOut(board.GREEN_LED)
b = digitalio.DigitalInOut(board.BLUE_LED)
leds = [r, g, b]
for led in leds:
    led.direction = digitalio.Direction.OUTPUT
    led.value = False

c = 0
while True:
    for i, led in enumerate(leds):
        if i == c % len(leds):
            led.value = True
        else:
            led.value = False
    c += 1
    time.sleep(0.2)
