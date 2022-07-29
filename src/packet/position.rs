use crate::{LigmaRead, LigmaWrite};
use async_trait::async_trait;
use tokio::io::{AsyncRead, AsyncWrite};

#[derive(Debug, Clone, Copy)]
pub struct Position {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct Rotation {
    //TODO the value is actually a f16 like
    r_axis: u16,
    y_axis: u16,
    z_axis: u16,
}
#[derive(Debug, Clone, Copy)]
pub struct PrecissionRotation {
    r_axis: f32,
    y_axis: f32,
    z_axis: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct Velocity {
    x: u16,
    y: u16,
    z: u16,
}

#[async_trait]
impl<R: AsyncRead + Sized + Send + Sync + std::marker::Unpin> LigmaRead<R>
    for Position
{
    async fn read(reader: &mut R) -> tokio::io::Result<Position> {
        let x = f32::read(reader).await?;
        let y = f32::read(reader).await?;
        let z = f32::read(reader).await?;
        Ok(Self { x, y, z })
    }
}
#[async_trait]
impl<W: AsyncWrite + Sized + Send + Sync + std::marker::Unpin> LigmaWrite<W>
    for Position
{
    async fn write(&self, writer: &mut W) -> tokio::io::Result<()> {
        f32::write(&self.x, writer).await?;
        f32::write(&self.y, writer).await?;
        f32::write(&self.z, writer).await
    }
}

#[async_trait]
impl<R: AsyncRead + Sized + Send + Sync + std::marker::Unpin> LigmaRead<R>
    for Rotation
{
    async fn read(reader: &mut R) -> tokio::io::Result<Rotation> {
        let r_axis = u16::read(reader).await?;
        let y_axis = u16::read(reader).await?;
        let z_axis = u16::read(reader).await?;
        Ok(Self {
            r_axis,
            y_axis,
            z_axis,
        })
    }
}
#[async_trait]
impl<W: AsyncWrite + Sized + Send + Sync + std::marker::Unpin> LigmaWrite<W>
    for Rotation
{
    async fn write(&self, writer: &mut W) -> tokio::io::Result<()> {
        u16::write(&self.r_axis, writer).await?;
        u16::write(&self.y_axis, writer).await?;
        u16::write(&self.z_axis, writer).await
    }
}

#[async_trait]
impl<R: AsyncRead + Sized + Send + Sync + std::marker::Unpin> LigmaRead<R>
    for PrecissionRotation
{
    async fn read(reader: &mut R) -> tokio::io::Result<PrecissionRotation> {
        let r_axis = f32::read(reader).await?;
        let y_axis = f32::read(reader).await?;
        let z_axis = f32::read(reader).await?;
        Ok(Self {
            r_axis,
            y_axis,
            z_axis,
        })
    }
}
#[async_trait]
impl<W: AsyncWrite + Sized + Send + Sync + std::marker::Unpin> LigmaWrite<W>
    for PrecissionRotation
{
    async fn write(&self, writer: &mut W) -> tokio::io::Result<()> {
        f32::write(&self.r_axis, writer).await?;
        f32::write(&self.y_axis, writer).await?;
        f32::write(&self.z_axis, writer).await
    }
}

#[async_trait]
impl<R: AsyncRead + Sized + Send + Sync + std::marker::Unpin> LigmaRead<R>
    for Velocity
{
    async fn read(reader: &mut R) -> tokio::io::Result<Velocity> {
        let x = u16::read(reader).await?;
        let y = u16::read(reader).await?;
        let z = u16::read(reader).await?;
        Ok(Self { x, y, z })
    }
}
#[async_trait]
impl<W: AsyncWrite + Sized + Send + Sync + std::marker::Unpin> LigmaWrite<W>
    for Velocity
{
    async fn write(&self, writer: &mut W) -> tokio::io::Result<()> {
        u16::write(&self.x, writer).await?;
        u16::write(&self.y, writer).await?;
        u16::write(&self.z, writer).await
    }
}
