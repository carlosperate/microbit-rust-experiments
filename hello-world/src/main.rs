#![no_std]
#![no_main]

use panic_halt as _;
use cortex_m_rt::entry;
use cortex_m::asm;
//use rtt_target::{rtt_init_print, rprintln};

#[entry]
fn main() -> ! {
    //rtt_init_print!();
    loop {
        //rprintln!("Hello, world!");
        for _ in 0..8_000_000 {
            asm::nop();
        }
    }
}
