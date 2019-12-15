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
