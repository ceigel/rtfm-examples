//! examples/static.rs

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
        key: u32,
    }

    #[init]
    fn init(_: init::Context) -> init::LateResources {
        rtfm::pend(Interrupt::SPI1);
        rtfm::pend(Interrupt::SPI2);

        init::LateResources { key: 0xdeadbeef }
    }

    #[task(binds = SPI1, resources = [&key])]
    fn spi1(cx: spi1::Context) {
        let key: &u32 = cx.resources.key;
        hprintln!("SPI1(key = {:#x})", key).unwrap();

        // debug::exit(debug::EXIT_SUCCESS);
        hprintln!("EXIT_SUCCESS").unwrap();
    }

    #[task(binds = SPI2, priority = 2, resources = [&key])]
    fn spi2(cx: spi2::Context) {
        hprintln!("SPI2(key = {:#x})", cx.resources.key).unwrap();
    }
};
