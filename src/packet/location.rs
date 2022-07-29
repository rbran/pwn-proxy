use async_trait::async_trait;
use tokio::io::{AsyncRead, AsyncWrite};

use crate::{LigmaRead, LigmaWrite};

#[derive(Debug, Clone)]
pub enum Location {
    Other(String),
}

#[async_trait]
impl<R: AsyncRead + Sized + Send + Sync + std::marker::Unpin> LigmaRead<R>
    for Location
{
    async fn read(reader: &mut R) -> tokio::io::Result<Location> {
        let name = String::read(reader).await?;
        match name {
            name => Ok(Location::Other(name)),
        }
    }
}
#[async_trait]
impl<W: AsyncWrite + Sized + Send + Sync + std::marker::Unpin> LigmaWrite<W>
    for Location
{
    async fn write(&self, writer: &mut W) -> tokio::io::Result<()> {
        match self {
            Location::Other(name) => String::write(name, writer).await,
        }
    }
}
