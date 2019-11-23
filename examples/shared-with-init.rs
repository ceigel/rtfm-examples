//! `examples/shared-with-init.rs`

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use cortex_m_semihosting::hprintln;
use panic_semihosting as _;
use rtfm::app;
use stm32f3::stm32f303::{self, Interrupt};

pub struct MustBeSend;

#[app(device = stm32f303)]
const APP: () = {
    struct Resources {
        #[init(None)]
        shared: Option<MustBeSend>,
    }

    #[init(resources = [shared])]
    fn init(c: init::Context) {
        // this `message` will be sent to task `SPI1`
        let message = MustBeSend;
        *c.resources.shared = Some(message);

        rtfm::pend(Interrupt::SPI1);
    }

    #[task(binds = SPI1, resources = [shared])]
    fn spi1(c: spi1::Context) {
        if let Some(message) = c.resources.shared.take() {
            // `message` has been received
            drop(message);

            // debug::exit(debug::EXIT_SUCCESS);
            hprintln!("EXIT_SUCCESS").unwrap();
        }
    }
};
