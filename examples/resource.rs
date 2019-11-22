//! examples/resource.rs

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use cortex_m_semihosting::hprintln;
use panic_semihosting as _;
use stm32f3::stm32f303::{self, Interrupt};

#[rtfm::app(device = stm32f303)]
const APP: () = {
    struct Resources {
        // A resource
        #[init(0)]
        shared: u32,
    }

    #[init]
    fn init(_: init::Context) {
        rtfm::pend(Interrupt::SPI1);
        rtfm::pend(Interrupt::SPI2);
    }

    // `shared` cannot be accessed from this context
    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        // debug::exit(debug::EXIT_SUCCESS);
        hprintln!("EXIT_SUCCESS").unwrap();

        // error: no `resources` field in `idle::Context`
        // _cx.resources.shared += 1;

        loop {}
    }

    // `shared` can be accessed from this context
    #[task(binds = SPI1, resources = [shared])]
    fn spi1(cx: spi1::Context) {
        let shared: &mut u32 = cx.resources.shared;
        *shared += 1;

        hprintln!("SPI1: shared = {}", shared).unwrap();
    }

    // `shared` can be accessed from this context
    #[task(binds = SPI2, resources = [shared])]
    fn uart1(cx: uart1::Context) {
        *cx.resources.shared += 1;

        hprintln!("SPI2: shared = {}", cx.resources.shared).unwrap();
    }
};
