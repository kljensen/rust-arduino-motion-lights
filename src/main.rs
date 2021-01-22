#![no_std]
#![no_main]

use arduino_uno::prelude::*;
use panic_halt as _;

#[arduino_uno::entry]
fn main() -> ! {
    let dp = arduino_uno::Peripherals::take().unwrap();

    let mut pins = arduino_uno::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD);

    // Digital pin 13 is also connected to an onboard LED marked "L"
    let mut led = pins.d13.into_output(&mut pins.ddr);
    let mut power_strip = pins.d2.into_output(&mut pins.ddr);

    // For some reaons, (UGH!!) my powertail seems to be reversed.
    // The HIGH voltage turns it off and the LOW voltage turns it
    // on. I don't believe that ought to be the case and I can't
    // quite understand it. When the signal wire is disconnected,
    // it is off. hmm. See
    // http://cdn.sparkfun.com/datasheets/Components/General/80136%20PST%20II%20Instructions%202015-11.pdf
    power_strip.set_high().void_unwrap();
    led.set_low().void_unwrap();

    let mut motion_countdown: u8 = 0;
    const MOTION_COUNTDOWN_MAX: u8 = 10;
    const SLEEP_TIME_MS: u16 = 500;
    loop {
        if pins.d8.is_high().void_unwrap() || pins.d11.is_high().void_unwrap() {
            motion_countdown = MOTION_COUNTDOWN_MAX;
        }

        if motion_countdown == 0 {
            // Turn off LED if it is on
            if led.is_set_high().void_unwrap() {
                led.set_low().void_unwrap();
            }
            if power_strip.is_set_low().void_unwrap() {
                power_strip.set_high().void_unwrap();
            }
        } else {
            // Turn on LED if it is off
            if led.is_set_low().void_unwrap() {
                led.set_high().void_unwrap();
            }
            if power_strip.is_set_high().void_unwrap() {
                power_strip.set_low().void_unwrap();
            }
            motion_countdown -= 1;
        }
        arduino_uno::delay_ms(SLEEP_TIME_MS);
    }
}
