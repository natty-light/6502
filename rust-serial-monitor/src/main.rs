#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use core::cell;

use arduino_hal::{
    hal::port::Dynamic,
    port::{
        mode::{Floating, Input, Output},
        Pin,
    },
};
use panic_halt as _;

static TIME: u16 = 250;

static INT_TRIGGERED: avr_device::interrupt::Mutex<cell::Cell<bool>> =
    avr_device::interrupt::Mutex::new(cell::Cell::new(false));

#[avr_device::interrupt(atmega2560)]
fn INT0() {
    avr_device::interrupt::free(|cs| INT_TRIGGERED.borrow(cs).set(true))
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    dp.EXINT.eicra.modify(|_, w| {
        w.isc0()
            .variant(avr_device::atmega2560::exint::eicra::ISC0_A::VAL_0X03)
    });
    dp.EXINT.eimsk.modify(|_, w| w.bits(0x01));

    unsafe { avr_device::interrupt::enable() }

    pins.d53.into_floating_input();

    // let pins: [Pin<Input<Floating>, Dynamic>; 16] = [
    //     pins.d22.into_floating_input().downgrade(),
    //     pins.d24.into_floating_input().downgrade(),
    //     pins.d26.into_floating_input().downgrade(),
    //     pins.d28.into_floating_input().downgrade(),
    //     pins.d30.into_floating_input().downgrade(),
    //     pins.d32.into_floating_input().downgrade(),
    //     pins.d34.into_floating_input().downgrade(),
    //     pins.d36.into_floating_input().downgrade(),
    //     pins.d38.into_floating_input().downgrade(),
    //     pins.d40.into_floating_input().downgrade(),
    //     pins.d42.into_floating_input().downgrade(),
    //     pins.d44.into_floating_input().downgrade(),
    //     pins.d46.into_floating_input().downgrade(),
    //     pins.d48.into_floating_input().downgrade(),
    //     pins.d50.into_floating_input().downgrade(),
    //     pins.d52.into_floating_input().downgrade(),
    // ];
    let mut int_triggered = false;
    loop {
        avr_device::interrupt::free(|cs| {
            int_triggered = INT_TRIGGERED.borrow(cs).get();
        });
        if int_triggered {
            ufmt::uwriteln!(&mut serial, "Interrupt triggered\n").unwrap();
            arduino_hal::delay_ms(TIME);
            avr_device::interrupt::free(|cs| {
                INT_TRIGGERED.borrow(cs).set(false);
            });
        }
    }
}

fn read_from_pins(pins: [Pin<Input<Floating>, Dynamic>; 16]) -> [u8; 16] {
    let mut ret: [u8; 16] = [0; 16];

    for i in 0..16 {
        match pins[i].is_high() {
            true => ret[i] = 1,
            false => (),
        }
    }

    ret
}
