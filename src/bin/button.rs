#![no_main]
#![no_std]

use hal::{
	gpio::{Input, Pin, PullUp},
	prelude::InputPin,
};
use nrf52840_co2 as _;
use nrf52840_hal::{self as hal};

pub struct Button(Pin<Input<PullUp>>);

impl Button {
	pub fn new<Mode>(pin: Pin<Mode>) -> Self {
		Button(pin.into_pullup_input())
	}

	pub fn is_pressed(&self) -> bool {
		self.0.is_low().unwrap()
	}
}
