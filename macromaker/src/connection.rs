use firmware::ButtonReport;
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

    pub async fn recv(&self) -> io::Result<()> {
        let mut buf: [u8; 1024] = [0; 1024];
        loop {
            self.socket.recv(&mut buf).await?;
            let message: firmware::Message = match bincode::deserialize(&buf) {
                Ok(t) => t,
                Err(_) => {
                    continue;
                }
            };
            if !(1..=9).contains(&report.id) {
                continue;
            }
            let button = &mut BUTTONS.lock()[report.id - 1];
            button.state = report.state;
            button.action.perform(report.state);
        }
    }
}

pub fn subscribe() -> Subscription<Message> {
    iced::subscription::channel(0, 64, |mut output| async move {
        loop {
            unsafe {
                if let Some(connection) = CONNECTION.as_mut() {
                    connection
                        .recv()
                        .await
                        .unwrap_or_else(|_| CONNECTION = None);
                    output.send(Message::Input).await.unwrap();
                } else {
                    std::thread::sleep(Duration::from_millis(100));
                    CONNECTION = Connection::new().await.ok();
                }
            }
        }
    })
}
