use serde::{Serialize, Deserialize};

// Packet
#[derive(Serialize, Deserialize, Debug)]
struct GenCamPacket {

}

// Comms type
enum PacketType {
    Acknowledged,
    NotAcknowledged,
    Data,
    Image,
}