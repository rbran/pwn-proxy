use serde::{Deserialize, Deserializer};

fn from_base64<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    String::deserialize(deserializer).and_then(|string| {
        let data: String = string.lines().collect();
        base64::decode(&data)
            .map_err(|err| serde::de::Error::custom(err.to_string()))
    })
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct Packet {
    pub packet: usize,
    pub peer: usize,
    pub index: usize,
    pub timestamp: f32,
    #[serde(deserialize_with = "from_base64")]
    pub data: Vec<u8>,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct Peer {
    pub peer: usize,
    pub host: String,
    pub port: u16,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct PacketFile {
    pub peers: Vec<Peer>,
    pub packets: Vec<Packet>,
}
