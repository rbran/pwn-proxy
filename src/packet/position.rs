use async_trait::async_trait;
use crate::{Ligma, PacketReader, PacketWriter};

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
impl Ligma for Position {
    async fn read(reader: &mut PacketReader) -> tokio::io::Result<Position> {
        let x = reader.read().await?;
        let y = reader.read().await?;
        let z = reader.read().await?;
        Ok(Self { x, y, z })
    }
    async fn write(&self, writer: &mut PacketWriter) -> tokio::io::Result<()> {
        self.x.write(writer).await?;
        self.y.write(writer).await?;
        self.z.write(writer).await
    }
}

#[async_trait]
impl Ligma for Rotation {
    async fn read(reader: &mut PacketReader) -> tokio::io::Result<Rotation> {
        let r_axis = reader.read().await?;
        let y_axis = reader.read().await?;
        let z_axis = reader.read().await?;
        Ok(Self { r_axis, y_axis, z_axis })
    }
    async fn write(&self, writer: &mut PacketWriter) -> tokio::io::Result<()> {
        self.r_axis.write(writer).await?;
        self.y_axis.write(writer).await?;
        self.z_axis.write(writer).await
    }
}

#[async_trait]
impl Ligma for PrecissionRotation {
    async fn read(reader: &mut PacketReader) -> tokio::io::Result<PrecissionRotation> {
        let r_axis = reader.read().await?;
        let y_axis = reader.read().await?;
        let z_axis = reader.read().await?;
        Ok(Self { r_axis, y_axis, z_axis })
    }
    async fn write(&self, writer: &mut PacketWriter) -> tokio::io::Result<()> {
        self.r_axis.write(writer).await?;
        self.y_axis.write(writer).await?;
        self.z_axis.write(writer).await
    }
}

#[async_trait]
impl Ligma for Velocity {
    async fn read(reader: &mut PacketReader) -> tokio::io::Result<Velocity> {
        let x = reader.read().await?;
        let y = reader.read().await?;
        let z = reader.read().await?;
        Ok(Self { x, y, z })
    }
    async fn write(&self, writer: &mut PacketWriter) -> tokio::io::Result<()> {
        self.x.write(writer).await?;
        self.y.write(writer).await?;
        self.z.write(writer).await
    }
}
