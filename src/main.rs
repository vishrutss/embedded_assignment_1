#![no_main]
#![no_std]

mod life;

use life::*;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use cortex_m_rt::entry;
use microbit::{
    board::Board,
    display::blocking::Display,
    hal::{prelude::*, timer},
};

use nanorand::{pcg64::Pcg64, Rng, SeedableRng};

fn randomize() -> [[u8; 5]; 5] {

    let mut image:[[u8; 5]; 5]=[[0;5];5];
    let mut rng = nanorand::Pcg64::new_seed(0);
    for i in 0..image.len() {
        for j in 0..image[i].len() {
            let b: bool = rng.generate();
            if b {
                image[i][j] = 1;
            } else {
                image[i][j] = 0;
            }
        }
    }
    image
}

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();
    let pins = board.display_pins;

    let mut image = randomize();

    let mut delay = timer::Timer::new(board.TIMER0);

    let mut display = Display::new(pins);

    loop {
        rprintln!("showing");
        display.show(&mut delay, image, 1000);
        life(&mut image);
        if done(&mut image) {
            image = randomize();
        }
    }
}
