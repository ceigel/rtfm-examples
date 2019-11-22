//! examples/idle.rs

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use cortex_m_semihosting::hprintln;
use panic_semihosting as _;
use stm32f3::stm32f303;

#[rtfm::app(device = stm32f303)]
const APP: () = {
    #[init]
    fn init(_: init::Context) {
        hprintln!("init").unwrap();
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        static mut X: u32 = 0;

        // Safe access to local `static mut` variable
        let _x: &'static mut u32 = X;

        hprintln!("idle").unwrap();

        // debug::exit(debug::EXIT_SUCCESS);
        hprintln!("EXIT_SUCCESS").unwrap();

        loop {}
    }
};
