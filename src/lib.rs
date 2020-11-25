//! This is a platform agnostic Rust driver for the iAQ-Core sensors
//! to measure VOC levels and provide CO2 equivalent and TVOC
//! equivalent predictions using the [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
//!
//! This driver allows you to:
//! - Read all the sensor data at once. See: [`data()`].
//! - Read the CO2 equivalent prediction value in ppm. See: [`co2()`].
//! - Read the TVOC equivalent prediction value in ppb. See: [`tvoc()`].
//! - Read the sensor resistance in Ohm. See: [`resistance()`].
//!
//! [`data()`]: struct.IaqCore.html#method.data
//! [`co2()`]: struct.IaqCore.html#method.co2
//! [`tvoc()`]: struct.IaqCore.html#method.tvoc
//! [`resistance()`]: struct.IaqCore.html#method.resistance
//!
//! <!-- TODO
//! [Introductory blog post](TODO)
//! -->
//!
//! ## The devices
//!
//! The ams iAQ-core Indoor Air Quality Module is a low-cost, ultracompact
//! solution for detecting poor air quality. It is equipped with an MOS sensor
//! element for the detection of a broad range of reducing gases such as CO
//! and VOCs. A change of resistance in the presence of these gases generates
//! a signal that is translated into parts per million (ppm) CO2 equivalent or
//! parts per billion (ppb) TVOC equivalent units.
//!
//! When defined threshold limits are exceeded, the module signals the system
//! to initiate activities such as increasing ventilation, releasing fragrance,
//! providing a message to open a window, switching on an air cleaner, etc.
//! When VOC levels are minimized, the module instructs the system to return
//! to standby, thereby saving energy, lowering operating costs and maintaining
//! a healthy environment.
//!
//! In any demand-controlled ventilation/actuation environment where air
//! quality is important, including commercial and residential facilities
//! (offices, classrooms, kitchens, bathrooms, living and bedrooms etc.)
//! the iAQ-core Indoor Air Quality Module performs accurately and reliably.
//! Plus, the module’s small size opens up a wide variety of new applications
//! where space is at a premium.
//!
//! Documentation:
//! - [Datasheet](https://ams.com/documents/20143/36005/iAQ-core_DS000334_1-00.pdf)
//! - [Factsheet](https://ams.com/documents/20143/36005/iAQ-core_FS000136_1-00.pdf)
//!
//! ## Usage examples (see also examples folder)
//!
//! To use this driver, import this crate and an `embedded_hal` implementation,
//! then create an instance of the driver.
//!
//! Please find additional examples using hardware in this repository: [driver-examples]
//!
//! [driver-examples]: https://github.com/eldruin/driver-examples
//!
//! ### Create an instance of the driver and print the measurements
//!
//! ```no_run
//! use iaq_core::IaqCore;
//! use linux_embedded_hal::I2cdev;
//! use nb::block;
//!
//! # fn main() {
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor = IaqCore::new(dev);
//! loop {
//!     let data = block!(sensor.data()).unwrap();
//!     println!(
//!         "CO2: {} ppm, TVOC: {} ppm, Resistance: {} Ohm",
//!         data.co2, data.tvoc, data.resistance
//!     );
//! }
//! # }
//! ```
//!
//! ### Get the CO2 and TVOC prediction values independently
//!
//! In case you are only interested in one of them, for example.
//!
//! ```no_run
//! use iaq_core::IaqCore;
//! use linux_embedded_hal::I2cdev;
//! use nb::block;
//!
//! # fn main() {
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor = IaqCore::new(dev);
//! loop {
//!     let co2 = block!(sensor.co2()).unwrap();
//!     let tvoc = block!(sensor.tvoc()).unwrap();
//!     println!("CO2: {} ppm, TVOC: {} ppb", co2, tvoc);
//! }
//! # }
//! ```
//!
//! ### Get only the raw sensor resistance value
//!
//! ```no_run
//! use iaq_core::IaqCore;
//! use linux_embedded_hal::I2cdev;
//! use nb::block;
//!
//! # fn main() {
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor = IaqCore::new(dev);
//! loop {
//!     let r = block!(sensor.resistance()).unwrap();
//!     println!("Resistance: {} Ohm", r);
//! }
//! # }
//! ```

#![deny(unsafe_code, missing_docs)]
#![no_std]

mod device_impl;
mod types;
pub use crate::types::{Error, Measurement};

/// iAQ-Core device driver
#[derive(Debug)]
pub struct IaqCore<I2C> {
    /// The concrete I²C device implementation.
    i2c: I2C,
}
