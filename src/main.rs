#![no_std]
#![no_main]
#![feature(impl_trait_in_assoc_type)]

use core::{
    fmt::Write,
    sync::atomic::{AtomicU32, Ordering},
};

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::{
    dma::NoDma,
    exti::ExtiInput,
    gpio::{AnyPin, Input, Level, Output, Pin, Pull, Speed},
    spi::{Config as SpiConfig, Instance, Spi},
    usart::{Config as UsartConfig, UartTx},
};
use embassy_time::{Duration, Timer};
use heapless::String;
use {defmt_rtt as _, panic_probe as _};

static BLINK_MS: AtomicU32 = AtomicU32::new(0);

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Hello World!");

    let button = Input::new(p.PC13, Pull::None);
    let mut button = ExtiInput::new(button, p.EXTI13);

    let mut delay_var = 2000;
    BLINK_MS.store(delay_var, Ordering::Relaxed);

    spawner.spawn(led_task(p.PA5.degrade())).unwrap();

    let mut usart = UartTx::new(p.USART2, p.PA2, NoDma, UsartConfig::default()).unwrap();
    let mut value: u8 = 0;
    let mut msg: String<4> = String::new();

    loop {
        button.wait_for_rising_edge().await;

        delay_var /= 10;
        if delay_var < 200 {
            delay_var = 2000;
        }

        info!("delay_var at: {}", delay_var);

        BLINK_MS.store(delay_var, Ordering::Relaxed);

        match writeln!(&mut msg, "{:03}", value) {
            Ok(_) => match usart.blocking_write(msg.as_bytes()) {
                Ok(_) => (),
                Err(e) => error!("unable to transfer message over uart {}", e),
            },
            Err(_e) => {
                error!("unable to format msg");
            }
        }

        value = value.wrapping_add(1);

        msg.clear();
    }
}

#[embassy_executor::task]
async fn led_task(led: AnyPin) {
    let mut led = Output::new(led, Level::Low, Speed::Low);

    loop {
        let delay = BLINK_MS.load(Ordering::Relaxed);
        Timer::after(Duration::from_millis(delay.into())).await;
        led.toggle();
    }
}
