use minecrevy_io::{
    options::{ListLength, OptionTag},
    McRead, McWrite, Packet,
};
use uuid::Uuid;

#[derive(Packet, McRead, McWrite, Clone, PartialEq, Debug)]
pub struct Start {
    #[options(max_len = 16)]
    pub name: String,
    #[options(tag = OptionTag::Bool)]
    pub id: Option<Uuid>,
}

#[derive(Packet, McRead, McWrite, Clone, PartialEq, Debug)]
#[meta(EnableEncryption)]
pub struct EncryptionResponse {
    #[options(length = ListLength::VarInt)]
    pub shared_secret: Vec<u8>,
    #[options(length = ListLength::VarInt)]
    pub verify_token: Vec<u8>,
}

#[derive(Packet, McRead, McWrite, Clone, PartialEq, Debug)]
pub struct PluginResponse {
    pub message_id: i32,
    #[options(tag = OptionTag::Bool)]
    pub data: Option<Vec<u8>>,
}
