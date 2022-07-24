mod actor;
mod item;
mod location;
mod position;
mod quest;
mod weapon;
use actor::*;
use item::*;
use location::*;
use position::*;
use quest::*;
use weapon::*;

use crate::{Ligma, PacketReader, PacketWriter};
use async_trait::async_trait;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

//recv: 00242760
#[derive(Debug, Clone)]
pub struct AuthPacket {
    actor: Actor,
    token: String,
}
//send: 00242a0c
#[derive(Debug, Clone)]
pub struct AuthOk {
    x0: u32,
    position: Position,
    rotation: Rotation,
}

#[async_trait]
impl Ligma for AuthPacket {
    async fn read(reader: &mut PacketReader) -> tokio::io::Result<AuthPacket> {
        Ok(Self {
            actor: reader.read().await?,
            token: reader.read().await?,
        })
    }
    async fn write(&self, writer: &mut PacketWriter) -> tokio::io::Result<()> {
        writer.write(&self.actor).await?;
        writer.write(&self.token).await?;
        writer.flush().await
    }
}

#[async_trait]
impl Ligma for AuthOk {
    async fn read(reader: &mut PacketReader) -> tokio::io::Result<AuthOk> {
        Ok(Self {
            x0: reader.read().await?,
            position: reader.read().await?,
            rotation: reader.read().await?,
        })
    }
    async fn write(&self, writer: &mut PacketWriter) -> tokio::io::Result<()> {
        writer.write(&self.x0).await?;
        writer.write(&self.position).await?;
        writer.write(&self.rotation).await?;
        writer.flush().await
    }
}

