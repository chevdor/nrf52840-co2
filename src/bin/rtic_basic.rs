#![no_main]
#![no_std]
use nrf52840_hal as hal;
use rtic::app;

#[app(device = crate::hal::pac)]
mod app {
	#[shared]
	struct Shared {}

	#[local]
	struct Local {}

	#[init]
	fn init(_: init::Context) -> (Shared, Local, init::Monotonics) {
		defmt::println!("init");

		(Shared {}, Local {}, init::Monotonics())
	}

	#[idle]
	fn idle(_: idle::Context) -> ! {
		defmt::println!("idle");
		let mut i = 0;

		while i < 10 {
			i += 1;
			defmt::println!("{=u8}", i);
		}
		nrf52840_co2::exit();
	}
}
