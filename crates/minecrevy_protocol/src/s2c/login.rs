use minecrevy_core::key::Key;
use minecrevy_io::{
    options::{ListLength, OptionTag},
    McRead, McWrite, Packet,
};
use minecrevy_text::Text;
use uuid::Uuid;

#[derive(Packet, McRead, McWrite, Clone, PartialEq, Debug)]
pub struct Disconnect(pub Text);

#[derive(Packet, McRead, McWrite, Clone, PartialEq, Debug)]
pub struct EncryptionRequest {
    #[options(max_len = 20)]
    pub server_id: String,
    #[options(length = ListLength::VarInt)]
    pub public_key: Vec<u8>,
    #[options(length = ListLength::VarInt)]
    pub verify_token: Vec<u8>,
}

#[derive(Packet, McRead, McWrite, Clone, PartialEq, Debug)]
pub struct Success {
    pub id: Uuid,
    #[options(max_len = 16)]
    pub username: String,
    #[options(length = ListLength::VarInt)]
    pub properties: Vec<ProfileProperty>,
}

#[derive(McRead, McWrite, Clone, PartialEq, Debug)]
pub struct ProfileProperty {
    #[options(max_len = 32767)]
    pub name: String,
    #[options(max_len = 32767)]
    pub value: String,
    #[options(tag = OptionTag::Bool, inner.max_len = 32767)]
    pub signature: Option<String>,
}

#[derive(Packet, McRead, McWrite, Clone, PartialEq, Debug)]
#[meta(EnableCompression)]
pub struct SetCompression {
    #[options(varint = true)]
    pub threshold: i32,
}

#[derive(Packet, McRead, McWrite, Clone, PartialEq, Debug)]
pub struct PluginRequest {
    #[options(varint = true)]
    pub message_id: i32,
    pub channel: Key,
    #[options(length = ListLength::Remaining)]
    pub data: Vec<u8>,
}
