#![no_main]
#![no_std]
// #![cfg_attr(not(test), no_std)]

use core::convert::From;
use nrf52840_co2 as _;
use nrf52840_hal::{self as _};

pub enum TemperatureUnit {
	Fahrenheit(f32),
	Celsius(f32),
	Kelvin(f32),
}

pub struct Temperature {
	pub value: TemperatureUnit,
}

impl From<f32> for Temperature {
	fn from(val: f32) -> Self {
		Self { value: TemperatureUnit::Celsius(val) }
	}
}

impl Temperature {
	pub fn to_fahrenheit(&self) -> TemperatureUnit {
		TemperatureUnit::Fahrenheit(451f32)
	}

    pub fn to_celcius(&self) -> TemperatureUnit {
		TemperatureUnit::Celsius(42f32)
	}

    pub fn to_kelvin(&self) -> TemperatureUnit {
		TemperatureUnit::Kelvin(10f32)
	}
}

// #[cfg(test)]
// #[macro_use]
// extern crate std;

// #[cfg(test)]
// mod test_super {
// 	use super::*;
// 	use core::prelude::rust_2021::test;

// 	#[test]
// 	fn test_from_celcius() {
//         assert!(true);
//         // let temp: Temperature = Temperature::from(20f32);
// 		// match temp.value {
// 		// 	TemperatureUnit::Celsius(x) => defmt::println!("temp = {:?}", x),
// 		// 	TemperatureUnit::Fahrenheit(x) => defmt::println!("temp = {:?}", x),
// 		// 	TemperatureUnit::Kelvin(x) => defmt::println!("temp = {:?}", x),
// 		// }
// 	}
// }
