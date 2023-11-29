# Asynchronous Raspberry Pi GPIO devices

![CI status](https://github.com/denwong47/rpi-devices/actions/workflows/CI.yml/badge.svg?branch=main)

This library provides a set of classes to control Raspberry Pi GPIO devices asynchronously.

Supports [Button](./src/models/button.rs)s and [RGB LED](./src/models/led_rgb.rs)s.

This is currently written for the express purpose of using a
[Pimoroni Display HAT Mini](https://shop.pimoroni.com/products/display-hat-mini?variant=39496084717651) on a Pi Zero 2W; features are added as required.

Not designed for general use.
