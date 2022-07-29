use super::position::{Position, Rotation};
use crate::{LigmaRead, LigmaWrite};
use async_trait::async_trait;
use tokio::io::{AsyncRead, AsyncWrite};

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
impl<R: AsyncRead + Sized + Send + Sync + std::marker::Unpin> LigmaRead<R>
    for OtherPlayer
{
    async fn read(reader: &mut R) -> tokio::io::Result<OtherPlayer> {
        Ok(OtherPlayer {
            actor: Actor::read(reader).await?,
            x1: String::read(reader).await?,
            x2: String::read(reader).await?,
            x3: u8::read(reader).await?,
            x4: u32::read(reader).await?,
            x5: u32::read(reader).await?,
            x6: u32::read(reader).await?,
            x7: u32::read(reader).await?,
            pos: Position::read(reader).await?,
            rot: Rotation::read(reader).await?,
            x10: String::read(reader).await?,
            x11: u32::read(reader).await?,
            x12: u16::read(reader).await?,
            x13: String::read(reader).await?,
            x14: u8::read(reader).await?,
        })
    }
}
#[async_trait]
impl<W: AsyncWrite + Sized + Send + Sync + std::marker::Unpin> LigmaWrite<W>
    for OtherPlayer
{
    async fn write(&self, writer: &mut W) -> tokio::io::Result<()> {
        Actor::write(&self.actor, writer).await?;
        String::write(&self.x1, writer).await?;
        String::write(&self.x2, writer).await?;
        u8::write(&self.x3, writer).await?;
        u32::write(&self.x4, writer).await?;
        u32::write(&self.x5, writer).await?;
        u32::write(&self.x6, writer).await?;
        u32::write(&self.x7, writer).await?;
        Position::write(&self.pos, writer).await?;
        Rotation::write(&self.rot, writer).await?;
        String::write(&self.x10, writer).await?;
        u32::write(&self.x11, writer).await?;
        u16::write(&self.x12, writer).await?;
        String::write(&self.x13, writer).await?;
        u8::write(&self.x14, writer).await
    }
}

#[async_trait]
impl<R: AsyncRead + Sized + Send + Sync + std::marker::Unpin> LigmaRead<R>
    for Actor
{
    async fn read(reader: &mut R) -> tokio::io::Result<Actor> {
        let id = u32::read(reader).await?;
        match id {
            id => Ok(Actor::Other(id)),
        }
    }
}
#[async_trait]
impl<W: AsyncWrite + Sized + Send + Sync + std::marker::Unpin> LigmaWrite<W>
    for Actor
{
    async fn write(&self, writer: &mut W) -> tokio::io::Result<()> {
        match self {
            Actor::Other(id) => u32::write(id, writer).await,
        }
    }
}
#[async_trait]
impl<R: AsyncRead + Sized + Send + Sync + std::marker::Unpin> LigmaRead<R>
    for Option<Actor>
{
    async fn read(reader: &mut R) -> tokio::io::Result<Option<Actor>> {
        let id = u32::read(reader).await?;
        match id {
            0 => Ok(None),
            id => Ok(Some(Actor::Other(id))),
        }
    }
}
#[async_trait]
impl<W: AsyncWrite + Sized + Send + Sync + std::marker::Unpin> LigmaWrite<W>
    for Option<Actor>
{
    async fn write(&self, writer: &mut W) -> tokio::io::Result<()> {
        match self {
            None => u32::write(&0u32, writer).await,
            Some(Actor::Other(id)) => u32::write(id, writer).await,
        }
    }
}
