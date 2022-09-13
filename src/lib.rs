//! Minecraft query library
pub mod packet;

use crate::packet::{Information, Marshaller, Request, Response};
use anyhow::Result;
use rand::prelude::*;
use std::io::Cursor;
use std::net::{ToSocketAddrs, UdpSocket};
use std::time::Duration;

/// Executes a query to the given host and port.
pub fn handle(address: impl ToSocketAddrs, timeout: Option<Duration>) -> Result<Information> {
    let mut rng = thread_rng();

    let socket = UdpSocket::bind("0.0.0.0:34254")?;
    let timeout = timeout.or(Some(Duration::from_secs(5)));
    socket.set_read_timeout(timeout)?;
    socket.set_write_timeout(timeout)?;

    let mut writer = Vec::new();
    let request = Request::new(packet::QUERY_TYPE_HANDSHAKE, rng.gen(), 0);
    request.marshall(&mut writer)?;
    socket.send_to(writer.as_slice(), &address)?;

    let mut data = [0; u16::MAX as usize];
    let (size, _) = socket.recv_from(&mut data)?;
    let mut response = Response::default();
    let mut reader = Cursor::new(data[..size].to_vec());
    response.unmarshall(&mut reader)?;

    let mut writer = Vec::new();
    Request::new(
        packet::QUERY_TYPE_INFORMATION,
        rng.gen(),
        response.response_number,
    )
    .marshall(&mut writer)?;
    socket.send_to(writer.as_slice(), &address)?;
    let (n, _) = socket.recv_from(&mut data)?;

    let mut resp = Response::default();
    let mut reader = Cursor::new(data[..n].to_vec());
    resp.unmarshall(&mut reader)?;

    Ok(resp.information)
}
