use iaq_core::IaqCore;
use linux_embedded_hal::I2cdev;
use nb::block;

fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let mut sensor = IaqCore::new(dev);
    loop {
        let data = block!(sensor.data()).unwrap();
        println!(
            "CO2: {} ppm, TVOC: {} ppb, Resistance: {} Ohm",
            data.co2, data.tvoc, data.resistance
        );
    }
}
