#![no_main]
#![no_std]

use panic_halt as _;
use cortex_m_rt::entry;
use embedded_hal::{delay::DelayNs, digital::OutputPin};
use microbit::{board::Board, hal::Timer, display::blocking::Display};


#[entry]
fn main() -> ! {
  
    let mut board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);

    let _ = board.display_pins.col1.set_low();
    let mut row1 = board.display_pins.row1;
    let mut row2 = board.display_pins.row2;
    let mut row3 = board.display_pins.row3;
    let mut row4 = board.display_pins.row4;
    let mut row5 = board.display_pins.row5;

    loop {
        let _ = row1.set_low();
        timer.delay_ms(200);

        let _ = row2.set_low();
        timer.delay_ms(200);

        let _ = row3.set_low();
        timer.delay_ms(200);

        let _ = row1.set_high();
        timer.delay_ms(200);

        let _ = row4.set_low();
        timer.delay_ms(200);

        let _ = row2.set_high();
        timer.delay_ms(200);

        let _ = row5.set_low();
        timer.delay_ms(200);
     
        let _ = row3.set_high();
        timer.delay_ms(200);

        let _ = row4.set_high();
        timer.delay_ms(200);

        let _ = row5.set_high();
        timer.delay_ms(200);
    
    }
}