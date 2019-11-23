//! `examples/not-sync.rs`

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use core::marker::PhantomData;

use cortex_m_semihosting::hprintln;
use panic_semihosting as _;
use stm32f3::stm32f303;

pub struct NotSync {
    _0: PhantomData<*const ()>,
}

#[rtfm::app(device = stm32f303)]
const APP: () = {
    struct Resources {
        #[init(NotSync { _0: PhantomData })]
        shared: NotSync,
    }

    #[init(spawn=[foo, bar])]
    fn init(_: init::Context) {
        // debug::exit(debug::EXIT_SUCCESS);
        hprintln!("EXIT_SUCCESS").unwrap();
    }

    #[task(resources = [&shared])]
    fn foo(c: foo::Context) {
        let _: &NotSync = c.resources.shared;
    }

    #[task(resources = [&shared])]
    fn bar(c: bar::Context) {
        let _: &NotSync = c.resources.shared;
    }

    extern "C" {
        fn SPI1();
    }
};
