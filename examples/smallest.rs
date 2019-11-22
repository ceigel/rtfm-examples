//! examples/smallest.rs

#![no_main]
#![no_std]

extern crate cortex_m_semihosting;
extern crate panic_semihosting;
extern crate rtfm;
extern crate stm32f3;
use rtfm::app;
use stm32f3::stm32f303;

#[app(device = stm32f303)]
const APP: () = {};
