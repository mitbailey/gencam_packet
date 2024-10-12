use std::hash::Hash;

use serde::{Serialize, Deserialize};
use crc::CRC_32_CKSUM;

// Frame read by the router to determination destination.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GenCamFrame {
    pub source: u32,
    pub destination: u32,
    pub frame_type: FrameType,
    pub packet: GenCamPacket,
}

// Comms type
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum FrameType {
    Id,
    Comms,
}

impl GenCamFrame {
    pub fn new(source: u32, destination: u32, packet_type: PacketType, packet_id: u32, x_dim: u32, y_dim: u32, 
        data: Option<Vec<u8>>) -> Self {

        let mut crc = 0;
    
        if let Some(data) = data.clone() {
            crc = crc::Crc::<u32>::new(&CRC_32_CKSUM).checksum(&data.clone());
        }

        GenCamFrame {
            source,
            destination,
            frame_type: FrameType::Comms,
            packet: GenCamPacket {
                packet_type,
                packet_id,
                x_dim,
                y_dim,
                data,
                crc,
            },
        }
    }

    // Change name to deserialize?
    pub fn from_bytes(bytes: Vec<u8>) -> std::result::Result<Self, serde_json::Error> {
        serde_json::from_slice(&bytes)
    }
}

// Packet
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GenCamPacket {
    pub packet_type: PacketType,
    pub packet_id: u32,
    // TODO: x_dim and y_dim probably should not be hardcoded.
    pub x_dim: u32,
    pub y_dim: u32,
    pub data: Option<Vec<u8>>,
    pub crc: u32,
}

// Comms type
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum PacketType {
    Ack,
    NAck,
    ImgReq,
    Image,
    Data,
    Id, // Router assignment of sink ID.
    ProdReq, // Request for router to send list of producers to subscribe to.
    Prods, // List of producers a client can subscribe to.
    Sub, // Subscribe to one producer.
}

impl PartialEq for PacketType {
    fn eq(&self, other: &Self) -> bool {
        matches!((self, other), (PacketType::Ack, PacketType::Ack) | (PacketType::NAck, PacketType::NAck) | (PacketType::ImgReq, PacketType::ImgReq) | (PacketType::Image, PacketType::Image) | (PacketType::Data, PacketType::Data) | (PacketType::Id, PacketType::Id) | (PacketType::ProdReq, PacketType::ProdReq) | (PacketType::Prods, PacketType::Prods) | (PacketType::Sub, PacketType::Sub))
    }
}