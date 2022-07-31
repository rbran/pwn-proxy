use async_trait::async_trait;
use bitvec::prelude::*;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

use crate::{LigmaRead, LigmaWrite};

#[derive(Clone)]
pub struct Circuit {
    data: BitVec<u8>,
}

impl std::fmt::Debug for Circuit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.data.as_raw_slice().fmt(f)
    }
}

#[async_trait]
impl<R: AsyncRead + Sized + Send + Sync + std::marker::Unpin> LigmaRead<R>
    for Circuit
{
    async fn read(reader: &mut R) -> tokio::io::Result<Circuit> {
        let len: usize = reader.read_u16_le().await?.into();
        let len_bytes = (len + 7) / 8;
        let mut data = vec![0; len_bytes];
        reader.read_exact(&mut data).await?;
        let mut data = BitVec::from_vec(data);
        unsafe { data.set_len(len) };
        Ok(Self { data })
    }
}
#[async_trait]
impl<W: AsyncWrite + Sized + Send + Sync + std::marker::Unpin> LigmaWrite<W>
    for Circuit
{
    async fn write(&self, writer: &mut W) -> tokio::io::Result<()> {
        writer.write_u16_le(self.data.len().try_into().unwrap()).await?;
        writer.write_all( self.data.as_raw_slice()).await
    }
}
