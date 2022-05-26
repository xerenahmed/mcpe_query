use anyhow::{anyhow, Result};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::collections::HashMap;
use std::io::{BufReader, BufWriter, Cursor, Read, Write};

const VERSION: [u8; 2] = [0xfe, 0xfd];

const PADDING: [u8; 4] = [0xff, 0xff, 0xff, 0x01];

const SPLIT_NUM: [u8; 9] = [b'S', b'P', b'L', b'I', b'T', b'N', b'U', b'M', 0x00];

const PLAYER_KEY: [u8; 11] = [
    0x00, 0x01, b'p', b'l', b'a', b'y', b'e', b'r', b'_', 0x00, 0x00,
];

pub const QUERY_TYPE_HANDSHAKE: u8 = 0x09;
pub const QUERY_TYPE_INFORMATION: u8 = 0x00;

/// Request to query the server
#[derive(Debug, Clone, Copy)]
pub struct Request {
    pub request_type: u8,
    pub sequence_number: i32,
    pub response_number: i32,
}

/// Response from the server
#[derive(Debug, Clone, Default)]
pub struct Response {
    pub response_type: u8,
    pub sequence_number: i32,
    pub response_number: i32,
    pub information: Information,
}

/// Information that is received in response
#[derive(Debug, Clone, Default)]
pub struct Information {
    /// Online players
    pub players: Vec<String>,
    /// Other information like server name, motd, etc.
    pub other: HashMap<String, String>,
}

impl Request {
    pub fn new(request_type: u8, sequence_number: i32, response_number: i32) -> Request {
        Request {
            request_type,
            sequence_number,
            response_number,
        }
    }
}

pub trait Marshaller {
    fn marshall(&self, writer: &mut BufWriter<Vec<u8>>) -> Result<()>;
    fn unmarshall(&mut self, reader: &mut BufReader<Cursor<Vec<u8>>>) -> Result<()>;
}

impl Marshaller for Request {
    fn marshall(&self, writer: &mut BufWriter<Vec<u8>>) -> Result<()> {
        writer.write_all(&VERSION)?;

        let mut wtr = vec![];
        wtr.write_u8(self.request_type)?;
        wtr.write_i32::<BigEndian>(self.sequence_number)?;
        writer.write_all(&wtr)?;

        if self.request_type == QUERY_TYPE_INFORMATION {
            let mut wtr = vec![];
            wtr.write_i32::<BigEndian>(self.response_number)?;
            writer.write_all(&wtr)?;
            writer.write_all(&PADDING)?;
        }

        Ok(())
    }

    fn unmarshall(&mut self, reader: &mut BufReader<Cursor<Vec<u8>>>) -> Result<()> {
        let mut v: [u8; 2] = [0x0, 0x0];
        reader.read_exact(&mut v)?;

        if v != VERSION {
            return Err(anyhow!("Invalid version"));
        }

        self.request_type = reader.read_u8()?;
        self.sequence_number = reader.read_i32::<BigEndian>()?;

        if self.request_type == QUERY_TYPE_INFORMATION {
            self.response_number = reader.read_i32::<BigEndian>()?;

            let mut padding = [0u8; 4];
            reader.read_exact(&mut padding)?;
        } else if self.request_type != QUERY_TYPE_HANDSHAKE {
            return Err(anyhow!("unknown request type"));
        }
        Ok(())
    }
}

impl Marshaller for Response {
    fn marshall(&self, writer: &mut BufWriter<Vec<u8>>) -> Result<()> {
        let mut wtr = vec![];
        wtr.write_u8(self.response_type)?;
        wtr.write_i32::<BigEndian>(self.sequence_number)?;
        writer.write_all(&wtr)?;

        if self.response_type == QUERY_TYPE_HANDSHAKE {
            let mut v = self.response_number.to_string().into_bytes();
            if v.len() != 12 {
                v.resize(12, 0);
            }
            writer.write_all(&v)?;
        } else {
            writer.write_all(&SPLIT_NUM)?;
            let mut wtr = vec![];
            wtr.write_u8(0x80)?;
            wtr.write_u8(0)?;
            writer.write_all(&wtr)?;

            let mut values: Vec<Vec<u8>> = Vec::new();
            for (key, value) in self.information.other.clone().into_iter() {
                values.push(key.into_bytes());
                values.push(value.into_bytes());
            }
            let values_bytes = values.join(&0);
            writer.write_all(values_bytes.as_slice())?;
        }

        Ok(())
    }

    fn unmarshall(&mut self, reader: &mut BufReader<Cursor<Vec<u8>>>) -> Result<()> {
        self.response_type = reader.read_u8()?;
        self.sequence_number = reader.read_i32::<BigEndian>()?;

        match self.response_type {
            QUERY_TYPE_HANDSHAKE => {
                let mut num_bytes = [0u8; 12];
                let n = reader.read(&mut num_bytes)?;
                let mut num_bytes = num_bytes[..n].to_vec();

                if let Some(index) = num_bytes.iter().position(|&x| x == 0x00) {
                    num_bytes.truncate(index);
                }
                self.response_number = String::from_utf8(num_bytes.to_vec())?.parse::<i32>()?;
                Ok(())
            }
            QUERY_TYPE_INFORMATION => {
                let mut v = [0u8; 11];
                reader.read_exact(&mut v)?;

                let mut information: Vec<u8> = Vec::new();
                reader.read_to_end(&mut information)?;

                let player_index = information
                    .windows(PLAYER_KEY.len())
                    .position(|window| window == PLAYER_KEY);
                let data = information.clone();
                if let Some(player_index) = player_index {
                    information.truncate(player_index);
                }

                let mut values: Vec<&[u8]> = information.split(|num| num == &0x00).collect();
                self.information.other = HashMap::with_capacity(values.len() / 2);
                if values.len() % 2 != 0 {
                    values.truncate(values.len() - 1);
                }
                for i in 0..values.len() {
                    if i % 2 != 0 {
                        continue;
                    }
                    let current = values[i].to_vec();
                    let next = values[i + 1];
                    let current_str = String::from_utf8(current)?;
                    let next_str = String::from_utf8_lossy(next).to_string();
                    self.information.other.insert(current_str, next_str);
                }
                if let Some(player_index) = player_index {
                    let index = player_index + PLAYER_KEY.len();
                    let player_data = data[index..].to_vec();
                    let values: Vec<&[u8]> = player_data.split(|num| num == &0x00).collect();
                    self.information.players = Vec::with_capacity(values.len());
                    for value in &values {
                        if value.is_empty() {
                            break;
                        }
                        self.information
                            .players
                            .push(String::from_utf8(value.to_vec())?);
                    }
                }
                Ok(())
            }
            _ => Err(anyhow!("unknown response type")),
        }
    }
}
