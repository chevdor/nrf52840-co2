pub enum TemperatureUnit {
	Fahrenheit,
	Celsius,
	Kelvin,
}

impl TemperatureUnit {
	pub fn convert_temperature(&self, temperature: f32) -> f32 {
		match self {
			TemperatureUnit::Fahrenheit => temperature * 1.8 + 32f32,

			TemperatureUnit::Kelvin => temperature + 273.15f32,

			TemperatureUnit::Celsius => temperature,
		}
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
