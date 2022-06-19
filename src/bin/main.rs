#![no_main]
#![no_std]
mod rgb_led;

use embedded_hal::{blocking::delay::DelayMs, digital::v2::OutputPin};
use hal::gpio::{Output, Pin, PushPull};

use nrf52840_co2 as _;
use nrf52840_hal::{
	self as hal,
	gpio::{p0::Parts as P0Parts, Level},
	Timer,
	Temp,
};

use crate::rgb_led::{RgbLed, Color}; // global logger + panicking-behavior + memory layout

#[cortex_m_rt::entry]
fn main() -> ! {
	defmt::info!("Starting...");

	let board = hal::pac::Peripherals::take().unwrap();
	let mut timer = Timer::new(board.TIMER0);
	let mut temp = Temp::new(board.TEMP);

	let p0_pins = P0Parts::new(board.P0);
	let mut led_1 = p0_pins.p0_13.into_push_pull_output(Level::Low);
	let mut rgb = RgbLed::new(
		p0_pins.p0_03.into_push_pull_output(Level::High).into(),
		p0_pins.p0_04.into_push_pull_output(Level::High).into(),
		p0_pins.p0_28.into_push_pull_output(Level::High).into(),
	);

	let delay = 100u32;
	loop {
		led_1.set_high().unwrap();

		timer.delay_ms(delay);
		rgb.set_color(Color::Blue);

		timer.delay_ms(delay);
		rgb.set_color(Color::White);

		timer.delay_ms(delay);
		rgb.set_color(Color::Red);


		timer.delay_ms(delay);
		rgb.set_color(Color::Yellow);

		timer.delay_ms(delay);
		rgb.set_color(Color::Cyan);

		timer.delay_ms(delay);
		rgb.set_color(Color::Magenta);


		timer.delay_ms(delay);
		rgb.set_color(Color::Green);

		timer.delay_ms(delay);
		rgb.set_color(Color::Off);

		led_1.set_low().unwrap();
		timer.delay_ms(delay);

		let temperature: f32 = temp.measure().to_num();
		defmt::println!("{=f32} °C", temperature);
		defmt::debug!("{=f32} °C", temperature);
		defmt::info!("{=f32} °C", temperature);

	}
}
