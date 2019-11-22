//! examples/message.rs

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use cortex_m_semihosting::hprintln;
use panic_semihosting as _;
use stm32f3::stm32f303;

#[rtfm::app(device = stm32f303)]
const APP: () = {
    #[init(spawn = [foo])]
    fn init(c: init::Context) {
        c.spawn.foo(/* no message */).unwrap();
    }

    #[task(spawn = [bar])]
    fn foo(c: foo::Context) {
        static mut COUNT: u32 = 0;

        hprintln!("foo").unwrap();

        c.spawn.bar(*COUNT).unwrap();
        *COUNT += 1;
    }

    #[task(spawn = [baz])]
    fn bar(c: bar::Context, x: u32) {
        hprintln!("bar({})", x).unwrap();

        c.spawn.baz(x + 1, x + 2).unwrap();
    }

    #[task(spawn = [foo])]
    fn baz(c: baz::Context, x: u32, y: u32) {
        hprintln!("baz({}, {})", x, y).unwrap();

        if x + y > 4 {
            // debug::exit(debug::EXIT_SUCCESS);
            hprintln!("EXIT_SUCCESS").unwrap();
        } else {
            c.spawn.foo().unwrap();
        }
    }

    extern "C" {
        fn EXTI0();
    }
};
