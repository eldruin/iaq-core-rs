/// All possible errors in this crate
#[derive(Debug)]
pub enum Error<E> {
    /// I²C bus error
    I2C(E),
    /// Device reports an erroneous status. If this happens constantly
    /// (or very frequently) this indicates that the module is reading
    /// non-realistic values, and the sensor element is probably defective.
    Device,
}

/// Sensor measurement data
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Measurement {
    /// CO2 (ppm) equivalent prediction value
    pub co2: u16,
    /// TVOC (ppb) equivalent prediction value
    pub tvoc: u16,
    /// Sensor resistance in Ohm
    pub resistance: u32,
}
