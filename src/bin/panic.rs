#![no_main]
#![no_std]

use nrf52840_co2 as _; // global logger + panicking-behavior + memory layout
use nrf52840_hal as _;

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("main");

    defmt::panic!()
}
