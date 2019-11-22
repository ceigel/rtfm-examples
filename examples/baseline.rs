//! examples/baseline.rs

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use cortex_m_semihosting::hprintln;
use panic_semihosting as _;
use stm32f3::stm32f303::{self, Interrupt};

// NOTE: does NOT properly work on QEMU
#[rtfm::app(device = stm32f303, monotonic = rtfm::cyccnt::CYCCNT)]
const APP: () = {
    #[init(spawn = [foo])]
    fn init(cx: init::Context) {
        // omitted: initialization of `CYCCNT`

        hprintln!("init(baseline = {:?})", cx.start).unwrap();

        // `foo` inherits the baseline of `init`: `Instant(0)`
        cx.spawn.foo().unwrap();
    }

    #[task(schedule = [foo])]
    fn foo(cx: foo::Context) {
        static mut ONCE: bool = true;

        hprintln!("foo(baseline = {:?})", cx.scheduled).unwrap();

        if *ONCE {
            *ONCE = false;

            rtfm::pend(Interrupt::SPI1);
        } else {
            // debug::exit(debug::EXIT_SUCCESS);
            hprintln!("EXIT_SUCCESS").unwrap();
        }
    }

    #[task(binds = SPI1, spawn = [foo])]
    fn spi1(cx: spi1::Context) {
        hprintln!("SPI1(baseline = {:?})", cx.start).unwrap();

        // `foo` inherits the baseline of `SPI1`: its `start` time
        cx.spawn.foo().unwrap();
    }

    extern "C" {
        fn SPI2();
    }
};
