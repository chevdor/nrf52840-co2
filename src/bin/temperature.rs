#![no_main]
// #![no_std]
#![cfg_attr(not(test), no_std)]

use defmt::{Format, Str};
use nrf52840_co2 as _;
use nrf52840_hal::{self as _};

pub enum TemperatureUnit {
	Fahrenheit,
	Celsius,
	Kelvin,
}

// impl Format for TemperatureUnit {
//     fn format(&self, fmt: defmt::Formatter) {\
// 		let s = match self {
//             TemperatureUnit::Fahrenheit => ("Fahrenheit"),
//             TemperatureUnit::Celsius => "Celcius",
//             TemperatureUnit::Kelvin => "Kelvin",
//         };
// 		defmt::export::istr(s.into());
//         defmt::export::u8(s)
// 	}
// }

// pub struct Temperature {
// 	celcius: f32, // Celcius
// }

impl TemperatureUnit {
    pub fn convert_temperature(&self, temperature: f32) -> f32 {
        match self {
            TemperatureUnit::Fahrenheit => {
                temperature * 1.8 + 32f32
            },

            TemperatureUnit::Kelvin => {
                temperature + 273.15f32
            },

            TemperatureUnit::Celsius => {
                temperature
            },
        }
    }
}

// impl Temperature {
// 	pub fn new(celcius: f32) -> Self {
// 		Self { celcius }
// 	}

// 	pub fn to_fahrenheit(&self) -> f32 {
// 		self.celcius * 1.8 + 32f32
// 	}

// 	pub fn to_celcius(&self) -> f32 {
// 		self.celcius
// 	}

// 	pub fn to_kelvin(&self) -> f32 {
// 		self.celcius + 273.15f32
// 	}
// }

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
