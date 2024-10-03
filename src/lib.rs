use serde::{Serialize, Deserialize};

// Packet
#[derive(Serialize, Deserialize, Debug)]
struct GenCamPacket {
    packet_type: PacketType,
    packet_id: u32,
    data: Vec<u8>,
    crc: u32,
}

// Comms type
#[derive(Serialize, Deserialize, Debug)]
enum PacketType {
    Ack,
    NAck,
    Data,
}