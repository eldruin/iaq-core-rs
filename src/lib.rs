//! This is a platform agnostic Rust driver for the iAQ-Core sesor
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
//! - [Datasheet](TODO)
//! - [Programming and interfacing guide](TODO)
//!

#![deny(unsafe_code, missing_docs)]
#![no_std]

use embedded_hal as hal;

mod device_impl;
mod types;
pub use crate::types::{Error, Measurement};

/// iAQ-Core device driver
#[derive(Debug)]
pub struct IaqCore<I2C> {
    /// The concrete I²C device implementation.
    i2c: I2C,
}
