use firmware::ButtonReport;
use std::{
    io,
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
};
use tokio::net::UdpSocket;

use crate::BUTTONS;

const REMOTE: SocketAddr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(192, 168, 42, 1), 5001));

#[derive(Debug)]
pub struct Connection {
    socket: UdpSocket,
}

impl Connection {
    pub async fn new() -> io::Result<Self> {
        let socket = UdpSocket::bind("0.0.0.0:0").await?;
        socket.connect(REMOTE).await?;
        let buf = "Yes, my lord.".as_bytes();
        socket.send(buf).await?;
        Ok(Self { socket })
    }

    pub async fn recv(&self) -> io::Result<()> {
        let mut buf: [u8; 1024] = [0; 1024];
        loop {
            self.socket.recv(&mut buf).await?;
            let report: ButtonReport = match bincode::deserialize(&buf) {
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
