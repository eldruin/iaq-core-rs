extern crate embedded_hal_mock as hal;
extern crate iaq_core;

mod common;
use common::{destroy, new};

#[test]
fn can_create_and_destroy() {
    let sensor = new(&[]);
    destroy(sensor);
}
