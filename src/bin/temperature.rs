#![no_main]
// #![no_std]
#![cfg_attr(not(test), no_std)]

use nrf52840_co2 as _;
use nrf52840_hal::{self as _};

// pub enum TemperatureUnit {
// 	Fahrenheit(f32),
// 	Celsius(f32),
// 	Kelvin(f32),
// }

pub struct Temperature {
	celcius: f32, // Celcius
}

impl Temperature {
	pub fn new(celcius: f32) -> Self {
		Self { celcius }
	}

	pub fn to_fahrenheit(&self) -> f32 {
		self.celcius * 1.8 + 32f32
	}

	pub fn to_celcius(&self) -> f32 {
		self.celcius
	}

	pub fn to_kelvin(&self) -> f32 {
		self.celcius + 273.15f32
	}
}

// #[cfg(test)]
// mod test_super {
// 	// use super::*;
// 	// use core::prelude::rust_2021::test;

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
