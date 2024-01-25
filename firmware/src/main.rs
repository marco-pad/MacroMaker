use sysfs_gpio::{Direction, Error, Pin};

fn main() -> sysfs_gpio::Result<()> {
    let input = Pin::new(508); // GP0
    input.export()?;
    input.set_direction(Direction::In)?;
    let mut button = Button::new(input)?;

    loop {
        let result = button.poll()?;
        println!("{result:?}");
    }
    // my_led.unexport()?;
}

struct Button {
    pin: Pin,
    previous_value: u8,
}

impl Button {
    pub fn new(pin: Pin) -> Result<Self, Error> {
        Ok(Self {
            pin,
            previous_value: pin.get_value()?,
        })
    }

    /// Polls the button until it has changed.
    pub fn poll(&mut self) -> Result<State, Error> {
        loop {
            let value = self.pin.get_value()?;
            if value != self.previous_value {
                self.previous_value = value;
                return Ok(match value {
                    0 => State::Pressed,
                    1 => State::Released,
                    _ => unreachable!(),
                });
            }
        }
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
enum State {
    Pressed,
    Released,
}
