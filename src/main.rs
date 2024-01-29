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

fn randomize(rng:&mut Pcg64, image:&mut [[u8;5];5]) {

    let mut image:[[u8; 5]; 5]=[[0;5];5];
    for i in 0..image.len() {
        for j in 0..image[i].len() {
            image[i][j]=rng.generate_range(0..=1);
        }
    }
}

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();
    let pins = board.display_pins;

    let mut image=[[0;5];5];
    let mut rng = nanorand::Pcg64::new_seed(1);
    randomize(&mut rng, &mut image);

    let mut delay = timer::Timer::new(board.TIMER0);

    let mut display = Display::new(pins);

    loop {
        rprintln!("showing");
        display.show(&mut delay, image, 1000);
        life(&mut image);
        if done(&mut image) {
            randomize(&mut rng, &mut image);
        }
    }
}
