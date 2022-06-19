#![no_std]
#![no_main]

use nrf52840_co2 as _; // memory layout + panic handler
use nrf52840_hal::{self as _};

// See https://crates.io/crates/defmt-test/0.3.0 for more documentation (e.g. about the 'state'
// feature)
#[defmt_test::tests]
mod tests {
	use defmt::assert;

	#[test]
	fn it_works() {
		assert!(true)
	}
}
