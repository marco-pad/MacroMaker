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
    let input = Pin::new(508); // pin 1
    input.export()?;
    input.set_direction(Direction::In)?;
    let mut b1 = Button::new(input, 1)?;
    let input = Pin::new(509); // pin 2
    input.export()?;
    input.set_direction(Direction::In)?;
    let mut b2 = Button::new(input, 1)?;
    let input = Pin::new(378); // pin 4
    input.export()?;
    input.set_direction(Direction::In)?;
    let mut b3 = Button::new(input, 1)?;
    let input = Pin::new(375); // pin 9
    input.export()?;
    input.set_direction(Direction::In)?;
    let mut b4 = Button::new(input, 1)?;
    let input = Pin::new(374); // pin 10
    input.export()?;
    input.set_direction(Direction::In)?;
    let mut b5 = Button::new(input, 1)?;
    let input = Pin::new(373); // pin 11
    input.export()?;
    input.set_direction(Direction::In)?;
    let mut b6 = Button::new(input, 1)?;
    let input = Pin::new(370); // pin 12
    input.export()?;
    input.set_direction(Direction::In)?;
    let mut b7 = Button::new(input, 1)?;
    let input = Pin::new(425); // pin 14
    input.export()?;
    input.set_direction(Direction::In)?;
    let mut b8 = Button::new(input, 1)?;
    let input = Pin::new(426); // pin 15
    input.export()?;
    input.set_direction(Direction::In)?;
    let mut b9 = Button::new(input, 1)?;

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
        let report = loop {
            if let Some(report) = b1.poll()? {
                break report;
            };
            if let Some(report) = b2.poll()? {
                break report;
            };
            if let Some(report) = b3.poll()? {
                break report;
            };
            if let Some(report) = b4.poll()? {
                break report;
            };
            if let Some(report) = b5.poll()? {
                break report;
            };
            if let Some(report) = b6.poll()? {
                break report;
            };
            if let Some(report) = b7.poll()? {
                break report;
            };
            if let Some(report) = b8.poll()? {
                break report;
            };
            if let Some(report) = b9.poll()? {
                break report;
            };
            std::thread::sleep(Duration::from_micros(1));
        };
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
    pub fn poll(&mut self) -> Result<Option<ButtonReport>, Error> {
        let value = self.pin.get_value()?;
        if value != self.previous_value {
            self.previous_value = value;
            Ok(Some(ButtonReport {
                id: self.id,
                state: match value {
                    0 => State::Pressed,
                    1 => State::Released,
                    _ => unreachable!(),
                },
            }))
        } else {
            Ok(None)
        }
    }
}
