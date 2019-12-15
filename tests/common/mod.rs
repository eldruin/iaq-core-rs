use embedded_hal_mock::i2c::{Mock as I2cMock, Transaction as I2cTrans};
use iaq_core::IaqCore;

pub const DEV_ADDR: u8 = 0x5A;

pub fn new(transactions: &[I2cTrans]) -> IaqCore<I2cMock> {
    IaqCore::new(I2cMock::new(transactions))
}

pub fn destroy(sensor: IaqCore<I2cMock>) {
    sensor.destroy().done();
}

#[macro_export]
macro_rules! assert_error {
    ($result:expr, Error::$variant:ident) => {
        match $result {
            Err(nb::Error::Other(Error::$variant)) => (),
            _ => panic!("Error not returned."),
        }
    };
    ($result:expr, nb::Error::WouldBlock) => {
        match $result {
            Err(nb::Error::WouldBlock) => (),
            _ => panic!("Error not returned."),
        }
    };
}
