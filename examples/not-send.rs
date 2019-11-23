//! `examples/not-send.rs`

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use core::marker::PhantomData;

use cortex_m_semihosting::hprintln;
use panic_semihosting as _;
use rtfm::app;
use stm32f3::stm32f303;

pub struct NotSend {
    _0: PhantomData<*const ()>,
}

#[app(device = stm32f303)]
const APP: () = {
    struct Resources {
        #[init(None)]
        shared: Option<NotSend>,
    }

    #[init(spawn = [baz, quux])]
    fn init(c: init::Context) {
        c.spawn.baz().unwrap();
        c.spawn.quux().unwrap();
    }

    #[task(spawn = [bar])]
    fn foo(c: foo::Context) {
        // scenario 1: message passed to task that runs at the same priority
        c.spawn.bar(NotSend { _0: PhantomData }).ok();
    }

    #[task]
    fn bar(_: bar::Context, _x: NotSend) {
        // scenario 1
    }

    #[task(priority = 2, resources = [shared])]
    fn baz(c: baz::Context) {
        // scenario 2: resource shared between tasks that run at the same priority
        *c.resources.shared = Some(NotSend { _0: PhantomData });
    }

    #[task(priority = 2, resources = [shared])]
    fn quux(c: quux::Context) {
        // scenario 2
        let _not_send = c.resources.shared.take().unwrap();

        // debug::exit(debug::EXIT_SUCCESS);
        hprintln!("EXIT_SUCCESS").unwrap();
    }

    extern "C" {
        fn SPI1();
        fn SPI2();
    }
};
