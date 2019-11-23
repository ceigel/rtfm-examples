//! examples/types.rs

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use panic_semihosting as _;
use rtfm::cyccnt;
use stm32f3::stm32f303;

#[rtfm::app(device = stm32f3::stm32f303, peripherals = true, monotonic = rtfm::cyccnt::CYCCNT)]
const APP: () = {
    struct Resources {
        #[init(0)]
        shared: u32,
    }

    #[init(schedule = [foo], spawn = [foo])]
    fn init(cx: init::Context) {
        let _: cyccnt::Instant = cx.start;
        let _: rtfm::Peripherals = cx.core;
        let _: stm32f303::Peripherals = cx.device;
        let _: init::Schedule = cx.schedule;
        let _: init::Spawn = cx.spawn;
    }

    #[idle(schedule = [foo], spawn = [foo])]
    fn idle(cx: idle::Context) -> ! {
        let _: idle::Schedule = cx.schedule;
        let _: idle::Spawn = cx.spawn;

        loop {}
    }

    #[task(binds = SPI1, resources = [shared], schedule = [foo], spawn = [foo])]
    fn spi1(cx: spi1::Context) {
        let _: cyccnt::Instant = cx.start;
        let _: resources::shared = cx.resources.shared;
        let _: spi1::Schedule = cx.schedule;
        let _: spi1::Spawn = cx.spawn;
    }

    #[task(priority = 2, resources = [shared], schedule = [foo], spawn = [foo])]
    fn foo(cx: foo::Context) {
        let _: cyccnt::Instant = cx.scheduled;
        let _: &mut u32 = cx.resources.shared;
        let _: foo::Resources = cx.resources;
        let _: foo::Schedule = cx.schedule;
        let _: foo::Spawn = cx.spawn;
    }

    extern "C" {
        fn SPI2();
    }
};
