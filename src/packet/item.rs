use async_trait::async_trait;
use tokio::io::{AsyncRead, AsyncWrite};

use crate::{LigmaRead, LigmaWrite};

#[derive(Debug, Clone)]
pub enum ItemName {
    Other(String),
}

#[derive(Debug, Clone, Copy)]
pub enum ItemId {
    Other(u32),
}

#[async_trait]
impl<R: AsyncRead + Sized + Send + Sync + std::marker::Unpin> LigmaRead<R>
    for ItemName
{
    async fn read(reader: &mut R) -> tokio::io::Result<ItemName> {
        let name = String::read(reader).await?;
        match name {
            name => Ok(ItemName::Other(name)),
        }
    }
}
#[async_trait]
impl<W: AsyncWrite + Sized + Send + Sync + std::marker::Unpin> LigmaWrite<W>
    for ItemName
{
    async fn write(&self, writer: &mut W) -> tokio::io::Result<()> {
        match self {
            ItemName::Other(name) => String::write(name, writer).await,
        }
    }
}

#[async_trait]
impl<R: AsyncRead + Sized + Send + Sync + std::marker::Unpin> LigmaRead<R>
    for ItemId
{
    async fn read(reader: &mut R) -> tokio::io::Result<ItemId> {
        let id = u32::read(reader).await?;
        match id {
            id => Ok(ItemId::Other(id)),
        }
    }
}
#[async_trait]
impl<W: AsyncWrite + Sized + Send + Sync + std::marker::Unpin> LigmaWrite<W>
    for ItemId
{
    async fn write(&self, writer: &mut W) -> tokio::io::Result<()> {
        match self {
            ItemId::Other(id) => u32::write(id, writer).await,
        }
    }
}
