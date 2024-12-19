// use core::result::{Result, Result::Ok};
use embassy_stm32::i2c::{I2c, Instance};

use crate::Error;

pub type I2cAddress = u8;

pub struct Mpu6050<'d, P: Instance> {
    i2c: I2c<'d, P>,
    address: I2cAddress,
}

// impl<'d, I: Instance> Mpu6050<'d, I> {
//     pub fn new(i2c: I2c<'d, I>, address: I2cAddress) -> Result<Self, Error> {
//         Ok(Self { i2c, address })
//     }
// }
