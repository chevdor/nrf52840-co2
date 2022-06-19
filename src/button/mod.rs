use hal::{
	gpio::{Input, Pin, PullUp},
	prelude::InputPin,
};
use nrf52840_hal::{self as hal};

pub struct Button {
	pin: Pin<Input<PullUp>>,
	was_pressed: bool,
}

impl Button {
	pub fn new<Mode>(pin: Pin<Mode>) -> Self {
		Self { pin: pin.into_pullup_input(), was_pressed: false }
	}

	pub fn is_pressed(&self) -> bool {
		self.pin.is_low().unwrap()
	}

	pub fn check_rising_edge(&mut self) -> bool {
		let mut rising_edge = false;

		let is_pressed = self.is_pressed();
		// Only trigger on "rising edge" of the signal
		// Term: "Edge Triggering"
		if self.was_pressed && !is_pressed {
			// Was pressed, now isn't:
			rising_edge = true;
		}
		self.was_pressed = is_pressed;
		rising_edge
	}
}
