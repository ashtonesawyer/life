#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::digital::InputPin;
#[rustfmt::skip]
use microbit::{
    board::{Board, Buttons},
    display::blocking::Display,
    hal::{
        Rng as HwRng,
        timer::Timer,
    },
};
use nanorand::{pcg64::Pcg64, Rng, SeedableRng};

mod life;
use life::*;

enum State {
    LedOn,
    LedOff,
}

fn randomize_board() {
    
}

#[entry]
fn init() -> ! {
    let mut board = Board::take().unwrap();
    let mut display = Display::new(board.display_pins);
    let mut timer = Timer::new(board.TIMER0);
    let mut btn_a = board.buttons.button_a.degrade();
    let mut btn_b = board.buttons.button_b.degrade();

    loop {
        let mut leds = randomize_board();
        board
    }
}
