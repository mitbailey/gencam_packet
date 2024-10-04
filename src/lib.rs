use serde::{Serialize, Deserialize};
use crc::CRC_32_CKSUM;

// Packet
#[derive(Serialize, Deserialize, Debug)]
pub struct GenCamPacket {
    pub packet_type: PacketType,
    pub packet_id: u32,
    pub x_dim: u32,
    pub y_dim: u32,
    pub data: Option<Vec<u8>>,
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

impl GenCamPacket {
    pub fn new(packet_type: PacketType, packet_id: u32, x_dim: u32, y_dim: u32, data: Option<Vec<u8>>) -> Self {
        let mut crc = 0;
        
        if let Some(data) = data.clone() {
            crc = crc::Crc::<u32>::new(&CRC_32_CKSUM).checksum(&data.clone());
        }
        
        GenCamPacket {
            packet_type,
            packet_id,
            x_dim,
            y_dim,
            data,
            crc,
        }
    }
}