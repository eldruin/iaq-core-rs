use crate::IaqCore;

impl<I2C> IaqCore<I2C> {
    /// Create new instance of the iAQ-Core device.
    pub fn new(i2c: I2C) -> Self {
        IaqCore { i2c }
    }

    /// Destroy driver instance, return I²C bus instance.
    pub fn destroy(self) -> I2C {
        self.i2c
    }
}