macro_rules! packet_class {
    ($name:ident,
         $(
            $packet_name:ident($value:literal) =>
                { $($param_name:ident:$param_type:ty),* $(,)? }
         ),*
     $(,)?) => {
        #[derive(Debug, Clone)]
        pub enum $name {
            $(
                $packet_name{$($param_name: $param_type),*},
            )*
        }
        #[async_trait]
        impl Ligma for $name {
            async fn read(
                reader: &mut PacketReader
            ) -> tokio::io::Result<$name> {
                let id = reader.read_u16_le().await?;
                println!("pkt id 0x{:02x}", id);
                match id {
                    $(
                        $value => {
                            Ok($name::$packet_name{
                                $(
                                    $param_name: <$param_type as Ligma>::read(
                                        reader
                                    ).await?
                                ),*
                            })
                        },
                    )*
                    id => unreachable!("Unknown id packet 0x{:04x}", id),
                }
            }
            async fn write(
                &self,
                writer: &mut PacketWriter
            ) -> tokio::io::Result<()> {
                match self {
                    $(
                     $name::$packet_name{$($param_name),*} => {
                        writer.write_u16_le($value).await?;
                        $(
                            writer.write($param_name).await?;
                        )*
                        writer.flush().await?;
                     }
                    )*
                }
                Ok(())
            }
        }
    };
}
// Server to Client messages.
// Sent by class `ServerWorld` by functions with names `Send{}Event`.
// Received by class `GameServerConnection` in the function at:002c9d90
packet_class!(ServerClient,
    //TODO find use and verify that is correct
    FinishedFlushing(0x0000) => {},

    //send: 00326150, send: 00326b70, recv: 002ca27e
    ActorSpawn(0x6b6d) => {
        actor: Actor,
        x1: u32,
        x2: u8,
        x3: String,
        position: Position,
        rotation: Rotation,
        x6: u32,
    },

    ////send: 00326500, recv: 002ca139
    ExistingPlayer(0x6e63) => {player: OtherPlayer},
    ////send: 00326460, recv: 002ca56c
    ActorDestroy(0x7878) => {actor: Actor},
    ////send: 00327320, recv: 002ca378
    ActorPosition(0x7070) => {
        actor: Actor,
        position: Position,
        rotation: Rotation,
        velocity: Velocity,
        forward_fraction: u8,
        strafe_fraction: u8,
    },
    ////send: 00327130, recv: 002c9faf
    CircuitOutput(0x3130) => {x0: String, x1: u32, x2: String},
    ////send: 00324980, recv: 002ca184
    CountdownUpdate(0x6463) => {count: u32},
    ////send: 00325050, recv: 002ca521
    Display(0x7665) => {x0: String, x1: String},
    ////send: 00324e30, recv: 002c9f57
    FireBullets(0x2a2a) => {
        actor: Actor,
        weapon: Weapon,
        position: Position,
        x3: u8,
        x4: f32
    },
    ////send: 00324820, recv: 002c9f83
    HealthUpdate(0x2b2b) => {actor: Actor, x1: u32, x2: u32},
    ////send: 00326f50, recv: 002c9fdb
    Kill(0x392d) => {
        dead: Actor,
        x1: u32,
        killer: Option<Actor>,
        x3: u32,
        x4: String
    },
    ////send: 00325fa0, recv: 002ca24c
    PlayerItem(0x6970) => {x0: Actor, x1: String},
    ////send: 00325880, recv: 002ca139
    PlayerJoined(0x636e) => {player: OtherPlayer},
    ////send: 00325ef0, recv: 002ca107
    PlayerLeft(0x635e) => {left: Actor},
    ////send: 00324a20, recv: 002c9ebd
    PvPCountdownUpdate(0x18e2) => {x0: u8, x1: u32},
    ////send: 00324ae0, recv: 002ca553
    PvPEnable(0x7670) => {enabled: bool},
    ////send: 003279d0, recv: 002ca1e8
    RegionChange(0x6863) => {region: Location},
    ////send: 00325530, recv: 002ca4bd
    RelativeTeleport(0x7472) => {actor: Actor, relative_pos: Position},
    ////send: 003255f0, recv: 002ca2b0
    Reload(0x6c72) => {x0: String, x1: String, x2: u32},
    ////send: 003255f0, recv: 002ca3f5
    RemoteReload(0x7272) => {x0: Actor, x1: u32},
    ////send: 00323de0, recv: 002ca2e2
    RemoveItem(0x6d72) => {item: ItemName, amount: u32},
    ////send: 00327320, recv: 002ca472
    PositionAndVelocity(0x7370) => {
        actor: Actor,
        position: Position,
        rotation: Rotation,
        velocity: Velocity,
    },
    ////send: 002c9d90, recv: 002ca53a
    Position(0x766d) => {
        position: Position,
        rotation: Rotation,
        velocity: Velocity,
        forward_fraction: u8,
        strafe_fraction: u8,
    },
    ////send: 00325300, recv: 002ca48b
    SpawnThisPlayer(0x7372) => {position: Position, rotation: Rotation},
    ////send: 00325300, recv: 002ca314
    RespawnOtherPlayer(0x6f72) => {
        player: Actor,
        position: Position,
        rotation: Rotation
    },
    ////send: 00324b90, recv: 002ca4d6
    State(0x7473) => {x0: Actor, x1: String, x2: u8},
    ////send: 00325460, recv: 002ca391
    Teleport(0x7074) => {
        actor: Actor,
        to_position: Position,
        with_rotation: Rotation
    },
    ////send: 00324cc0, recv: 002ca40e
    Trigger(0x7274) => {x0: Actor, x1: String, x2: Option<Actor>},

    ////User related
    //send: 00326e90, recv: 002c9f2b
    Chat(0x2a23) => {source: Actor, message: String},
    ////send: 00323cb0, recv: 002ca346
    AddItem(0x7063) => {item: ItemName, ammount: u32},
    ////send: 003248e0, recv: 002ca0d5
    ManaUpdate(0x616d) => {new_mana: u32},
    ////send: 00324210, recv: 002ca058
    CurrentSlot(0x3d73) => {slot: u8},
    ////send: 003240e0, recv: 002ca00d
    EquipItem(0x3d69) => {x0: u8, item: ItemName},
    ////send: 00327a70, recv: 002ca21a
    LastHitByItem(0x686c) => {x0: String},
    ////send: 00323f10, recv: 002ca0bc
    LoadedAmmo(0x616c) => {x0: String, ammount: u32},
    ////0x6623, send: 003251c0, recv: 002ca1b6
    NPCConversationEnd(0x6623) => {},
    ////send: 00325100, recv: 002ca440
    NPCConversationState(0x7323) => {npc: Actor, x1: String},
    ////send: 00325250, recv: 002c9ee9
    NPCShop(0x2424) => {npc: Actor},
    ////send: 00324040, recv: 002ca508
    PickedUp(0x7570) => {x0: String},

    ////Quest stuff
    ////send: 00324710, recv: 002ca16b
    CompleteQuest(0x645e) => {quest: Quest},
    ////send: 00324400, recv: 002ca3c3
    StartQuest(0x716e) => {quest: Quest},
    ////send: 003242b0, recv: 002ca03f
    SetCurrentQuest(0x3d71) => {quest: Quest},
    ////send: 00324710, recv: 002ca08a
    AdvanceQuestToState(0x3e71) => {x0: String, quest: Quest},
);

