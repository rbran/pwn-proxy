use async_trait::async_trait;
use crate::{PacketReader, Ligma, PacketWriter};

#[derive(Debug, Clone)]
pub enum Weapon {
    Other(String),
}

#[async_trait]
impl Ligma for Weapon {
    async fn read(reader: &mut PacketReader) -> tokio::io::Result<Weapon> {
        let name: String = reader.read().await?;
        match name {
            name => Ok(Weapon::Other(name)),
        }
    }
    async fn write(&self, writer: &mut PacketWriter) -> tokio::io::Result<()> {
        match self {
            Weapon::Other(name) => writer.write(name).await,
        }
    }
}
