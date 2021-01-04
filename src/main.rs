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

    led.set_low().void_unwrap();
    let mut motion_countdown: u8 = 0;
    const MOTION_COUNTDOWN_MAX: u8 = 10;
    const SLEEP_TIME_MS:u16 = 500;
    let d8 = pins.d8.into_pull_up_input(&mut pins.ddr);
    let d11 = pins.d11.into_pull_up_input(&mut pins.ddr);
    // let d8 = pins.d8.into_floating_input(&mut pins.ddr);
    // let d11 = pins.d11.into_floating_input(&mut pins.ddr);

    loop {
        if d8.is_high().void_unwrap() || d11.is_high().void_unwrap() {
        // if d8.is_high().void_unwrap(){
            motion_countdown = MOTION_COUNTDOWN_MAX;
        }

        if motion_countdown == 0 {
            // Turn off LED if it is on
            if led.is_set_high().void_unwrap() {
                led.set_low().void_unwrap();
            }
        } else {
            // Turn on LED if it is off
            if led.is_set_low().void_unwrap() {
                led.set_high().void_unwrap();
            }
            motion_countdown -= 1;
        }
        arduino_uno::delay_ms(SLEEP_TIME_MS);
    }
}
