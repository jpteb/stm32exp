#![no_std]
#![no_main]
#![feature(impl_trait_in_assoc_type)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::{
    bind_interrupts,
    dma::NoDma,
    gpio::{AnyPin, Level, Output, Pin, Speed},
    i2c::{self, I2c},
    peripherals,
    time::Hertz,
    usart::{Config as UsartConfig, UartTx},
};
use embassy_time::{Duration, Timer};
use heapless::String;
use {defmt_rtt as _, panic_probe as _};

const ADDRESS: u8 = 0b1101000;
const POWERUP: u8 = 0b00000000;
const WHOAMI: u8 = 0x75;
const PWR_MGMT_1: u8 = 0x6B;
const TEMP_OUT_H: u8 = 0x41;
const TEMP_OUT_L: u8 = 0x42;
const GYRO_XOUT_H: u8 = 0x43;

static BLINK_MS: u32 = 500;
bind_interrupts!(struct Irqs {
    I2C1_EV => i2c::EventInterruptHandler<peripherals::I2C1>;
    I2C1_ER => i2c::ErrorInterruptHandler<peripherals::I2C1>;
});

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Hello World!");

    spawner.spawn(led_task(p.PA5.degrade())).unwrap();

    let mut usart = UartTx::new(p.USART2, p.PA2, NoDma, UsartConfig::default()).unwrap();
    let mut msg: String<128> = String::new();

    // I2C Configuration
    let mut i2c = I2c::new(
        p.I2C1,
        p.PB8,
        p.PB9,
        Irqs,
        NoDma,
        NoDma,
        Hertz(400_000),
        Default::default(),
    );

    let mut data = [0u8; 1];
    match i2c.blocking_write_read(ADDRESS, &[WHOAMI], &mut data) {
        Ok(()) => {
            info!("WHOAMI: {:#01x}", data[0]);
            if data[0] != 0x68 {
                error!("unexpected whoami result!");
                None.unwrap()
            }
        }
        Err(i2c::Error::Timeout) => error!("i2c whoami call timed out!"),
        Err(e) => error!("i2c error: {:?}", e),
    }
    match i2c.blocking_write(ADDRESS, &[PWR_MGMT_1, POWERUP]) {
        Ok(()) => info!("Successfully powered up the chip",),
        Err(e) => error!("failed to power up the chip: {:?}", e),
    }

    let mut temp_data = [0u8; 1];

    loop {
        match i2c.blocking_write_read(ADDRESS, &[TEMP_OUT_H], &mut temp_data) {
            Ok(()) => {
                info!("Successfully retrieved data {}", temp_data[0]);
                let mut temperature = (temp_data[0] as i16) << 8;
                i2c.blocking_write_read(ADDRESS, &[TEMP_OUT_L], &mut temp_data)
                    .unwrap();
                info!("Successfully retrieved data {}", temp_data[0]);
                temperature |= temp_data[0] as i16;
                info!("raw temperature: {}", temperature);
                let real_temperature = (temperature as f32 / 340.0) + 36.53;
                info!("received temperature: {}", real_temperature);
            }
            Err(e) => error!("failed to retrieve temperature data: {:?}", e),
        }

        Timer::after(Duration::from_millis(BLINK_MS.into())).await;
    }
}

#[embassy_executor::task]
async fn led_task(led: AnyPin) {
    let mut led = Output::new(led, Level::Low, Speed::Low);

    loop {
        Timer::after(Duration::from_millis(BLINK_MS.into())).await;
        led.toggle();
    }
}
