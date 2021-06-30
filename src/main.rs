#![no_std]
#![no_main]

extern crate panic_halt;
use arduino_mega2560::prelude::*;

#[arduino_mega2560::entry]
fn main() -> ! {
    let dp = arduino_mega2560::Peripherals::take().unwrap();

    let mut pins = arduino_mega2560::Pins::new(
        dp.PORTA,
        dp.PORTB,
        dp.PORTC,
        dp.PORTD,
        dp.PORTE,
        dp.PORTF,
        dp.PORTG,
        dp.PORTH,
        dp.PORTJ,
        dp.PORTK,
        dp.PORTL,
    );

    let mut row = pins.d34.into_output(&mut pins.ddr);
    let mut col = pins.d27.into_output(&mut pins.ddr);

    loop {
        row.set_low().void_unwrap();
        col.set_high().void_unwrap();
    }
}