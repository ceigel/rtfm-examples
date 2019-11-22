//! examples/schedule.rs

#![deny(warnings)]
#![no_main]
#![no_std]

use cortex_m_semihosting::hprintln;
use panic_semihosting as _;
use rtfm::cyccnt::{Instant, U32Ext as _};
use stm32f3::stm32f303;

// NOTE: does NOT work on QEMU!
#[rtfm::app(device = stm32f303, monotonic = rtfm::cyccnt::CYCCNT)]
const APP: () = {
    #[init(schedule = [foo, bar])]
    fn init(mut cx: init::Context) {
        // Initialize (enable) the monotonic timer (CYCCNT)
        cx.core.DCB.enable_trace();
        cx.core.DWT.enable_cycle_counter();

        // semantically, the monotonic timer is frozen at time "zero" during `init`
        let now = cx.start; // the start time of the system

        hprintln!("init @ {:?}", now).unwrap();

        // Schedule `foo` to run 8e6 cycles (clock cycles) in the future
        cx.schedule.foo(now + 8_000_000.cycles()).unwrap();

        // Schedule `bar` to run 4e6 cycles in the future
        cx.schedule.bar(now + 4_000_000.cycles()).unwrap();
    }

    #[task]
    fn foo(_: foo::Context) {
        hprintln!("foo  @ {:?}", Instant::now()).unwrap();
    }

    #[task]
    fn bar(_: bar::Context) {
        hprintln!("bar  @ {:?}", Instant::now()).unwrap();
    }

    extern "C" {
        fn SPI1();
    }
};
