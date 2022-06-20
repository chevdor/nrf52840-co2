#![no_main]
#![no_std]
#![deny(unsafe_code)]

#[rtic::app(device = hal::pac, peripherals = true, dispatchers = [SWI0_EGU0])]
mod app {
	use embedded_hal::digital::v2::InputPin;
	pub use fugit::Duration;
	use nrf52840_co2 as _;
	use systick_monotonic::*;
	use {
		hal::{
			gpio::{Input, Level, Output, Pin, PullUp, PushPull},
			gpiote::*,
			// PPI allows telling the microcontroller how to handle the perfipherals, without interefering with the cpu
			// https://infocenter.nordicsemi.com/index.jsp?topic=%2Fcom.nordic.infocenter.nrf52832.ps.v1.1%2Fppi.html
			ppi::{self, ConfigurablePpi, Ppi},
		},
		nrf52840_hal as hal,
	};

	#[shared]
	struct Shared {
		gpiote: Gpiote,
	}

	#[local]
	struct Local {
		btn_1: Pin<Input<PullUp>>,
		btn_2: Pin<Input<PullUp>>,
		btn_3: Pin<Input<PullUp>>,
		btn_4: Pin<Input<PullUp>>,
		// led1 is managed via ppi
		led2: Pin<Output<PushPull>>,
		led2_state: bool,
	}

	#[monotonic(binds = SysTick, default = true)]
	type MonoTimer = Systick<1000>;

	#[init]
	fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {
		let port_0 = hal::gpio::p0::Parts::new(ctx.device.P0);

		let led1 = port_0.p0_13.into_push_pull_output(Level::High).degrade();
		let led2 = port_0.p0_14.into_push_pull_output(Level::High).degrade();
		let mono = Systick::new(ctx.core.SYST, 36_000_000);

		let btn_1 = port_0.p0_11.into_pullup_input().degrade();
		let btn_2 = port_0.p0_12.into_pullup_input().degrade();
		let btn_3 = port_0.p0_24.into_pullup_input().degrade();
		let btn_4 = port_0.p0_25.into_pullup_input().degrade();

		// see https://github.com/nrf-rs/nrf-hal/blob/663008c033ad67263e4ac0c561d5d1fac28d7170/examples/gpiote-demo/src/main.rs
		let gpiote = Gpiote::new(ctx.device.GPIOTE);

		// Set btn1 to generate event on channel 0 and enable interrupt
		gpiote.channel0().input_pin(&btn_1).hi_to_lo().enable_interrupt();
		gpiote.port().input_pin(&btn_3).low();
		gpiote.port().input_pin(&btn_4).low();

		// Enable interrupt for port event
		gpiote.port().enable_interrupt();

		// PPI usage, channel 2 event triggers "task out" operation (toggle) on channel 1 (toggles led1)
		gpiote.channel1().output_pin(led1).task_out_polarity(TaskOutPolarity::Toggle).init_high();
		gpiote.channel2().input_pin(&btn_2).hi_to_lo();
		let ppi_channels = ppi::Parts::new(ctx.device.PPI);
		let mut ppi0 = ppi_channels.ppi0;
		ppi0.set_task_endpoint(gpiote.channel1().task_out());
		ppi0.set_event_endpoint(gpiote.channel2().event());
		ppi0.enable();

		#[cfg(feature = "semihosting")]
		hprintln!("starting").unwrap();

		defmt::println!("Now press the buttons 1,2,3 and 4. Observe how the output and how pressing buttons 1 and 2 toggle the LED where pressing 3 and 4 only turn it on");
		blink1::spawn_after(Duration::<u64, 1, 1000>::from_ticks(100)).unwrap();
		(Shared { gpiote }, Local { btn_1, btn_2, btn_3, btn_4, led2, led2_state: false }, init::Monotonics(mono))
	}

	#[task(local = [led2, led2_state])]
	fn blink1(cx: blink1::Context) {
		if *cx.local.led2_state {
			// let mut led2 = cx.local.led2.into_push_pull_output(Level::High);
			let _ = hal::prelude::OutputPin::set_high(cx.local.led2);
			*cx.local.led2_state = false;
		} else {
			// let mut led2 = cx.local.led2.into_push_pull_output(Level::High);
			let _ = hal::prelude::OutputPin::set_low(cx.local.led2);
			*cx.local.led2_state = true;
		}

		blink1::spawn_after(Duration::<u64, 1, 1000>::from_ticks(250)).unwrap();
	}

	#[task(binds = GPIOTE, shared = [gpiote])]
	fn on_gpiote(mut ctx: on_gpiote::Context) {
		ctx.shared.gpiote.lock(|gpiote| {
			if gpiote.channel0().is_event_triggered() {
				defmt::println!("Interrupt from channel 0 event");
			}
			if gpiote.port().is_event_triggered() {
				defmt::println!("Interrupt from port event");
			}
			// Reset all events
			gpiote.reset_events();
			// Debounce
			debounce::spawn_after(50.millis()).ok();
		});
	}

	#[task(shared = [gpiote], local = [btn_1, btn_2, btn_3, btn_4])]
	fn debounce(mut ctx: debounce::Context) {
		let btn1_pressed = ctx.local.btn_1.is_low().unwrap();
		let btn2_pressed = ctx.local.btn_2.is_low().unwrap();
		let btn3_pressed = ctx.local.btn_3.is_low().unwrap();
		let btn4_pressed = ctx.local.btn_4.is_low().unwrap();

		ctx.shared.gpiote.lock(|gpiote| {
			if btn1_pressed {
				defmt::println!("Button 1 was pressed!");
				// Manually run "task out" operation (toggle) on channel 1 (toggles led1)
				gpiote.channel1().out();
			}
			if btn2_pressed {
				defmt::println!("Button 2 was pressed!");
				// Manually run "task clear" on channel 1 (led1 on)
				gpiote.channel1().clear();
			}
			if btn3_pressed {
				defmt::println!("Button 3 was pressed!");
				// Manually run "task clear" on channel 1 (led1 on)
				gpiote.channel1().clear();
			}
			if btn4_pressed {
				defmt::println!("Button 4 was pressed!");
				// Manually run "task clear" on channel 1 (led1 on)
				gpiote.channel1().clear();
			}
		});
	}
}
