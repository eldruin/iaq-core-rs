use iaq_core::{IaqCore, Measurement};
use linux_embedded_hal::I2cdev;
use nb::block;

fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let mut sensor = IaqCore::new(dev);
    loop {
        let data = block!(sensor.data()).unwrap();
        println!("CO2: {}, TVOC: {}", data.co2, data.tvoc);
    }
}
