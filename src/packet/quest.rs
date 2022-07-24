use async_trait::async_trait;
use crate::{PacketReader, PacketWriter, Ligma};

#[derive(Debug, Clone)]
pub enum Quest {
    Other(String),
}

#[async_trait]
impl Ligma for Quest {
    async fn read(reader: &mut PacketReader) -> tokio::io::Result<Quest> {
        let name: String = reader.read().await?;
        match name {
            name => Ok(Quest::Other(name)),
        }
    }
    async fn write(&self, writer: &mut PacketWriter) -> tokio::io::Result<()> {
        match self {
            Quest::Other(name) => writer.write(name).await,
        }
    }
}
