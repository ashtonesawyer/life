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
use panic_rtt_target as _;
use rtt_target::{rtt_init_print, rprintln};

mod life;
use life::*;

enum State {
    LedOn,
    LedOff,
}

fn randomize_board(rng: &mut Pcg64) -> [[u8; 5]; 5] { 
    let mut leds = [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ];
    let mut num = 0;

    for i in  0..5 {
        for j in 0..5 {
            num = rng.generate::<usize>();
            if num % 2 == 0 {
               leds[i][j] = 1; 
            }
        }
    }

    leds
}

#[entry]
fn init() -> ! {
    rtt_init_print!();
    let mut board = Board::take().unwrap();
    let mut display = Display::new(board.display_pins);
    let mut timer = Timer::new(board.TIMER0);
    let mut btn_a = board.buttons.button_a.degrade();
    let mut btn_b = board.buttons.button_b.degrade();


    let mut hwrng = HwRng::new(board.RNG);
    let seed = hwrng.random_u64();
    let mut rng = Pcg64::new_seed(seed as u128);

    // init starting random board
    let mut leds = randomize_board(&mut rng);
    display.show(&mut timer, leds, 100);

    let mut waited = 0;

    loop {
        let pressed_a = btn_a.is_low().unwrap();

        if pressed_a {
            waited = 0;
            leds = randomize_board(&mut rng); 
            display.show(&mut timer, leds, 100);
        }
        else if life::done(&leds) {
            waited += 1;

            if waited > 5 {
                waited = 0;
                leds = randomize_board(&mut rng);
                display.show(&mut timer, leds, 100);
            }
        }

        life::life(&mut leds);
        display.show(&mut timer, leds, 100);
    }
}
