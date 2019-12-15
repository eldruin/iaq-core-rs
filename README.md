# Rust iAQ-Core Indoor Air Quality Sensor Driver

<!-- TODO
[![crates.io](https://img.shields.io/crates/v/iaq-core.svg)](https://crates.io/crates/iaq-core)
[![Docs](https://docs.rs/iaq-core/badge.svg)](https://docs.rs/iaq-core)
-->
[![Build Status](https://travis-ci.com/eldruin/iaq-core-rs.svg?branch=master)](https://travis-ci.com/eldruin/iaq-core-rs)
[![Coverage Status](https://coveralls.io/repos/github/eldruin/iaq-core-rs/badge.svg?branch=master)](https://coveralls.io/github/eldruin/iaq-core-rs?branch=master)

This is a platform agnostic Rust driver for the iAQ-Core indoor air quality sensors
using the [`embedded-hal`] traits.

This driver allows you to:
- Read all the sensor data at once. See: `data()`.
- Read the CO2 equivalent prediction value in ppm. See: `co2()`.
- Read the TVOC equivalent prediction value in ppb. See: `tvoc()`.
- Read the sensor resistance in Ohm. See: `resistance()`.

<!-- TODO
[Introductory blog post]()
-->

The ams iAQ-core Indoor Air Quality Module is a low-cost, ultracompact
solution for detecting poor air quality. It is equipped with an MOS sensor
element for the detection of a broad range of reducing gases such as CO
and VOCs. A change of resistance in the presence of these gases generates
a signal that is translated into parts per million (ppm) CO2 equivalent or
parts per billion (ppb) TVOC equivalent units.

When defined threshold limits are exceeded, the module signals the system
to initiate activities such as increasing ventilation, releasing fragrance,
providing a message to open a window, switching on an air cleaner, etc.
When VOC levels are minimized, the module instructs the system to return
to standby, thereby saving energy, lowering operating costs and maintaining
a healthy environment.

In any demand-controlled ventilation/actuation environment where air
quality is important, including commercial and residential facilities
(offices, classrooms, kitchens, bathrooms, living and bedrooms etc.)
the iAQ-core Indoor Air Quality Module performs accurately and reliably.
Plus, the module’s small size opens up a wide variety of new applications
where space is at a premium.

Documentation:
- [Datasheet](TODO)
- [Programming and interfacing guide](TODO)

## Usage

To use this driver, import this crate and an `embedded_hal` implementation,
then instantiate the device.

Please find additional examples using hardware in this repository: [driver-examples]

[driver-examples]: https://github.com/eldruin/driver-examples

```rust
use iaq_core::IaqCore;
use linux_embedded_hal::I2cdev;
use nb::block;

fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let mut sensor = IaqCore::new(dev);
    loop {
        let data = block!(sensor.data()).unwrap();
        println!("CO2: {} ppm, TVOC: {} ppb", data.co2, data.tvoc);
    }
}
```

## Support

For questions, issues, feature requests, and other changes, please file an
[issue in the github project](https://github.com/eldruin/iaq-core-rs/issues).

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

[`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
