//! examples/task.rs

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
        c.spawn.foo().unwrap();
    }

    #[task(spawn = [bar, baz])]
    fn foo(c: foo::Context) {
        hprintln!("foo - start").unwrap();

        // spawns `bar` onto the task scheduler
        // `foo` and `bar` have the same priority so `bar` will not run until
        // after `foo` terminates
        c.spawn.bar().unwrap();

        hprintln!("foo - middle").unwrap();

        // spawns `baz` onto the task scheduler
        // `baz` has higher priority than `foo` so it immediately preempts `foo`
        c.spawn.baz().unwrap();

        hprintln!("foo - end").unwrap();
    }

    #[task]
    fn bar(_: bar::Context) {
        hprintln!("bar").unwrap();

        // debug::exit(debug::EXIT_SUCCESS);
        hprintln!("EXIT_SUCCESS").unwrap();
    }

    #[task(priority = 2)]
    fn baz(_: baz::Context) {
        hprintln!("baz").unwrap();
    }

    // Interrupt handlers used to dispatch software tasks
    extern "C" {
        fn SPI2();
        fn SPI1();
    }
};
