use embedded_hal::digital::v2::OutputPin;
use hal::gpio::{Output, Pin, PushPull};
use nrf52840_hal::{self as hal};

pub enum Color {
	Off,

	Red,
	Green,
	Blue,

	Yellow,
	Cyan,
	Magenta,

	White,
}

pub struct RgbLed {
	r: Pin<Output<PushPull>>,
	g: Pin<Output<PushPull>>,
	b: Pin<Output<PushPull>>,
}

impl RgbLed {
	pub fn new(r: Pin<Output<PushPull>>, b: Pin<Output<PushPull>>, g: Pin<Output<PushPull>>) -> Self {
		Self { r, g, b }
	}

	pub fn set_color(&mut self, color: Color) {
		match color {
			Color::Off => {
				self.r.set_high().unwrap();
				self.b.set_high().unwrap();
				self.g.set_high().unwrap();
			}
			Color::Red => {
				self.r.set_low().unwrap();
				self.b.set_high().unwrap();
				self.g.set_high().unwrap();
			}
			Color::Green => {
				self.r.set_high().unwrap();
				self.b.set_high().unwrap();
				self.g.set_low().unwrap();
			}
			Color::Blue => {
				self.r.set_high().unwrap();
				self.b.set_low().unwrap();
				self.g.set_high().unwrap();
			}
			Color::Magenta => {
				self.r.set_low().unwrap();
				self.b.set_low().unwrap();
				self.g.set_high().unwrap();
			}
			Color::Yellow => {
				self.r.set_low().unwrap();
				self.b.set_high().unwrap();
				self.g.set_low().unwrap();
			}
			Color::Cyan => {
				self.r.set_high().unwrap();
				self.b.set_low().unwrap();
				self.g.set_low().unwrap();
			}
			Color::White => {
				self.r.set_low().unwrap();
				self.b.set_low().unwrap();
				self.g.set_low().unwrap();
			} // _ => {}
		}
	}
}
