//! examples/preempt.rs

#![no_main]
#![no_std]

use cortex_m_semihosting::hprintln;
use panic_semihosting as _;
use stm32f3::stm32f303::{self, Interrupt};

#[rtfm::app(device = stm32f303)]
const APP: () = {
    #[init]
    fn init(_: init::Context) {
        rtfm::pend(Interrupt::EXTI0);
    }

    #[task(binds = EXTI0, priority = 1)]
    fn gpioa(_: gpioa::Context) {
        hprintln!("EXTI0 - start").unwrap();
        rtfm::pend(Interrupt::EXTI2_TSC);
        hprintln!("EXTI0 - end").unwrap();
        // debug::exit(debug::EXIT_SUCCESS);
        hprintln!("EXIT_SUCCESS").unwrap();
    }

    #[task(binds = EXTI1, priority = 2)]
    fn gpiob(_: gpiob::Context) {
        hprintln!(" EXTI1").unwrap();
    }

    #[task(binds = EXTI2_TSC, priority = 2)]
    fn gpioc(_: gpioc::Context) {
        hprintln!(" EXTI2_TSC - start").unwrap();
        rtfm::pend(Interrupt::EXTI1);
        hprintln!(" EXTI2_TSC - end").unwrap();
    }
};
