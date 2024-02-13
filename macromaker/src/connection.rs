use iced::{futures::SinkExt, Subscription};
use std::{
    io,
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
    time::{Duration, SystemTime},
};
use tokio::net::UdpSocket;

use crate::{updates::Message, BUTTONS, CONNECTION};

const REMOTE: SocketAddr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(192, 168, 42, 1), 5001));

#[derive(Debug)]
pub struct Connection {
    socket: UdpSocket,
    last_request: SystemTime,
    pub ping: Duration,
}

impl Connection {
    pub async fn new() -> io::Result<Self> {
        let socket = UdpSocket::bind("0.0.0.0:0").await?;
        socket.connect(REMOTE).await?;
        let buf = "Yes, my lord.".as_bytes();
        socket.send(buf).await?;
        Ok(Self {
            socket,
            last_request: SystemTime::now(),
            ping: Duration::default(),
        })
    }

    pub async fn recv(&mut self) -> io::Result<firmware::Message> {
        let mut buf: [u8; 1024] = [0; 1024];
        self.socket.recv(&mut buf).await?;
        let message: firmware::Message = match bincode::deserialize(&buf) {
            Ok(t) => t,
            Err(_) => {
                return Ok(firmware::Message::Ping);
            }
        };
        match message {
            firmware::Message::Ping => {
                self.ping = self.last_request.elapsed().unwrap();
            }
            firmware::Message::ButtonReport(report) => {
                if (1..=9).contains(&report.id) {
                    let button = &mut BUTTONS.lock()[report.id - 1];
                    button.state = report.state;
                    button.action.perform(report.state);
                }
            }
        }
        Ok(message)
    }
    pub fn ping(&mut self) -> io::Result<usize> {
        self.last_request = SystemTime::now();
        self.socket
            .try_send(&bincode::serialize(&firmware::Message::Ping).unwrap())
    }
}

pub fn ping_loop() {
    std::thread::spawn(|| loop {
        unsafe {
            if let Some(connection) = CONNECTION.as_mut() {
                connection.ping().unwrap_or_else(|_| {
                    CONNECTION = None;
                    0
                });
            }
            std::thread::sleep(Duration::from_millis(100));
        }
    });
}

pub fn subscribe() -> Subscription<Message> {
    iced::subscription::channel(0, 64, |mut output| async move {
        loop {
            unsafe {
                if let Some(connection) = CONNECTION.as_mut() {
                    let message;
                    let result = connection.recv().await;
                    if let Ok(firmware_message) = result {
                        if let firmware::Message::Ping = firmware_message {
                            message = Message::Nothing;
                        } else {
                            message = Message::Input;
                        }
                    } else {
                        CONNECTION = None;
                        message = Message::Nothing;
                    }

                    output.send(message).await.unwrap();
                } else {
                    std::thread::sleep(Duration::from_millis(100));
                    CONNECTION = Connection::new().await.ok();
                }
            }
        }
    })
}
