#![no_main]
#![no_std]
mod button;
mod rgb_led;
mod temperature;

use embedded_hal::{blocking::delay::DelayMs, digital::v2::OutputPin};
use nrf52840_co2 as _;
use nrf52840_hal::{
	self as hal,
	gpio::{p0::Parts as P0Parts, Level},
	Temp, Timer,
};

use crate::{
	button::Button,
	rgb_led::{Color, RgbLed},
	temperature::TemperatureUnit,
}; // global logger + panicking-behavior + memory layout

#[cortex_m_rt::entry]
fn main() -> ! {
	defmt::info!("Starting...");

	let board = hal::pac::Peripherals::take().unwrap();
	let mut timer = Timer::new(board.TIMER0);
	let mut temp = Temp::new(board.TEMP);

	let p0_pins = P0Parts::new(board.P0);

	let button_1 = Button::new(p0_pins.p0_11.degrade());
	defmt::debug!("is pressed  {=bool}", button_1.is_pressed()); // first call return erroneously true
	defmt::debug!("is pressed  {=bool}", button_1.is_pressed()); // second false...

	let mut led_1 = p0_pins.p0_13.into_push_pull_output(Level::Low);
	let mut rgb = RgbLed::new(
		p0_pins.p0_03.into_push_pull_output(Level::High).into(),
		p0_pins.p0_04.into_push_pull_output(Level::High).into(),
		p0_pins.p0_28.into_push_pull_output(Level::High).into(),
	);

	let delay = 1000u32;
	let mut current_unit = TemperatureUnit::Celsius;

	loop {
		led_1.set_high().unwrap();

		if button_1.is_pressed() {
			current_unit = match current_unit {
				TemperatureUnit::Fahrenheit => TemperatureUnit::Kelvin,
				TemperatureUnit::Kelvin => TemperatureUnit::Celsius,
				TemperatureUnit::Celsius => TemperatureUnit::Fahrenheit,
			};
			// defmt::info!("Unit changed to {=?}", current_unit);
			defmt::info!("Unit changed");
		};

		// if button_1.is_pressed() {

		timer.delay_ms(delay / 10);
		rgb.set_color(Color::Blue);

		timer.delay_ms(delay / 10);
		rgb.set_color(Color::White);

		timer.delay_ms(delay / 10);
		rgb.set_color(Color::Red);

		timer.delay_ms(delay / 10);
		rgb.set_color(Color::Yellow);

		timer.delay_ms(delay / 10);
		rgb.set_color(Color::Cyan);

		timer.delay_ms(delay / 10);
		rgb.set_color(Color::Magenta);

		timer.delay_ms(delay / 10);
		rgb.set_color(Color::Green);

		timer.delay_ms(delay / 10);
		rgb.set_color(Color::Off);
		// } else {
		// 	timer.delay_ms(delay);
		// }

		led_1.set_low().unwrap();
		timer.delay_ms(delay / 10);

		let temperature: f32 = temp.measure().to_num();
		match current_unit {
			TemperatureUnit::Fahrenheit => defmt::info!("{=f32} °F", current_unit.convert_temperature(temperature)),
			TemperatureUnit::Celsius => defmt::info!("{=f32} °C", current_unit.convert_temperature(temperature)),
			TemperatureUnit::Kelvin => defmt::info!("{=f32} °K", current_unit.convert_temperature(temperature)),
		};
	}
}
                                                                                                        