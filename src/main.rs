#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::digital::InputPin;
#[rustfmt::skip]
use microbit::{
    board::Board,
    display::blocking::Display,
    hal::{
        Rng as HwRng,
        timer::Timer,
    },
};
use nanorand::{Rng, pcg64::Pcg64};
use panic_rtt_target as _;

mod life;
use life::*;

fn randomize_board(rng: &mut Pcg64) -> [[u8; 5]; 5] {
    let mut leds = [[0; 5]; 5];

    for row in leds.iter_mut() {
        for pin in row.iter_mut() {
            let rand: bool = rng.generate();
            if rand {
                *pin = 1;
            }
        }
    }

    leds
}

#[entry]
fn init() -> ! {
    let board = Board::take().unwrap();

    let mut display = Display::new(board.display_pins);
    let mut timer = Timer::new(board.TIMER0);
    let mut btn_a = board.buttons.button_a;
    let mut btn_b = board.buttons.button_b;

    let mut hwrng = HwRng::new(board.RNG);
    let seed = hwrng.random_u64();
    let mut rng = Pcg64::new_seed(seed as u128);

    // init starting random board
    let mut leds = randomize_board(&mut rng);
    display.show(&mut timer, leds, 100);

    let mut waited = 0; // wait after total death
    let mut ignore = 0; // ignore after btn_b press

    loop {
        let pressed_a = btn_a.is_low().unwrap();
        let pressed_b = btn_b.is_low().unwrap();

        if pressed_a {
            waited = 0;
            leds = randomize_board(&mut rng);
        } else if pressed_b && ignore > 5 {
            waited = 0;
            ignore = 0;
            for row in leds.iter_mut() {
                for pin in row.iter_mut() {
                    *pin ^= 1;
                }
            }
        } else if done(&leds) {
            waited += 1;

            if waited > 5 {
                waited = 0;
                leds = randomize_board(&mut rng);
            }
        } else {
            life(&mut leds);
        }

        ignore += 1;
        display.show(&mut timer, leds, 100);
    }
}
