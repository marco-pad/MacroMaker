use std::thread::sleep;
use std::time::Duration;
use sysfs_gpio::{Direction, Pin, Result};

fn main() -> Result<()> {
    let my_led = Pin::new(508); // GP0
    my_led.export()?;
    my_led.set_direction(Direction::Out)?;
    loop {
        my_led.set_value(0).unwrap();
        sleep(Duration::from_millis(200));
        my_led.set_value(1).unwrap();
        sleep(Duration::from_millis(200));
        println!("On");
    }
    // my_led.unexport()?;
}
