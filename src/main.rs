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

    let mut row_a = pins.d22.into_output(&mut pins.ddr);
    let mut row_b = pins.d24.into_output(&mut pins.ddr);
    let mut row_c = pins.d26.into_output(&mut pins.ddr);
    let mut row_d = pins.d28.into_output(&mut pins.ddr);
    let mut row_e = pins.d30.into_output(&mut pins.ddr);
    let mut row_f = pins.d32.into_output(&mut pins.ddr);
    let mut row_g = pins.d34.into_output(&mut pins.ddr);
    let mut row_h = pins.d36.into_output(&mut pins.ddr);

    let mut col_0 = pins.d23.into_output(&mut pins.ddr);
    let mut col_1 = pins.d25.into_output(&mut pins.ddr);
    let mut col_2 = pins.d27.into_output(&mut pins.ddr);
    let mut col_3 = pins.d29.into_output(&mut pins.ddr);
    let mut col_4 = pins.d31.into_output(&mut pins.ddr);
    let mut col_5 = pins.d33.into_output(&mut pins.ddr);
    let mut col_6 = pins.d35.into_output(&mut pins.ddr);
    let mut col_7 = pins.d37.into_output(&mut pins.ddr);

    row_a.set_high().void_unwrap();
    row_b.set_high().void_unwrap();
    row_c.set_high().void_unwrap();
    row_d.set_high().void_unwrap();
    row_e.set_high().void_unwrap();
    row_f.set_high().void_unwrap();
    row_g.set_high().void_unwrap();
    row_h.set_high().void_unwrap();

    col_0.set_low().void_unwrap();
    col_1.set_low().void_unwrap();
    col_2.set_low().void_unwrap();
    col_3.set_low().void_unwrap();
    col_4.set_low().void_unwrap();
    col_5.set_low().void_unwrap();
    col_6.set_low().void_unwrap();
    col_7.set_low().void_unwrap();


    loop {
        row_d.set_low().void_unwrap();
        row_e.set_low().void_unwrap();
        col_3.set_high().void_unwrap();
        col_4.set_high().void_unwrap();
    }
}