mod packet;

mod file;

use std::io::Cursor;

use async_trait::async_trait;

use tokio::fs::File;
use tokio::io::{
    AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt, BufReader, BufWriter,
};
use tokio::net::{TcpListener, TcpSocket};
use tokio::select;
use tokio::sync::mpsc;

#[async_trait]
pub trait LigmaRead<R>: Sized
where
    R: AsyncRead + Sized,
{
    async fn read(reader: &mut R) -> tokio::io::Result<Self>;
}
#[async_trait]
pub trait LigmaWrite<W>
where
    W: AsyncWrite,
{
    async fn write(&self, writer: &mut W) -> tokio::io::Result<()>;
}

//read packet from the socket and put it in the channel
async fn packet_to_channel<R, L>(
    mut input: R,
    channel: mpsc::Sender<L>,
) -> Result<(), &'static str>
where
    R: AsyncRead + Sized,
    L: LigmaRead<R>,
{
    loop {
        let packet = L::read(&mut input)
            .await
            .map_err(|_| "Unable to read packet")?;
        channel
            .send(packet)
            .await
            .map_err(|_| "Unable to send to channel")?;
    }
}

async fn net_mode(
    local: &str,
    remote: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let client_socket = TcpListener::bind(local).await?;

    loop {
        let (client_stream, _) = client_socket.accept().await?;
        let (client_stream_read, client_stream_write) =
            client_stream.into_split();
        let mut client_stream_read = BufReader::new(client_stream_read);
        let mut client_stream_write = BufWriter::new(client_stream_write);

        let remote = remote.parse().unwrap(/*TODO*/);
        let server_socket = TcpSocket::new_v4()?;
        let server_stream = server_socket.connect(remote).await?;
        let (server_stream_read, server_stream_write) =
            server_stream.into_split();
        let mut server_stream_read = BufReader::new(server_stream_read);
        let mut server_stream_write = BufWriter::new(server_stream_write);

        //TODO timeout
        //first the client will send an auth message
        let auth_req =
            packet::AuthPacket::read(&mut client_stream_read).await?;
        println!("AuthReq {:?}", auth_req);
        packet::AuthPacket::write(&auth_req, &mut server_stream_write).await?;
        server_stream_write.flush().await?;
        //if valid, the server will reply with a packet with the user position
        let auth_ok = packet::AuthOk::read(&mut server_stream_read).await?;
        println!("AuthOk {:?}", auth_ok);
        packet::AuthOk::write(&auth_ok, &mut client_stream_write).await?;
        client_stream_write.flush().await?;

        //after that all packets are `ClientServer` or `ServerClient`
        let (client_sender, mut client_receiver) = mpsc::channel(1024);
        let (server_sender, mut server_receiver) = mpsc::channel(1024);
        let _cs_task =
            tokio::spawn(packet_to_channel(client_stream_read, client_sender));
        let _sc_task =
            tokio::spawn(packet_to_channel(server_stream_read, server_sender));

        //loop that receive server from client2server or server2client
        loop {
            select! {
                Some(packet) = client_receiver.recv() => {
                    //intercepted a packet from client/server
                    println!("CS: {:#?}", packet);
                    packet::ClientServer::write(&packet, &mut server_stream_write).await?;
                    server_stream_write.flush().await?;
                },
                Some(packet) = server_receiver.recv() => {
                    //intercepted a packet from server/client
                    println!("SC: {:#?}", packet);
                    packet::ServerClient::write(&packet, &mut client_stream_write).await?;
                    client_stream_write.flush().await?;
                }
                else => {
                    //TODO only for connection closed
                    break
                },
            }
        }
    }
}

async fn file_mode(file: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open(file).await?;
    let mut data = String::new();
    file.read_to_string(&mut data).await?;
    let file: file::PacketFile = serde_yaml::from_str(&data)?;
    let (server, _client) = match &file.peers[..] {
        [file::Peer {
            peer: server,
            port: 3003,
            ..
        }, file::Peer { peer: client, .. }]
        | [file::Peer { peer: client, .. }, file::Peer {
            peer: server,
            port: 3003,
            ..
        }] => (*server, *client),
        _ => panic!("File have too many peers"),
    };

    let mut packets = file.packets.iter();

    let packet = packets.next().unwrap(/*TODO*/);
    let mut cur_req = Cursor::new(&packet.data);
    let auth_req = packet::AuthPacket::read(&mut cur_req).await?;
    println!("Auth {:#?}", auth_req);
    assert!(cur_req.read_u8().await.is_err());

    let packet = packets.next().unwrap(/*TODO*/);
    let mut cur_ok = Cursor::new(&packet.data);
    let auth_ok = packet::AuthOk::read(&mut cur_ok).await?;
    println!("Auth Ok {:#?}", auth_ok);
    assert!(cur_ok.read_u8().await.is_err());

    for packet in packets {
        let mut stream = Cursor::new(&packet.data);
        println!("Packet ID {}", &packet.packet);
        loop {
            if packet.peer == server {
                let packet = packet::ServerClient::read(&mut stream).await?;
                println!("SC: {:#?}", packet);
            } else {
                let packet = packet::ClientServer::read(&mut stream).await?;
                println!("CS: {:#?}", packet);
            }
            if stream.position() == packet.data.len() as u64 {
                break;
            }
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args();
    let _bin_name = args.next().unwrap();
    match args.next().expect("Param1: proxy host:port").as_ref() {
        "net" => {
            let local = args.next().expect("net Param1: proxy host:port");
            let remote =
                args.next().expect("net Param2: game server host:port");
            net_mode(&local, &remote).await
        }
        "file" => {
            let file = args.next().expect("file Param1: filename");
            file_mode(&file).await
        }
        _ => panic!("Invalid mode, available net/file"),
    }
}
