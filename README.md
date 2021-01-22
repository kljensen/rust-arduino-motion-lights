# Rust Arduino Motion Lights

## Overview
This is my attempt to get motion lights working on my Arduino Uno.
I don't enjoy writing C and found I had stupid bugs in my Arduino code. I ❤️
Rust.

## Sources

The `av-atmega328p.json` and `uno-runner.sh` files come from Rahix's
[avr-hal](https://github.com/Rahix/avr-hal), which is MIT licensed. Everything
else here (everything authored by me) is Unlicence (see the `LICENSE` file).

The main inspiration for this work is [creativcoder's blog
post](https://dev.to/creativcoder/how-to-run-rust-on-arduino-uno-40c0) and the
[associated repo](https://github.com/creativcoder/rust-arduino-blink).

## Building

First, tell rustup to use nightly rust.

```
 rustup override set nightly
```

Then, deploy using

```
cargo run --release
```

which will run the `uno-runner.sh` script. You'll notice that requires various
avr dependencies. On my mac I install those dependencies using
[Homebrew](https://brew.sh/) as [described
here](https://book.avr-rust.com/002.1-installing-required-third-party-tools.html).

You might want to edit the `SERIAL_PORT` variable in `uno-runner.sh`. 

## Notes

* I had to [load the
  bootloader](https://www.arduino.cc/en/Tutorial/BuiltInExamples/ArduinoISP/#toc2)
  on my Arduino nano knock-offs.
