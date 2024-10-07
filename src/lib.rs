use std::hash::Hash;

use serde::{Serialize, Deserialize};
use crc::CRC_32_CKSUM;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NodeID {
    pub id: u32,
    pub name: String,
}

impl PartialEq for NodeID {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.name == other.name
    }
}

impl Eq for NodeID {}

impl Hash for NodeID {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.name.hash(state);
    }
}

// Frame read by the router to determination destination.
#[derive(Serialize, Deserialize, Debug)]
pub struct GenCamFrame {
    pub source: NodeID,
    pub destination: NodeID,
    pub frame_type: FrameType,
    pub packet: GenCamPacket,
}

// Comms type
#[derive(Serialize, Deserialize, Debug)]
pub enum FrameType {
    Id,
    Comms,
}

impl GenCamFrame {
    pub fn new(source: NodeID, destination: NodeID, packet_type: PacketType, packet_id: u32, x_dim: u32, y_dim: u32, 
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

    pub fn identifier(source: NodeID, destination: NodeID) -> Self {
        GenCamFrame {
            source,
            destination,
            frame_type: FrameType::Id,
            packet: GenCamPacket {
                packet_type: PacketType::Ack,
                packet_id: 0,
                x_dim: 0,
                y_dim: 0,
                data: None,
                crc: 0,
            },
        }
    }
}

// Packet
#[derive(Serialize, Deserialize, Debug)]
struct GenCamPacket {
    packet_type: PacketType,
    packet_id: u32,
    x_dim: u32,
    y_dim: u32,
    data: Option<Vec<u8>>,
    crc: u32,
}

// Comms type
#[derive(Serialize, Deserialize, Debug)]
pub enum PacketType {
    Ack,
    NAck,
    ImgReq,
    Image,
    Data,
}