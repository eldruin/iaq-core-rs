use hal::i2c::{Mock as I2cMock, Transaction as I2cTrans};
use iaq_core::IaqCore;

pub fn new(transactions: &[I2cTrans]) -> IaqCore<I2cMock> {
    IaqCore::new(I2cMock::new(transactions))
}

pub fn destroy(sensor: IaqCore<I2cMock>) {
    sensor.destroy().done();
}
