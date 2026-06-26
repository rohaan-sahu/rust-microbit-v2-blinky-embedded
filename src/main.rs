#![no_main]
#![no_std]

use cortex_m::asm::nop;
use embedded_hal::digital::OutputPin;
use embedded_hal::digital::PinState;
use nrf52833_hal as hal;
use nrf52833_hal::gpio::Level;

#[panic_handler] // panicking behavior
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = hal::pac::Peripherals::take().unwrap();
    let port0 = hal::gpio::p0::Parts::new(p.P0);

    let _col1 = port0.p0_28.into_push_pull_output(Level::Low);
    let mut row1 = port0.p0_21.into_push_pull_output(Level::Low);

    let mut is_on: bool = false;

    loop {
        let _ = row1.set_state(PinState::from(is_on));

        for _ in 1..100_000 {
            nop();
        }

        is_on = !is_on;
    }
}