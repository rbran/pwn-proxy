use crate::{LigmaRead, LigmaWrite};
use async_trait::async_trait;
use tokio::io::{AsyncRead, AsyncWrite};

#[derive(Debug, Clone)]
pub enum Weapon {
    Other(String),
}

#[async_trait]
impl<R: AsyncRead + Sized + Send + Sync + std::marker::Unpin> LigmaRead<R>
    for Weapon
{
    async fn read(reader: &mut R) -> tokio::io::Result<Weapon> {
        let name = String::read(reader).await?;
        match name {
            name => Ok(Weapon::Other(name)),
        }
    }
}
#[async_trait]
impl<W: AsyncWrite + Sized + Send + Sync + std::marker::Unpin> LigmaWrite<W>
    for Weapon
{
    async fn write(&self, writer: &mut W) -> tokio::io::Result<()> {
        match self {
            Weapon::Other(name) => String::write(name, writer).await,
        }
    }
}
