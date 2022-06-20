#![no_main]
#![no_std]
#![deny(unsafe_code)]

// pub use panic_halt as _;

// pub use stm32f4xx_hal::gpio::gpioa::{PA0, PA1};
// pub use stm32f4xx_hal::gpio::gpiog::{PG13, PG14};
// pub use stm32f4xx_hal::gpio::*;
// pub use stm32f4xx_hal::{
// 	gpio::{Edge, Input, Output, PushPull},
// 	prelude::*,
// };

#[rtic::app(device = nrf52840_hal::pac, peripherals = true, dispatchers = [TIMER0])]
mod app {
	// use crate::prelude::*;
	// use cortex_m::prelude::_embedded_hal_timer_CountDown;
	use embedded_hal::digital::v2::OutputPin;
	use hal::gpio::p0::{self, P0_11, P0_13, P0_14};
	use hal::gpio::{Input, Output, PullDown, PushPull};

	use hal::pac::P0;
	// use nb::block;
	// use nrf52840_co2::{
	// 	button::Button, buzzer::Buzzer, rgb_led::*, scd30::SCD30, settings::*, temperature::TemperatureUnit,
	// };

	use nrf52840_co2::{
		button::Button, buzzer::Buzzer, rgb_led::*, scd30::SCD30, settings::*, temperature::TemperatureUnit,
	};

	pub use fugit::Duration;
	use nrf52840_hal::{
		self as hal,
		gpio::{p0::Parts as P0Parts, Level},
	};

	pub use systick_monotonic::Systick;

	#[shared]
	struct Shared {}

	#[local]
	struct Local {
		button: P0_11<Input<PullDown>>,
		state: bool,
		led1: P0_13<Output<PushPull>>,
		led1_state: bool,
		led2: P0_14<Output<PushPull>>,
		led2_state: bool,
		// logger: Logger,
		// ir_gate_1: IRGate1,
	}

	#[monotonic(binds = SysTick, default = true)]
	type MonoTimer = Systick<1000>;

	#[init]
	fn init(mut ctx: init::Context) -> (Shared, Local, init::Monotonics) {
		// let core: cortex_m::Peripherals = ctx.core;
		// let mut logger = Logger::new(ctx.core.ITM);
		// logger.log("start");
		// let mut syscfg = ctx.device.SYSCFG.constrain();

		let port_0 = P0Parts::new(ctx.device.P0);

		// let gpioa = ctx.device.GPIOA.split();
		// let gpiog = ctx.device.GPIOG.split();

		let led1 = port_0.p0_13.into_push_pull_output(Level::Low);
		let led2 = port_0.p0_14.into_push_pull_output(Level::Low);
		let mono = Systick::new(ctx.core.SYST, 36_000_000);

		let mut button = port_0.p0_11.into_pulldown_input();
		// button.make_interrupt_source(&mut syscfg);
		// button.enable_interrupt(&mut ctx.device.TIMER2);
		// button.trigger_on_edge(&mut ctx.device.TIMER2, Edge::Rising);

		// let mut ir_gate_1 = IRGate1 { pin: gpioa.pa1.into_pull_down_input(), state: IRGateState::Close };
		// ir_gate_1.pin.make_interrupt_source(&mut syscfg);
		// ir_gate_1.pin.enable_interrupt(&mut ctx.device.EXTI);
		// ir_gate_1.pin.trigger_on_edge(&mut ctx.device.EXTI, Edge::Rising);

		#[cfg(feature = "semihosting")]
		hprintln!("starting").unwrap();

		blink1::spawn_after(Duration::<u64, 1, 1000>::from_ticks(100)).unwrap();
		(
			Shared {},
			Local {
				button,
				state: false,
				led1,
				led1_state: false,
				led2,
				led2_state: false,
				// ir_gate_1,
				// 	logger,
			},
			init::Monotonics(mono),
		)
	}

	#[task(local = [led1, led1_state])]
	fn blink1(cx: blink1::Context) {
		if *cx.local.led1_state {
			let _ = cx.local.led1.set_low();
			*cx.local.led1_state = false;
		} else {
			let _ = cx.local.led1.set_high();
			*cx.local.led1_state = true;
		}

		blink1::spawn_after(Duration::<u64, 1, 1000>::from_ticks(250)).unwrap();
	}

	// #[task(binds = TIMER1, local = [button, state, led2], priority = 2)]
	// fn btn_user(cx: btn_user::Context) {
	// 	cx.local.button.clear_interrupt_pending_bit();
	// 	if *cx.local.state {
	// 		cx.local.led2.set_low();
	// 		*cx.local.state = false;
	// 	} else {
	// 		cx.local.led2.set_high();
	// 		*cx.local.state = true;
	// 	}
	// }

	// #[idle( local = [logger])]
	// fn idle(cx: idle::Context) -> ! {
	// 	#[cfg(feature = "semihosting")]
	// 	hprintln!("idle").unwrap();
	// 	cx.local.logger.log("idle");
	// 	// cx.local.led2.toggle();
	// 	loop {}
	// }
}
