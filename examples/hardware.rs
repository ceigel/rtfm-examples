//! examples/hardware.rs

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use cortex_m_semihosting::hprintln;
use panic_semihosting as _;
use stm32f3::stm32f303::{self, Interrupt};

#[rtfm::app(device = stm32f303)]
const APP: () = {
    #[init]
    fn init(_: init::Context) {
        // Pends the SPI1 interrupt but its handler won't run until *after*
        // `init` returns because interrupts are disabled
        rtfm::pend(Interrupt::SPI1); // equivalent to NVIC::pend

        hprintln!("init").unwrap();
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        // interrupts are enabled again; the `SPI1` handler runs at this point

        hprintln!("idle").unwrap();

        rtfm::pend(Interrupt::SPI1);

        // debug::exit(debug::EXIT_SUCCESS);
        hprintln!("EXIT_SUCCESS").unwrap();

        loop {}
    }

    #[task(binds = SPI1)]
    fn spi1(_: spi1::Context) {
        static mut TIMES: u32 = 0;

        // Safe access to local `static mut` variable
        *TIMES += 1;

        hprintln!(
            "SPI1 called {} time{}",
            *TIMES,
            if *TIMES > 1 { "s" } else { "" }
        )
        .unwrap();
    }
};
