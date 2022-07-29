use crate::{LigmaRead, LigmaWrite};
use async_trait::async_trait;
use tokio::io::{AsyncRead, AsyncWrite};

#[derive(Debug, Clone)]
pub enum Quest {
    Other(String),
}

#[async_trait]
impl<R: AsyncRead + Sized + Send + Sync + std::marker::Unpin> LigmaRead<R>
    for Quest
{
    async fn read(reader: &mut R) -> tokio::io::Result<Quest> {
        let name: String = String::read(reader).await?;
        match name {
            name => Ok(Quest::Other(name)),
        }
    }
}
#[async_trait]
impl<W: AsyncWrite + Sized + Send + Sync + std::marker::Unpin> LigmaWrite<W>
    for Quest
{
    async fn write(&self, writer: &mut W) -> tokio::io::Result<()> {
        match self {
            Quest::Other(name) => String::write(name, writer).await,
        }
    }
}
