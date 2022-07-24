use async_trait::async_trait;
use crate::{Ligma, PacketReader, PacketWriter};
use super::position::{Position, Rotation};

#[derive(Debug, Clone, Copy)]
pub enum Actor {
    Other(u32),
}

#[derive(Debug, Clone)]
pub struct OtherPlayer {
    actor: Actor,
    x1: String,
    x2: String,
    x3: u8,
    x4: u32,
    x5: u32,
    x6: u32,
    x7: u32,
    pos: Position,
    rot: Rotation,
    x10: String,
    x11: u32,
    x12: u16,
    x13: String,
    x14: u8,
}
#[async_trait]
impl Ligma for OtherPlayer {
    async fn read(reader: &mut PacketReader) -> tokio::io::Result<OtherPlayer> {
        Ok(OtherPlayer {
            actor: reader.read().await?,
            x1: reader.read().await?,
            x2: reader.read().await?,
            x3: reader.read().await?,
            x4: reader.read().await?,
            x5: reader.read().await?,
            x6: reader.read().await?,
            x7: reader.read().await?,
            pos: reader.read().await?,
            rot: reader.read().await?,
            x10: reader.read().await?,
            x11: reader.read().await?,
            x12: reader.read().await?,
            x13: reader.read().await?,
            x14: reader.read().await?,
        })
    }
    async fn write(&self, writer: &mut PacketWriter) -> tokio::io::Result<()> {
        writer.write(&self.actor).await?;
        writer.write(&self.x1).await?;
        writer.write(&self.x2).await?;
        writer.write(&self.x3).await?;
        writer.write(&self.x4).await?;
        writer.write(&self.x5).await?;
        writer.write(&self.x6).await?;
        writer.write(&self.x7).await?;
        writer.write(&self.pos).await?;
        writer.write(&self.rot).await?;
        writer.write(&self.x10).await?;
        writer.write(&self.x11).await?;
        writer.write(&self.x12).await?;
        writer.write(&self.x13).await?;
        writer.write(&self.x14).await
    }
}

#[async_trait]
impl Ligma for Actor {
    async fn read(reader: &mut PacketReader) -> tokio::io::Result<Actor> {
        let id = reader.read().await?;
        match id {
            id => Ok(Actor::Other(id)),
        }
    }
    async fn write(&self, writer: &mut PacketWriter) -> tokio::io::Result<()> {
        match self {
            Actor::Other(id) => writer.write(id).await,
        }
    }
}
#[async_trait]
impl Ligma for Option<Actor> {
    async fn read(
        reader: &mut PacketReader,
    ) -> tokio::io::Result<Option<Actor>> {
        let id = reader.read().await?;
        match id {
            0 => Ok(None),
            id => Ok(Some(Actor::Other(id))),
        }
    }
    async fn write(&self, writer: &mut PacketWriter) -> tokio::io::Result<()> {
        match self {
            None => writer.write(&0u32).await,
            Some(Actor::Other(id)) => writer.write(id).await,
        }
    }
}
