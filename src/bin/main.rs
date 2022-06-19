#![no_main]
#![cfg_attr(not(test), no_std)]

use cortex_m::prelude::_embedded_hal_timer_CountDown;
use embedded_hal::digital::v2::OutputPin;
use nb::block;
use nrf52840_co2::{button::Button, rgb_led::*, scd30::SCD30, settings::*, temperature::TemperatureUnit, buzzer::Buzzer};
use nrf52840_hal::{
	self as hal,
	gpio::{p0::Parts as P0Parts, Level},
	Temp, Timer,
};

#[cortex_m_rt::entry]
fn main() -> ! {
	defmt::info!("Starting...");

	let board = hal::pac::Peripherals::take().unwrap();
	let mut periodic_timer = Timer::periodic(board.TIMER0);
	let mut timer = Timer::new(board.TIMER1);

	let mut temp = Temp::new(board.TEMP);

	let p0_pins = P0Parts::new(board.P0);

	let mut button_1 = Button::new(p0_pins.p0_11.degrade());
	defmt::debug!("is pressed  {=bool}", button_1.is_pressed()); // first call return erroneously true
	defmt::debug!("is pressed  {=bool}", button_1.is_pressed()); // second false...

	let mut led_1 = p0_pins.p0_13.into_push_pull_output(Level::Low);

	let pin_buzzer = p0_pins.p0_29.into_push_pull_output(Level::Low).degrade();
	let mut buzzer = Buzzer::init(pin_buzzer);
    buzzer.noise_variable(&mut timer, 440_u32, 300_u32);
    buzzer.noise_variable(&mut timer, 660_u32, 100_u32);

	let scl = p0_pins.p0_30.into_floating_input().degrade();
	let sda = p0_pins.p0_31.into_floating_input().degrade();

	let mut sdc30 = SCD30::init(board.TWIM0, scl, sda);
	let fw_version = sdc30.get_firmware_version().unwrap();
	defmt::info!("SCD30 Firmware v{=u8}.{=u8}", fw_version[0], fw_version[1]);

	let mut rgb = RgbLed::new(
		p0_pins.p0_03.into_push_pull_output(Level::High).into(),
		p0_pins.p0_04.into_push_pull_output(Level::High).into(),
		p0_pins.p0_28.into_push_pull_output(Level::High).into(),
	);

	let mut current_unit = TemperatureUnit::Celsius;
	let mut millis: u64 = 0;

	sdc30.start_continuous_measurement(PRESSURE).unwrap();

	loop {
		periodic_timer.start(1000_u32);

		if (millis % 1000) == 0 {
			// defmt::debug("Tick (milliseconds): {=u64}", millis);
			led_1.set_low().unwrap();
			let temp: f32 = TEMP_ADJUST + temp.measure().to_num::<f32>();
			match current_unit {
				TemperatureUnit::Fahrenheit => defmt::info!("{=f32} °F", current_unit.convert_temperature(temp)),
				TemperatureUnit::Celsius => defmt::info!("{=f32} °C", current_unit.convert_temperature(temp)),
				TemperatureUnit::Kelvin => defmt::info!("{=f32} °K", current_unit.convert_temperature(temp)),
			};

			match temp {
				t if t < TEMP_THRESHOLD_COLD => rgb.set_color(Color::White),
				t if t >= TEMP_THRESHOLD_COLD && t < TEMP_THRESHOLD_GOOD => rgb.set_color(Color::Blue),
				t if t >= TEMP_THRESHOLD_GOOD && t < TEMP_THRESHOLD_WARM => rgb.set_color(Color::Green),
				t if t >= TEMP_THRESHOLD_WARM && t < TEMP_THRESHOLD_HOT => rgb.set_color(Color::Yellow),
				t if t >= TEMP_THRESHOLD_HOT => rgb.set_color(Color::Red),
				_ => {}
			};
			led_1.set_high().unwrap();
		};
		if (millis % 5) == 0 {
			if button_1.check_rising_edge() {
				current_unit = match current_unit {
					TemperatureUnit::Fahrenheit => TemperatureUnit::Kelvin,
					TemperatureUnit::Kelvin => TemperatureUnit::Celsius,
					TemperatureUnit::Celsius => TemperatureUnit::Fahrenheit,
				};
				// defmt::info!("Unit changed to {=?}", current_unit);
				defmt::debug!("Unit changed");
			};
		};

		loop {
			if sdc30.data_ready().unwrap() {
				defmt::info!("Data ready.");
				break;
			}
		}

		let result = sdc30.read_measurement().unwrap();

		let co2 = result.co2;
		let temp = result.temperature;
		let humidity = result.humidity;

		defmt::info!(
			"
				CO2 {=f32} ppm
				Temperature {=f32} °C
				Humidity {=f32} %
				",
			co2,
			temp,
			humidity
		);

		block!(periodic_timer.wait()).unwrap();
		millis = millis.saturating_add(1);
	}
}
