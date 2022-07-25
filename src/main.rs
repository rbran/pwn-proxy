mod packet;
use packet::*;

use async_trait::async_trait;

use tokio::io::{BufReader, BufWriter};
use tokio::net::{tcp, TcpListener, TcpSocket};
use tokio::select;
use tokio::sync::mpsc;

#[async_trait]
pub trait Ligma: Sized {
    async fn read(reader: &mut PacketReader) -> tokio::io::Result<Self>;
    async fn write(&self, writer: &mut PacketWriter) -> tokio::io::Result<()>;
}

pub struct PacketReader(BufReader<tcp::OwnedReadHalf>);
impl PacketReader {
    pub async fn read<L: Ligma>(&mut self) -> tokio::io::Result<L> {
        L::read(self).await
    }
}
impl std::ops::Deref for PacketReader {
    type Target = BufReader<tcp::OwnedReadHalf>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for PacketReader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
pub struct PacketWriter(BufWriter<tcp::OwnedWriteHalf>);
impl PacketWriter {
    pub async fn write<L: Ligma>(&mut self, data: &L) -> tokio::io::Result<()> {
        L::write(data, self).await
    }
}
impl std::ops::Deref for PacketWriter {
    type Target = BufWriter<tcp::OwnedWriteHalf>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for PacketWriter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

//read packet from the socket and put it in the channel
async fn packet_to_channel<T: Ligma>(
    mut input: PacketReader,
    channel: mpsc::Sender<T>,
) -> Result<(), &'static str> {
    loop {
        let packet = input.read().await.map_err(|_| "Unable to read packet")?;
        channel
            .send(packet)
            .await
            .map_err(|_| "Unable to send to channel")?;
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args();
    let _bin_name = args.next().unwrap();
    let local = args.next().expect("Param1: proxy host:port");
    let remote = args
        .next()
        .expect("Param2: game server host:port")
        .parse()
        .unwrap(/*TODO*/);

    let client_socket = TcpListener::bind(local).await?;
    let (client_stream, _) = client_socket.accept().await?;
    let (client_stream_read, client_stream_write) = client_stream.into_split();
    let mut client_stream_read =
        PacketReader(BufReader::new(client_stream_read));
    let mut client_stream_write =
        PacketWriter(BufWriter::new(client_stream_write));

    let server_socket = TcpSocket::new_v4()?;
    let server_stream = server_socket.connect(remote).await?;
    let (server_stream_read, server_stream_write) = server_stream.into_split();
    let mut server_stream_read =
        PacketReader(BufReader::new(server_stream_read));
    let mut server_stream_write =
        PacketWriter(BufWriter::new(server_stream_write));

    //TODO timeout
    //first the client will send an auth message
    let auth_req: AuthPacket = client_stream_read.read().await?;
    println!("AuthReq {:?}", auth_req);
    server_stream_write.write(&auth_req).await?;
    //if valid, the server will reply with a packet with the user position
    let auth_ok: AuthOk = server_stream_read.read().await?;
    println!("AuthOk {:?}", auth_ok);
    client_stream_write.write(&auth_ok).await?;

    //after that all packets are `ClientServer` or `ServerClient`
    let (client_sender, mut client_receiver) = mpsc::channel(1024);
    let (server_sender, mut server_receiver) = mpsc::channel(1024);
    let _cs_task = tokio::spawn(packet_to_channel::<ClientServer>(
        client_stream_read,
        client_sender,
    ));
    let _sc_task = tokio::spawn(packet_to_channel::<ServerClient>(
        server_stream_read,
        server_sender,
    ));

    //loop that receive server from client2server or server2client
    loop {
        select! {
            Some(packet) = client_receiver.recv() => {
                //intercepted a packet from client/server
                println!("CS: {:?}", packet);
                server_stream_write.write(&packet).await?;
            },
            Some(packet) = server_receiver.recv() => {
                //intercepted a packet from server/client
                println!("SC: {:?}", packet);
                client_stream_write.write(&packet).await?;
            }
            else => panic!("Channel was closed"),
        }
    }
}
