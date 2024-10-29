use std::hash::Hash;

use serde::{Serialize, Deserialize};
use crc::CRC_32_CKSUM;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GenCamHeader {
    source: u32,
    destination: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GenCamFooter {
    crc: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum GenCamPacket {
    Ack{header: GenCamHeader, footer: GenCamFooter},
    NAck{header: GenCamHeader, footer: GenCamFooter},
    Image{header: GenCamHeader, data: Vec<u8>, width: u32, height: u32, footer: GenCamFooter},
    ImageRequest{header: GenCamHeader, footer: GenCamFooter},
    ProducersRequest{header: GenCamHeader, footer: GenCamFooter},
    Producers{header: GenCamHeader, producers: Vec<u32>, footer: GenCamFooter},
    Subscription{header: GenCamHeader, producer: u32, footer: GenCamFooter},
}

impl GenCamPacket {
    pub fn ack() -> Self {
        GenCamPacket::Ack{header: GenCamHeader{source: 0, destination: 0}, footer: GenCamFooter{crc: 0}}
    }

    pub fn nack() -> Self {
        GenCamPacket::NAck{header: GenCamHeader{source: 0, destination: 0}, footer: GenCamFooter{crc: 0}}
    }

    pub fn image(data: Vec<u8>, width: u32, height: u32) -> Self {
        GenCamPacket::Image{header: GenCamHeader{source: 0, destination: 0}, data, width, height, footer: GenCamFooter{crc: 0}}
    }

    pub fn image_request() -> Self {
        GenCamPacket::ImageRequest{header: GenCamHeader{source: 0, destination: 0}, footer: GenCamFooter{crc: 0}}
    }

    pub fn producers_request() -> Self {
        GenCamPacket::ProducersRequest{header: GenCamHeader{source: 0, destination: 0}, footer: GenCamFooter{crc: 0}}
    }

    pub fn producers(producers: Vec<u32>) -> Self {
        GenCamPacket::Producers{header: GenCamHeader{source: 0, destination: 0}, producers, footer: GenCamFooter{crc: 0}}
    }

    pub fn subscription(producer: u32) -> Self {
        GenCamPacket::Subscription{header: GenCamHeader{source: 0, destination: 0}, producer, footer: GenCamFooter{crc: 0}}
    }

    pub fn from_bytes(bytes: Vec<u8>) -> std::result::Result<Self, serde_json::Error> {
        serde_json::from_slice(&bytes)
    }

    pub fn to_bytes(&self) -> std::result::Result<Vec<u8>, serde_json::Error> {
        serde_json::to_vec(&self)
    }

    pub fn get_data(&self) -> Option<Vec<u8>> {
        match self {
            GenCamPacket::Image{data, ..} => Some(data.clone()),
            _ => None,
        }
    }
}