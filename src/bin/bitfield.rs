#![no_main]
#![no_std]

use nrf52840_co2 as _; // global logger + panicking-behavior + memory layout
use nrf52840_hal as _;

#[cortex_m_rt::entry]
fn main() -> ! {
    // value of the FREQUENCY register (nRF52840 device; RADIO peripheral)
    let frequency: u32 = 276;
    defmt::println!("FREQUENCY: {0=0..7}, MAP: {0=8..9}", frequency);

    nrf52840_co2::exit()
}
