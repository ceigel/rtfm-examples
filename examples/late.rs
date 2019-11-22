//! examples/late.rs

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use cortex_m_semihosting::hprintln;
use heapless::{
    consts::*,
    i,
    spsc::{Consumer, Producer, Queue},
};
use panic_semihosting as _;
use stm32f3::stm32f303::{self, Interrupt};

#[rtfm::app(device = stm32f303)]
const APP: () = {
    // Late resources
    struct Resources {
        p: Producer<'static, u32, U4>,
        c: Consumer<'static, u32, U4>,
    }

    #[init]
    fn init(_: init::Context) -> init::LateResources {
        static mut Q: Queue<u32, U4> = Queue(i::Queue::new());

        let (p, c) = Q.split();

        // Initialization of late resources
        init::LateResources { p, c }
    }

    #[idle(resources = [c])]
    fn idle(c: idle::Context) -> ! {
        loop {
            if let Some(byte) = c.resources.c.dequeue() {
                hprintln!("received message: {}", byte).unwrap();

            // debug::exit(debug::EXIT_SUCCESS);
            } else {
                rtfm::pend(Interrupt::SPI1);
            }
        }
    }

    #[task(binds = SPI1, resources = [p])]
    fn spi1(c: spi1::Context) {
        c.resources.p.enqueue(42).unwrap();
    }
};
