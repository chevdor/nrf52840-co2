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
	temperature::Temperature,
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
	loop {
		led_1.set_high().unwrap();

		if button_1.is_pressed() {
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
		} else {
			timer.delay_ms(delay);
		}

		led_1.set_low().unwrap();
		timer.delay_ms(delay / 10);

		let temperature: f32 = temp.measure().to_num();
		defmt::info!("{=f32} °C", Temperature::new(temperature).to_celcius()); // we could have used `temperature` directly as well
		defmt::info!("{=f32} °F", Temperature::new(temperature).to_fahrenheit());
		defmt::info!("{=f32} °K", Temperature::new(temperature).to_kelvin());
	}
}
