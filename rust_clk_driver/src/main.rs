#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use core::cell;

use panic_halt as _;

static TIME: u16 = 250;
static IS_MANUAL: avr_device::interrupt::Mutex<cell::Cell<bool>> =
    avr_device::interrupt::Mutex::new(cell::Cell::new(false));

static SHOULD_TOGGLE_CLK: avr_device::interrupt::Mutex<cell::Cell<bool>> =
avr_device::interrupt::Mutex::new(cell::Cell::new(false));

#[avr_device::interrupt(atmega328p)]
// D2 interrupt
fn INT0() {
    avr_device::interrupt::free(|cs| {
        let cur = IS_MANUAL.borrow(cs).get();
        IS_MANUAL.borrow(cs).set(!cur);
    })
}

#[avr_device::interrupt(atmega328p)]
// D3 interrupt
fn INT1() {
    avr_device::interrupt::free(|cs| {
        SHOULD_TOGGLE_CLK.borrow(cs).set(true);
    })
}

fn toggle_clk(clk: &mut arduino_hal::port::Pin<arduino_hal::port::mode::Output, arduino_hal::hal::port::PD6>) -> () {
    clk.set_high();
    arduino_hal::delay_ms(TIME);
    clk.set_low();
    arduino_hal::delay_ms(TIME);
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    dp.EXINT.eicra.modify(|_, w| w.isc0().bits(0x03));
    dp.EXINT.eimsk.modify(|_, w| w.int0().set_bit());
    dp.EXINT.eicra.modify(|_, w| w.isc1().bits(0x03));
    dp.EXINT.eimsk.modify(|_, w| w.int1().set_bit());
    unsafe { avr_device::interrupt::enable() }

    /*
     * For examples (and inspiration), head to
     *
     *     https://github.com/Rahix/avr-hal/tree/main/examples
     *
     * NOTE: Not all examples were ported to all boards!  There is a good chance though, that code
     * for a different board can be adapted for yours.  The Arduino Uno currently has the most
     * examples available.
     */

    let mut is_manual = false;
    let mut should_toggle_clock = false;

    let mut clk = pins.d6.into_output();
    pins.d2.into_floating_input();
    pins.d3.into_floating_input();

    loop {
        avr_device::interrupt::free(|cs| {
            is_manual = IS_MANUAL.borrow(cs).get();
            should_toggle_clock = SHOULD_TOGGLE_CLK.borrow(cs).get();
        });
        if !is_manual {
            toggle_clk(&mut clk);
        } else if should_toggle_clock {
            toggle_clk(&mut clk);
            avr_device::interrupt::free(|cs| {
                SHOULD_TOGGLE_CLK.borrow(cs).set(false);
            });
        }
    }
}
