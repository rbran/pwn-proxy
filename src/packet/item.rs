use async_trait::async_trait;

use crate::{Ligma, PacketReader, PacketWriter};

#[derive(Debug, Clone)]
pub enum ItemName {
    Other(String),
}

#[derive(Debug, Clone, Copy)]
pub enum ItemId {
    Other(u32),
}

#[async_trait]
impl Ligma for ItemName {
    async fn read(reader: &mut PacketReader) -> tokio::io::Result<ItemName> {
        let name: String = reader.read().await?;
        match name {
            name => Ok(ItemName::Other(name)),
        }
    }
    async fn write(&self, writer: &mut PacketWriter) -> tokio::io::Result<()> {
        match self {
            ItemName::Other(name) => writer.write(name).await,
        }
    }
}

#[async_trait]
impl Ligma for ItemId {
    async fn read(reader: &mut PacketReader) -> tokio::io::Result<ItemId> {
        let id = reader.read().await?;
        match id {
            id => Ok(ItemId::Other(id)),
        }
    }
    async fn write(&self, writer: &mut PacketWriter) -> tokio::io::Result<()> {
        match self {
            ItemId::Other(id) => writer.write(id).await,
        }
    }
}
