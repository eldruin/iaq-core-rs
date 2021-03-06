use iaq_core::{Error, Measurement};
mod common;
use crate::common::{destroy, new, DEV_ADDR};
use embedded_hal_mock::i2c::Transaction as I2cTrans;

#[test]
fn can_create_and_destroy() {
    let sensor = new(&[]);
    destroy(sensor);
}

#[test]
fn if_busy_returns_would_block() {
    let mut sensor = new(&[I2cTrans::read(DEV_ADDR, vec![0, 0, 0x01])]);
    assert_error!(sensor.co2(), nb::Error::WouldBlock);
    destroy(sensor);
}

#[test]
fn if_warm_up_returns_would_block() {
    let mut sensor = new(&[I2cTrans::read(DEV_ADDR, vec![0, 0, 0x10])]);
    assert_error!(sensor.co2(), nb::Error::WouldBlock);
    destroy(sensor);
}

#[test]
fn can_return_device_error() {
    let mut sensor = new(&[I2cTrans::read(DEV_ADDR, vec![0, 0, 0x80])]);
    assert_error!(sensor.co2(), Error::Device);
    destroy(sensor);
}

#[test]
fn can_get_co2() {
    let mut sensor = new(&[I2cTrans::read(DEV_ADDR, vec![0x12, 0x34, 0])]);
    assert_eq!(0x1234, sensor.co2().unwrap());
    destroy(sensor);
}

#[test]
fn can_get_tvoc() {
    let mut sensor = new(&[I2cTrans::read(
        DEV_ADDR,
        vec![0, 0, 0, 0, 0, 0, 0, 0x12, 0x34],
    )]);
    assert_eq!(0x1234, sensor.tvoc().unwrap());
    destroy(sensor);
}

#[test]
fn can_get_resistance() {
    let mut sensor = new(&[I2cTrans::read(DEV_ADDR, vec![0, 0, 0, 0, 0x12, 0x34, 0x56])]);
    assert_eq!(0x123456, sensor.resistance().unwrap());
    destroy(sensor);
}

#[test]
fn can_get_measurement() {
    let mut sensor = new(&[I2cTrans::read(
        DEV_ADDR,
        vec![0x1A, 0x1B, 0, 0, 0x2A, 0x2B, 0x2C, 0x3A, 0x3B],
    )]);
    assert_eq!(
        Measurement {
            co2: 0x1A1B,
            tvoc: 0x3A3B,
            resistance: 0x2A2B2C,
        },
        sensor.data().unwrap()
    );
    destroy(sensor);
}
