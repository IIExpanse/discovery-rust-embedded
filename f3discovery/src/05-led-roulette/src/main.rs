#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{Delay, DelayMs, LedArray, OutputSwitch, entry};

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, LedArray) = aux5::init();

    let half_period = 50_u16;

    leds[0].on().ok();
    let mut cur: usize = 0;
    loop {
        let prev: usize = cur;
        cur = (cur + 1) % 8;
        leds[cur].on().ok();
        delay.delay_ms(half_period);
        leds[prev].off().ok();
        delay.delay_ms(half_period);
    }
}