// Client to Server messages.
// Sent by class `GameServerConnection` by multiple functions.
// Received by the function `ClientHandler::ProcessCommands` at:00242760
packet_class!(ClientServer,
    //recv: 002bf9e0
    Disconnect(0x0000) => {},
    //send: 002c5aa0, recv: 00242b5a
    Chat(0x2a23) => {message: String},
    ////send: 002c4720, recv: 00242b92
    CircuitOutput(0x3130) => {x0: Vec<u8>, x1: u32},
    ////send: 002c6840, recv: 00242d36
    EquipItem(0x3d69) => {x0: u8, item: ItemName},
    ////send: 002c5f80, recv: 00242d52
    SetCurrentQuest(0x3d71) => {quest: Quest},
    ////send: 002c63f0, recv: 00242d6e
    CurrentSlot(0x3d73) => {slot: u8},
    ////send: 002c7fe0, recv: 00242d8a
    TransitionToNPCState(0x3e23) => {x0: String},
    ////send: 002c7ae0, recv: 00242da6
    ByItem(0x6224) => {x0: u32, x1: String, x2: u32},
    ////send: 002c9980, recv: 00242dc2
    Use(0x6565) => {x0: u32},
    ////send: 002c94f0, recv: 00242dde
    Activate(0x692a) => {x0: String, x1: PrecissionRotation},
    ////send: 002c90c0, recv: 00242dfa
    Reload(0x6c72) => {},
    ////send: 002c8890, recv: 00242e16
    Sprint(0x6e72) => {run: bool},
    ////send: 002c8cb0, recv: 00242e32
    Jump(0x706a) => {x0: bool},
    ////send: 002c6d10, recv: 00242e4e
    Teleport(0x7074) => {x0: String},
    ////send: 002c8470, recv: 00242e6a
    FireRequest(0x7266) => {x0: bool},
    ////send: 002c75d0, recv: 00242e86
    SellItem(0x7324) => {x0: u32, x1: String, x2: u32},
    ////send: 002c7190, recv: 00242ea2
    SpawnThisPlayer(0x7372) => {},
    ////send: 002c5560, recv: 00242ebe
    FastTravel(0x7466) => {x0: String, x1: String},
    ////send: 002c9d90, recv: 00242eda
    Position(0x766d) => {
        position: Position,
        rotation: Rotation,
        forward_fraction: u8,
        strafe_fraction: u8,
    },
    ////send: 002c50b0, recv: 00242ef6
    PvPEnable(0x7670) => {enabled: bool},
    ////send: 002c4c20, recv: 00242f12
    DLCKey(0x796b) => {key: String},
);

#[async_trait]
impl Ligma for String {
    async fn read(reader: &mut PacketReader) -> tokio::io::Result<String> {
        let data = <Vec<u8> as Ligma>::read(reader).await?;
        Ok(String::from_utf8(data).unwrap())
    }
    async fn write(&self, writer: &mut PacketWriter) -> tokio::io::Result<()> {
        writer
            .write_u16_le(self.len().try_into().unwrap(/*TODO*/))
            .await?;
        writer.write_all(self.as_bytes()).await
    }
}
#[async_trait]
impl Ligma for Vec<u8> {
    async fn read(reader: &mut PacketReader) -> tokio::io::Result<Vec<u8>> {
        let len = reader.read_u16_le().await?;
        let mut data = vec![0; len.into()];
        reader.read_exact(&mut data).await?;
        Ok(data)
    }
    async fn write(&self, writer: &mut PacketWriter) -> tokio::io::Result<()> {
        writer
            .write_u16(self.len().try_into().unwrap(/*TODO*/))
            .await?;
        writer.write_all(&self).await
    }
}
#[async_trait]
impl Ligma for u32 {
    async fn read(reader: &mut PacketReader) -> tokio::io::Result<u32> {
        reader.read_u32_le().await
    }
    async fn write(&self, writer: &mut PacketWriter) -> tokio::io::Result<()> {
        writer.write_u32_le(*self).await
    }
}
#[async_trait]
impl Ligma for u16 {
    async fn read(reader: &mut PacketReader) -> tokio::io::Result<u16> {
        reader.read_u16_le().await
    }
    async fn write(&self, writer: &mut PacketWriter) -> tokio::io::Result<()> {
        writer.write_u16_le(*self).await
    }
}
#[async_trait]
impl Ligma for u8 {
    async fn read(reader: &mut PacketReader) -> tokio::io::Result<u8> {
        reader.read_u8().await
    }
    async fn write(&self, writer: &mut PacketWriter) -> tokio::io::Result<()> {
        writer.write_u8(*self).await
    }
}
#[async_trait]
impl Ligma for bool {
    async fn read(reader: &mut PacketReader) -> tokio::io::Result<bool> {
        reader.read_u8().await.map(|x| x != 0)
    }
    async fn write(&self, writer: &mut PacketWriter) -> tokio::io::Result<()> {
        writer.write_u8((*self).into()).await
    }
}
#[async_trait]
impl Ligma for f32 {
    async fn read(reader: &mut PacketReader) -> tokio::io::Result<f32> {
        reader.read_f32().await
    }
    async fn write(&self, writer: &mut PacketWriter) -> tokio::io::Result<()> {
        writer.write_f32(*self).await
    }
}
