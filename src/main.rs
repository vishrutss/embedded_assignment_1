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

use nanorand::{pcg64::Pcg64, Rng};

fn randomize(rng: &mut Pcg64, image: &mut [[u8; 5]; 5]) {
    for row in image {
        for cell in row {
            *cell = rng.generate_range(0..=1);
        }
    }
}

fn complement(image: &mut [[u8; 5]; 5]) {
    for row in image {
        for cell in row {
            if *cell == 0 {
                *cell = 1;
            } else {
                *cell = 0;
            }
        }
    }
}

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();
    let pins = board.display_pins;
    let mut image = [[0; 5]; 5];
    let mut rng = nanorand::Pcg64::new_seed(1);
    let buttons = board.buttons;

    randomize(&mut rng, &mut image);

    let mut delay = timer::Timer::new(board.TIMER0);

    let mut display = Display::new(pins);

    loop {
        rprintln!("showing");
        display.show(&mut delay, image, 1000);
        life(&mut image);
        if done(&image) {
            delay.delay_ms(500u16);
            if buttons.button_a.is_high().unwrap() && buttons.button_b.is_high().unwrap() {
                randomize(&mut rng, &mut image);
            }
        }
        if buttons.button_a.is_low().unwrap() {
            rprintln!("button A");
            randomize(&mut rng, &mut image)
        }
        else if buttons.button_b.is_low().unwrap() {
            rprintln!("button B");
            complement(&mut image);
            delay.delay_ms(500u16);
            continue;
        }
        delay.delay_ms(100u16);
    }
}
