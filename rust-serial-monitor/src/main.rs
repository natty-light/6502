#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use core::cell;

use arduino_hal::pac::exint::eicrb::ISC4_A;
use arduino_hal::{
    delay_ms,
    hal::port::Dynamic,
    port::{
        mode::{Floating, Input, Output},
        Pin,
    },
};
use panic_halt as _;

static INT_TRIGGERED: avr_device::interrupt::Mutex<cell::Cell<bool>> =
    avr_device::interrupt::Mutex::new(cell::Cell::new(false));

#[avr_device::interrupt(atmega2560)]
fn INT4() {
    avr_device::interrupt::free(|cs| INT_TRIGGERED.borrow(cs).set(true))
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    let mut led = pins.d13.into_output().downgrade();

    // Set INT4 to trigger on rising edge of pin change
    dp.EXINT
        .eicrb
        .modify(|_, w| w.isc4().variant(ISC4_A::VAL_0X03));
    // Enable INT4 by setting Bit 0 if EIMSK to 1
    dp.EXINT.eimsk.modify(|_, w| w.bits(0x10));

    unsafe { avr_device::interrupt::enable() }

    // Pin 2: INT4
    pins.d2.into_floating_input();

    let addr_pins: [Pin<Input<Floating>, Dynamic>; 16] = [
        pins.d22.into_floating_input().downgrade(),
        pins.d24.into_floating_input().downgrade(),
        pins.d26.into_floating_input().downgrade(),
        pins.d28.into_floating_input().downgrade(),
        pins.d30.into_floating_input().downgrade(),
        pins.d32.into_floating_input().downgrade(),
        pins.d34.into_floating_input().downgrade(),
        pins.d36.into_floating_input().downgrade(),
        pins.d38.into_floating_input().downgrade(),
        pins.d40.into_floating_input().downgrade(),
        pins.d42.into_floating_input().downgrade(),
        pins.d44.into_floating_input().downgrade(),
        pins.d46.into_floating_input().downgrade(),
        pins.d48.into_floating_input().downgrade(),
        pins.d50.into_floating_input().downgrade(),
        pins.d52.into_floating_input().downgrade(),
    ];

    // Data pins
    let data_pins: [Pin<Input<Floating>, Dynamic>; 8] = [
        pins.d39.into_floating_input().downgrade(),
        pins.d41.into_floating_input().downgrade(),
        pins.d43.into_floating_input().downgrade(),
        pins.d45.into_floating_input().downgrade(),
        pins.d47.into_floating_input().downgrade(),
        pins.d49.into_floating_input().downgrade(),
        pins.d51.into_floating_input().downgrade(),
        pins.d53.into_floating_input().downgrade(),
    ];

    let rwb = pins.d37.into_floating_input().downgrade();

    // Flag for interrupt detection, updated by Mutex value in loop
    let mut int_triggered = false;
    loop {
        // Retrieve value from Mutex and store it in int_triggered
        avr_device::interrupt::free(|cs| {
            int_triggered = INT_TRIGGERED.borrow(cs).get();
        });
        if int_triggered {
            let addr: u16 = read_address(&addr_pins);
            let data: u8 = read_data(&data_pins);
            let rwb_val: char = if rwb.is_high() { 'r' } else { 'w' };
            ufmt::uwrite!(&mut serial, "Address: ").unwrap();
            for i in 0..16 {
                let bit = if (addr & 1 << i) != 0 { 1 } else { 0 };
                ufmt::uwrite!(&mut serial, "{}", bit).unwrap();
            }
            ufmt::uwrite!(&mut serial, " {:4x}   {}   Data: ", addr, rwb_val).unwrap();
            for i in 0..8 {
                let bit = if (data & 1 << i) != 0 { 1 } else { 0 };
                ufmt::uwrite!(&mut serial, "{}", bit).unwrap();
            }
            ufmt::uwriteln!(&mut serial, " {:4x}", data).unwrap();
            // Set Mutex value back to false, updated on next execution of line 68, setting int_triggered to false
            avr_device::interrupt::free(|cs| {
                INT_TRIGGERED.borrow(cs).set(false);
            });
        }
    }
}

fn read_address(pins: &[Pin<Input<Floating>, Dynamic>; 16]) -> u16 {
    let mut addr: u16 = 0;
    for i in 0..16 {
        let bit: u16 = if pins[i].is_high() { 1 } else { 0 };
        addr = (addr << 1) + bit
    }
    addr
}

fn read_data(pins: &[Pin<Input<Floating>, Dynamic>]) -> u8 {
    let mut data: u8 = 0;
    for i in 0..8 {
        let bit: u8 = if pins[i].is_high() { 1 } else { 0 };
        data = (data << 1) + bit
    }
    data
}

#[allow(dead_code)]
fn toggle_pin(pin: &mut Pin<Output, Dynamic>) {
    pin.set_high();
    delay_ms(250);
    pin.set_low();
    delay_ms(250);
}
