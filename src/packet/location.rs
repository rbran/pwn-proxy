use async_trait::async_trait;

use crate::{PacketReader, Ligma, PacketWriter};

#[derive(Debug, Clone)]
pub enum Location {
    Other(String),
}

#[async_trait]
impl Ligma for Location {
    async fn read(reader: &mut PacketReader) -> tokio::io::Result<Location> {
        let name: String = reader.read().await?;
        match name {
            name => Ok(Location::Other(name)),
        }
    }
    async fn write(&self, writer: &mut PacketWriter) -> tokio::io::Result<()> {
        match self {
            Location::Other(name) => writer.write(name).await,
        }
    }
}
