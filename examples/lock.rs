//! examples/lock.rs

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
        #[init(0)]
        shared: u32,
    }

    #[init]
    fn init(_: init::Context) {
        rtfm::pend(Interrupt::EXTI0);
    }

    // when omitted priority is assumed to be `1`
    #[task(binds = EXTI0, resources = [shared])]
    fn exti0(mut c: exti0::Context) {
        hprintln!("A").unwrap();

        // the lower priority task requires a critical section to access the data
        c.resources.shared.lock(|shared| {
            // data can only be modified within this critical section (closure)
            *shared += 1;

            // EXTI1 will *not* run right now due to the critical section
            rtfm::pend(Interrupt::EXTI1);

            hprintln!("B - shared = {}", *shared).unwrap();

            // EXTI3 does not contend for `shared` so it's allowed to run now
            rtfm::pend(Interrupt::EXTI3);
        });

        // critical section is over: EXTI1 can now start

        hprintln!("E").unwrap();

        // debug::exit(debug::EXIT_SUCCESS);
        hprintln!("EXIT_SUCCESS").unwrap();
    }

    #[task(binds = EXTI1, priority = 2, resources = [shared])]
    fn exti1(c: exti1::Context) {
        // the higher priority task does *not* need a critical section
        *c.resources.shared += 1;

        hprintln!("D - shared = {}", *c.resources.shared).unwrap();
    }

    #[task(binds = EXTI3, priority = 3)]
    fn exti3(_: exti3::Context) {
        hprintln!("C").unwrap();
    }
};
