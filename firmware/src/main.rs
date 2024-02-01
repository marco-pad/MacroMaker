use firmware::*;
use std::{
    net::{SocketAddr, UdpSocket},
    sync::Arc,
    thread,
    time::Duration,
};
use sysfs_gpio::{Direction, Error, Pin};

use crossbeam::atomic::AtomicCell;

fn main() -> sysfs_gpio::Result<()> {
    let input = Pin::new(508); // GP0
    input.export()?;
    input.set_direction(Direction::In)?;
    let mut button = Button::new(input, 1)?;

    let socket = UdpSocket::bind("0.0.0.0:5001").unwrap();

    let mut buf = [0; 1024];
    let (len, addr) = socket.recv_from(&mut buf).unwrap();
    let address = Arc::new(AtomicCell::new(addr));

    println!(
        "{addr:?} connected: {}",
        String::from_utf8_lossy(&buf[..len])
    );

    update_ip(socket.try_clone().unwrap(), address.clone());

    loop {
        let report = button.poll()?;
        let buf = bincode::serialize(&firmware::Message::ButtonReport(report)).unwrap();
        socket.send_to(&buf, address.load()).unwrap();
    }
}

fn update_ip(socket: UdpSocket, address: Arc<AtomicCell<SocketAddr>>) {
    thread::spawn(move || {
        let address = address.clone();
        let message = bincode::serialize(&firmware::Message::Ping).unwrap();
        let mut buf = [0; 1024];
        let mut addr = address.load();
        loop {
            let (len, addr2) = socket.recv_from(&mut buf).unwrap();
            socket.send_to(&message, addr).unwrap();
            if addr != addr2 {
                addr = addr2;
                println!("IP changed: {}", String::from_utf8_lossy(&buf[..len]));
                address.store(addr);
            }
        }
    });
}

struct Button {
    pin: Pin,
    id: usize,
    previous_value: u8,
}

impl Button {
    pub fn new(pin: Pin, id: usize) -> Result<Self, Error> {
        Ok(Self {
            pin,
            id,
            previous_value: pin.get_value()?,
        })
    }

    /// Polls the button until it has changed.
    /// 1000hz
    pub fn poll(&mut self) -> Result<ButtonReport, Error> {
        loop {
            let value = self.pin.get_value()?;
            if value != self.previous_value {
                self.previous_value = value;
                return Ok(ButtonReport {
                    id: self.id,
                    state: match value {
                        0 => State::Pressed,
                        1 => State::Released,
                        _ => unreachable!(),
                    },
                });
            }
            std::thread::sleep(Duration::from_millis(1));
        }
    }
}
