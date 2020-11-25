use crate::{hal::blocking::i2c::Read, Error, IaqCore, Measurement};
use nb;

const DEV_ADDR: u8 = 0x5A;

impl<E, I2C> IaqCore<I2C>
where
    I2C: Read<Error = E>,
{
    /// Create new instance of the iAQ-Core device.
    pub fn new(i2c: I2C) -> Self {
        IaqCore { i2c }
    }

    /// Destroy driver instance, return I²C bus instance.
    pub fn destroy(self) -> I2C {
        self.i2c
    }

    /// Get all data from the sensor measurement
    ///
    /// Returns `nb::Error::WouldBlock` in case the device reports a busy or warm up status.
    pub fn data(&mut self) -> nb::Result<Measurement, Error<E>> {
        let mut data = [0; 9];
        self.read(&mut data)?;
        Ok(Measurement {
            co2: (u16::from(data[0]) << 8) | u16::from(data[1]),
            tvoc: (u16::from(data[7]) << 8) | u16::from(data[8]),
            resistance: (u32::from(data[4]) << 16) | (u32::from(data[5]) << 8) | u32::from(data[6]),
        })
    }

    /// Get the CO2 (ppm) equivalent prediction value
    ///
    /// Returns `nb::Error::WouldBlock` in case the device reports a busy or warm up status.
    pub fn co2(&mut self) -> nb::Result<u16, Error<E>> {
        let mut data = [0; 3];
        self.read(&mut data)?;
        Ok((u16::from(data[0]) << 8) | u16::from(data[1]))
    }

    /// Get the TVOC (ppb) equivalent prediction value
    ///
    /// Returns `nb::Error::WouldBlock` in case the device reports a busy or warm up status.
    pub fn tvoc(&mut self) -> nb::Result<u16, Error<E>> {
        let mut data = [0; 9];
        self.read(&mut data)?;
        Ok((u16::from(data[7]) << 8) | u16::from(data[8]))
    }

    /// Get the sensor resistance in Ohm
    ///
    /// Returns `nb::Error::WouldBlock` in case the device reports a busy or warm up status.
    pub fn resistance(&mut self) -> nb::Result<u32, Error<E>> {
        let mut data = [0; 7];
        self.read(&mut data)?;
        Ok((u32::from(data[4]) << 16) | (u32::from(data[5]) << 8) | u32::from(data[6]))
    }

    fn read(&mut self, data: &mut [u8]) -> nb::Result<(), Error<E>> {
        self.i2c.read(DEV_ADDR, data).map_err(Error::I2C).map_err(nb::Error::Other)?;
        Self::check_status(data[2])
    }

    fn check_status(status: u8) -> nb::Result<(), Error<E>> {
        if status == 0x80 {
            Err(nb::Error::Other(Error::Device))
        } else if status == 0 {
            Ok(())
        } else {
            // warm up or busy
            Err(nb::Error::WouldBlock)
        }
    }
}
