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
            let addr = read_address(&addr_pins);
            let data = read_data(&data_pins);
            let rwb_val = match rwb.is_high() {
                true => 'r',
                false => 'w',
            };
            ufmt::uwrite!(&mut serial, "Address: ").unwrap();
            for i in 0..addr.bin.len() {
                ufmt::uwrite!(&mut serial, "{}", addr.bin[i]).unwrap();
            }
            ufmt::uwrite!(&mut serial, "  ").unwrap();
            for i in 0..addr.hex.len() {
                ufmt::uwrite!(&mut serial, "{}", addr.hex[i] as char).unwrap();
            }
            ufmt::uwrite!(&mut serial, "{}  Data ", rwb_val).unwrap();
            for i in 0..data.bin.len() {
                ufmt::uwrite!(&mut serial, "{}", data.bin[i]).unwrap();
            }
            ufmt::uwrite!(&mut serial, "  ").unwrap();
            ufmt::uwrite!(&mut serial, "{}", data.hex as char).unwrap();
            ufmt::uwriteln!(&mut serial, "").unwrap();
            // Set Mutex value back to false, updated on next execution of line 68, setting int_triggered to false
            avr_device::interrupt::free(|cs| {
                INT_TRIGGERED.borrow(cs).set(false);
            });
        }
    }
}

struct DataPins {
    bin: [u8; 8],
    hex: u8,
}

struct AddressPins {
    bin: [u8; 16],
    hex: [u8; 2],
}

fn read_address(pins: &[Pin<Input<Floating>, Dynamic>; 16]) -> AddressPins {
    let mut bin_ret: [u8; 16] = [0; 16];
    let mut hex_ret: [u8; 2] = [0; 2];
    for i in 0..16 {
        let word = i / 8;
        let bit = i % 8;
        if pins[i].is_high() {
            {
                bin_ret[i] = 1;
                hex_ret[word] += 1 << bit;
            }
        }
    }
    AddressPins {
        bin: bin_ret,
        hex: hex_ret,
    }
}

fn read_data(pins: &[Pin<Input<Floating>, Dynamic>]) -> DataPins {
    let mut bin_ret: [u8; 8] = [0; 8];
    let mut hex_ret: u8 = 0;
    for i in 0..8 {
        let bit = i % 8;
        if pins[i].is_high() {
            {
                bin_ret[i] = 1;
                hex_ret += 1 << bit;
            }
        }
    }
    DataPins {
        bin: bin_ret,
        hex: hex_ret,
    }
}

fn toggle_pin(pin: &mut Pin<Output, Dynamic>) {
    pin.set_high();
    delay_ms(250);
    pin.set_low();
    delay_ms(250);
}
