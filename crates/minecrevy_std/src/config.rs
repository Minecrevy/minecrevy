//! This module contains the [`ConfigPlugin`], which handles the configuration
//! process.

use bevy::prelude::*;
use minecrevy_io::packet::RawPacket;
use minecrevy_net::{
    client::{PacketWriter, ProtocolState},
    packet::Recv,
};
use minecrevy_protocol::config::{
    AcknowledgeFinish, ClientInformation, DataPack, Finish, KnownDataPacks,
};

use crate::play::EnterPlay;

/// [`Event`] that's triggered when a client has finished the login process, or
/// when the client is in play state but needs to be updated with the latest
/// configuration.
#[derive(Event, Clone, PartialEq, Debug)]
pub struct EnterConfig {
    /// The previous state, either [`Login`](ProtocolState::Login) or
    /// [`Play`](ProtocolState::Play).
    pub previous_state: ProtocolState,
}

/// [`Plugin`] for handling the configuration process.
#[derive(Default)]
pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(Self::on_enter_config);
        app.add_observer(Self::on_client_information);
        app.add_observer(Self::on_known_data_packs);
        app.add_observer(Self::on_acknowledge_finish);
    }
}

impl ConfigPlugin {
    /// [`Observer`] [`System`] that handles the configuration state being entered.
    pub fn on_enter_config(trigger: Trigger<EnterConfig>, mut writer: PacketWriter) {
        let client = trigger.entity();

        writer.send(
            client,
            &KnownDataPacks {
                packs: Vec::from_iter([DataPack {
                    id: "core".into(),
                    namespace: "minecraft".into(),
                    version: "1.21.1".into(),
                }]),
            },
        );
    }

    /// [`Observer`] [`System`] that handles incoming [`ClientInformation`] packets.
    pub fn on_client_information(
        trigger: Trigger<Recv<ClientInformation>>,
        mut commands: Commands,
    ) {
        let packet = &trigger.event().0;
        let client = trigger.entity();

        commands.entity(client).insert(ClientInfo::from(packet));
    }

    /// [`Observer`] [`System`] that handles incoming [`KnownDataPacks`] packets.
    pub fn on_known_data_packs(
        trigger: Trigger<Recv<KnownDataPacks>>,
        mut writer: PacketWriter,
        packets: Local<RegistryDataPackets>,
    ) {
        let client = trigger.entity();

        let mut writer = writer.client(client);
        for packet in &**packets {
            writer.client().send_raw(packet.clone());
        }

        writer.send(&Finish);
    }

    /// [`Observer`] [`System`] that handles incoming [`AcknowledgeFinish`] packets.
    pub fn on_acknowledge_finish(
        trigger: Trigger<Recv<AcknowledgeFinish>>,
        mut writer: PacketWriter,
        mut commands: Commands,
    ) {
        let client = trigger.entity();

        writer.client(client).set_state(ProtocolState::Play);
        commands.entity(client).trigger(EnterPlay);
    }
}

/// [`Component`] that stores information about the client.
#[derive(Component, Clone, PartialEq, Debug)]
pub struct ClientInfo {
    /// The locale of the client, e.g. `en_US`.
    pub locale: String,
    /// The view distance of the client.
    pub view_distance: i8,
    /// The chat mode of the client.
    /// 0: enabled, 1: commands only, 2: hidden.
    pub chat_mode: i32,
    /// Whether chat colors are enabled.
    pub chat_colors: bool,
    /// The displayed skin parts of the client.
    pub displayed_skin_parts: u8,
    /// The main hand of the client.
    /// 0: left, 1: right.
    pub main_hand: i32,
    /// Whether text filtering is enabled.
    pub enable_text_filtering: bool,
    /// Whether server listings are allowed.
    pub allow_server_listings: bool,
}

impl From<&ClientInformation> for ClientInfo {
    fn from(info: &ClientInformation) -> Self {
        Self {
            locale: info.locale.clone(),
            view_distance: info.view_distance,
            chat_mode: info.chat_mode,
            chat_colors: info.chat_colors,
            displayed_skin_parts: info.displayed_skin_parts,
            main_hand: info.main_hand,
            enable_text_filtering: info.enable_text_filtering,
            allow_server_listings: info.allow_server_listings,
        }
    }
}

// TODO: replace these RegistryData packets with ones we've built ourselves,
// rather than ones replayed from vanilla.
#[derive(Deref, DerefMut)]
pub struct RegistryDataPackets(Vec<RawPacket>);

impl Default for RegistryDataPackets {
    fn default() -> Self {
        Self(vec![
            RawPacket::new(0x07, hex::decode("186D696E6563726166743A776F726C6467656E2F62696F6D6540126D696E6563726166743A6261646C616E647300176D696E6563726166743A62616D626F6F5F6A756E676C6500176D696E6563726166743A626173616C745F64656C746173000F6D696E6563726166743A626561636800166D696E6563726166743A62697263685F666F7265737400166D696E6563726166743A6368657272795F67726F766500146D696E6563726166743A636F6C645F6F6365616E00186D696E6563726166743A6372696D736F6E5F666F7265737400156D696E6563726166743A6461726B5F666F7265737400196D696E6563726166743A646565705F636F6C645F6F6365616E00136D696E6563726166743A646565705F6461726B001B6D696E6563726166743A646565705F66726F7A656E5F6F6365616E001D6D696E6563726166743A646565705F6C756B657761726D5F6F6365616E00146D696E6563726166743A646565705F6F6365616E00106D696E6563726166743A64657365727400196D696E6563726166743A6472697073746F6E655F636176657300156D696E6563726166743A656E645F62617272656E7300176D696E6563726166743A656E645F686967686C616E647300166D696E6563726166743A656E645F6D69646C616E647300196D696E6563726166743A65726F6465645F6261646C616E647300176D696E6563726166743A666C6F7765725F666F7265737400106D696E6563726166743A666F7265737400166D696E6563726166743A66726F7A656E5F6F6365616E00166D696E6563726166743A66726F7A656E5F7065616B7300166D696E6563726166743A66726F7A656E5F7269766572000F6D696E6563726166743A67726F766500146D696E6563726166743A6963655F7370696B657300166D696E6563726166743A6A61676765645F7065616B7300106D696E6563726166743A6A756E676C6500186D696E6563726166743A6C756B657761726D5F6F6365616E00146D696E6563726166743A6C7573685F636176657300186D696E6563726166743A6D616E67726F76655F7377616D7000106D696E6563726166743A6D6561646F7700196D696E6563726166743A6D757368726F6F6D5F6669656C647300176D696E6563726166743A6E65746865725F776173746573000F6D696E6563726166743A6F6365616E00216D696E6563726166743A6F6C645F67726F7774685F62697263685F666F72657374001F6D696E6563726166743A6F6C645F67726F7774685F70696E655F746169676100216D696E6563726166743A6F6C645F67726F7774685F7370727563655F746169676100106D696E6563726166743A706C61696E73000F6D696E6563726166743A726976657200116D696E6563726166743A736176616E6E6100196D696E6563726166743A736176616E6E615F706C6174656175001B6D696E6563726166743A736D616C6C5F656E645F69736C616E647300156D696E6563726166743A736E6F77795F626561636800166D696E6563726166743A736E6F77795F706C61696E7300166D696E6563726166743A736E6F77795F736C6F70657300156D696E6563726166743A736E6F77795F7461696761001A6D696E6563726166743A736F756C5F73616E645F76616C6C657900176D696E6563726166743A7370617273655F6A756E676C6500156D696E6563726166743A73746F6E795F7065616B7300156D696E6563726166743A73746F6E795F73686F7265001A6D696E6563726166743A73756E666C6F7765725F706C61696E73000F6D696E6563726166743A7377616D70000F6D696E6563726166743A746169676100116D696E6563726166743A7468655F656E6400126D696E6563726166743A7468655F766F696400146D696E6563726166743A7761726D5F6F6365616E00176D696E6563726166743A7761727065645F666F72657374001A6D696E6563726166743A77696E6473776570745F666F7265737400226D696E6563726166743A77696E6473776570745F67726176656C6C795F68696C6C7300196D696E6563726166743A77696E6473776570745F68696C6C73001B6D696E6563726166743A77696E6473776570745F736176616E6E6100196D696E6563726166743A776F6F6465645F6261646C616E647300").unwrap()),
            RawPacket::new(0x07, hex::decode("136D696E6563726166743A636861745F74797065070E6D696E6563726166743A6368617400176D696E6563726166743A656D6F74655F636F6D6D616E64001E6D696E6563726166743A6D73675F636F6D6D616E645F696E636F6D696E67001E6D696E6563726166743A6D73675F636F6D6D616E645F6F7574676F696E6700156D696E6563726166743A7361795F636F6D6D616E6400236D696E6563726166743A7465616D5F6D73675F636F6D6D616E645F696E636F6D696E6700236D696E6563726166743A7465616D5F6D73675F636F6D6D616E645F6F7574676F696E6700").unwrap()),
            RawPacket::new(0x07, hex::decode("166D696E6563726166743A7472696D5F7061747465726E120E6D696E6563726166743A626F6C74000F6D696E6563726166743A636F617374000E6D696E6563726166743A64756E65000D6D696E6563726166743A657965000E6D696E6563726166743A666C6F77000E6D696E6563726166743A686F737400106D696E6563726166743A726169736572000D6D696E6563726166743A72696200106D696E6563726166743A73656E74727900106D696E6563726166743A73686170657200116D696E6563726166743A73696C656E6365000F6D696E6563726166743A736E6F7574000F6D696E6563726166743A7370697265000E6D696E6563726166743A74696465000D6D696E6563726166743A766578000E6D696E6563726166743A7761726400136D696E6563726166743A77617966696E646572000E6D696E6563726166743A77696C6400").unwrap()),
            RawPacket::new(0x07, hex::decode("176D696E6563726166743A7472696D5F6D6174657269616C0A126D696E6563726166743A616D65746879737400106D696E6563726166743A636F7070657200116D696E6563726166743A6469616D6F6E6400116D696E6563726166743A656D6572616C64000E6D696E6563726166743A676F6C64000E6D696E6563726166743A69726F6E000F6D696E6563726166743A6C6170697300136D696E6563726166743A6E657468657269746500106D696E6563726166743A71756172747A00126D696E6563726166743A72656473746F6E6500").unwrap()),
            RawPacket::new(0x07, hex::decode("166D696E6563726166743A776F6C665F76617269616E74090F6D696E6563726166743A617368656E000F6D696E6563726166743A626C61636B00126D696E6563726166743A63686573746E7574000E6D696E6563726166743A70616C65000F6D696E6563726166743A7275737479000F6D696E6563726166743A736E6F777900116D696E6563726166743A73706F7474656400116D696E6563726166743A73747269706564000F6D696E6563726166743A776F6F647300").unwrap()),
            RawPacket::new(0x07, hex::decode("1A6D696E6563726166743A7061696E74696E675F76617269616E74320F6D696E6563726166743A616C62616E000F6D696E6563726166743A617A74656300106D696E6563726166743A617A7465633200126D696E6563726166743A6261636B7961726400116D696E6563726166743A6261726F717565000E6D696E6563726166743A626F6D6200116D696E6563726166743A626F757175657400176D696E6563726166743A6275726E696E675F736B756C6C000E6D696E6563726166743A6275737400126D696E6563726166743A636176656269726400126D696E6563726166743A6368616E67696E67000F6D696E6563726166743A636F74616E00116D696E6563726166743A636F757262657400116D696E6563726166743A6372656562657400156D696E6563726166743A646F6E6B65795F6B6F6E67000F6D696E6563726166743A656172746800116D696E6563726166743A656E64626F7373000E6D696E6563726166743A6665726E00126D696E6563726166743A666967687465727300116D696E6563726166743A66696E64696E67000E6D696E6563726166743A6669726500106D696E6563726166743A67726168616D00106D696E6563726166743A68756D626C65000F6D696E6563726166743A6B6562616200116D696E6563726166743A6C6F776D697374000F6D696E6563726166743A6D6174636800146D696E6563726166743A6D656469746174697665000D6D696E6563726166743A6F726200126D696E6563726166743A6F776C656D6F6E7300116D696E6563726166743A7061737361676500126D696E6563726166743A7069677363656E65000F6D696E6563726166743A706C616E7400116D696E6563726166743A706F696E746572000E6D696E6563726166743A706F6E64000E6D696E6563726166743A706F6F6C00166D696E6563726166743A707261697269655F72696465000D6D696E6563726166743A73656100126D696E6563726166743A736B656C65746F6E00196D696E6563726166743A736B756C6C5F616E645F726F736573000F6D696E6563726166743A737461676500146D696E6563726166743A73756E666C6F7765727300106D696E6563726166743A73756E736574000F6D696E6563726166743A746964657300126D696E6563726166743A756E7061636B6564000E6D696E6563726166743A766F696400126D696E6563726166743A77616E646572657200136D696E6563726166743A77617374656C616E64000F6D696E6563726166743A7761746572000E6D696E6563726166743A77696E6400106D696E6563726166743A77697468657200").unwrap()),
            RawPacket::new(0x07, hex::decode("186D696E6563726166743A64696D656E73696F6E5F7479706504136D696E6563726166743A6F766572776F726C6400196D696E6563726166743A6F766572776F726C645F636176657300116D696E6563726166743A7468655F656E6400146D696E6563726166743A7468655F6E657468657200").unwrap()),
            RawPacket::new(0x07, hex::decode("156D696E6563726166743A64616D6167655F747970652F0F6D696E6563726166743A6172726F77001B6D696E6563726166743A6261645F7265737061776E5F706F696E7400106D696E6563726166743A63616374757300126D696E6563726166743A63616D706669726500126D696E6563726166743A6372616D6D696E6700176D696E6563726166743A647261676F6E5F627265617468000F6D696E6563726166743A64726F776E00116D696E6563726166743A6472795F6F757400136D696E6563726166743A6578706C6F73696F6E000E6D696E6563726166743A66616C6C00176D696E6563726166743A66616C6C696E675F616E76696C00176D696E6563726166743A66616C6C696E675F626C6F636B001C6D696E6563726166743A66616C6C696E675F7374616C61637469746500126D696E6563726166743A6669726562616C6C00136D696E6563726166743A66697265776F726B7300176D696E6563726166743A666C795F696E746F5F77616C6C00106D696E6563726166743A667265657A6500116D696E6563726166743A67656E6572696300166D696E6563726166743A67656E657269635F6B696C6C00136D696E6563726166743A686F745F666C6F6F7200116D696E6563726166743A696E5F6669726500116D696E6563726166743A696E5F77616C6C00186D696E6563726166743A696E6469726563745F6D61676963000E6D696E6563726166743A6C61766100186D696E6563726166743A6C696768746E696E675F626F6C74000F6D696E6563726166743A6D6167696300146D696E6563726166743A6D6F625F61747461636B001D6D696E6563726166743A6D6F625F61747461636B5F6E6F5F616767726F00186D696E6563726166743A6D6F625F70726F6A656374696C6500116D696E6563726166743A6F6E5F6669726500166D696E6563726166743A6F75745F6F665F776F726C6400186D696E6563726166743A6F7574736964655F626F7264657200176D696E6563726166743A706C617965725F61747461636B001A6D696E6563726166743A706C617965725F6578706C6F73696F6E00146D696E6563726166743A736F6E69635F626F6F6D000E6D696E6563726166743A7370697400146D696E6563726166743A7374616C61676D69746500106D696E6563726166743A737461727665000F6D696E6563726166743A7374696E67001A6D696E6563726166743A73776565745F62657272795F6275736800106D696E6563726166743A74686F726E7300106D696E6563726166743A7468726F776E00116D696E6563726166743A74726964656E74001F6D696E6563726166743A756E617474726962757465645F6669726562616C6C00156D696E6563726166743A77696E645F63686172676500106D696E6563726166743A77697468657200166D696E6563726166743A7769746865725F736B756C6C00").unwrap()),
            RawPacket::new(0x07, hex::decode("186D696E6563726166743A62616E6E65725F7061747465726E2B0E6D696E6563726166743A6261736500106D696E6563726166743A626F7264657200106D696E6563726166743A627269636B7300106D696E6563726166743A636972636C6500116D696E6563726166743A63726565706572000F6D696E6563726166743A63726F737300166D696E6563726166743A6375726C795F626F7264657200176D696E6563726166743A646961676F6E616C5F6C65667400186D696E6563726166743A646961676F6E616C5F7269676874001A6D696E6563726166743A646961676F6E616C5F75705F6C656674001B6D696E6563726166743A646961676F6E616C5F75705F7269676874000E6D696E6563726166743A666C6F7700106D696E6563726166743A666C6F776572000F6D696E6563726166743A676C6F626500126D696E6563726166743A6772616469656E7400156D696E6563726166743A6772616469656E745F757000106D696E6563726166743A67757374657200196D696E6563726166743A68616C665F686F72697A6F6E74616C00206D696E6563726166743A68616C665F686F72697A6F6E74616C5F626F74746F6D00176D696E6563726166743A68616C665F766572746963616C001D6D696E6563726166743A68616C665F766572746963616C5F726967687400106D696E6563726166743A6D6F6A616E6700106D696E6563726166743A7069676C696E00116D696E6563726166743A72686F6D627573000F6D696E6563726166743A736B756C6C00176D696E6563726166743A736D616C6C5F73747269706573001C6D696E6563726166743A7371756172655F626F74746F6D5F6C656674001D6D696E6563726166743A7371756172655F626F74746F6D5F726967687400196D696E6563726166743A7371756172655F746F705F6C656674001A6D696E6563726166743A7371756172655F746F705F726967687400186D696E6563726166743A73747261696768745F63726F737300176D696E6563726166743A7374726970655F626F74746F6D00176D696E6563726166743A7374726970655F63656E74657200196D696E6563726166743A7374726970655F646F776E6C656674001A6D696E6563726166743A7374726970655F646F776E726967687400156D696E6563726166743A7374726970655F6C65667400176D696E6563726166743A7374726970655F6D6964646C6500166D696E6563726166743A7374726970655F726967687400146D696E6563726166743A7374726970655F746F7000196D696E6563726166743A747269616E676C655F626F74746F6D00166D696E6563726166743A747269616E676C655F746F70001A6D696E6563726166743A747269616E676C65735F626F74746F6D00176D696E6563726166743A747269616E676C65735F746F7000").unwrap()),
            RawPacket::new(0x07, hex::decode("156D696E6563726166743A656E6368616E746D656E742A176D696E6563726166743A617175615F616666696E697479001C6D696E6563726166743A62616E655F6F665F61727468726F706F647300176D696E6563726166743A62696E64696E675F6375727365001A6D696E6563726166743A626C6173745F70726F74656374696F6E00106D696E6563726166743A62726561636800146D696E6563726166743A6368616E6E656C696E6700116D696E6563726166743A64656E7369747900176D696E6563726166743A64657074685F7374726964657200146D696E6563726166743A656666696369656E637900196D696E6563726166743A666561746865725F66616C6C696E6700156D696E6563726166743A666972655F61737065637400196D696E6563726166743A666972655F70726F74656374696F6E000F6D696E6563726166743A666C616D6500116D696E6563726166743A666F7274756E6500166D696E6563726166743A66726F73745F77616C6B657200126D696E6563726166743A696D70616C696E6700126D696E6563726166743A696E66696E69747900136D696E6563726166743A6B6E6F636B6261636B00116D696E6563726166743A6C6F6F74696E6700116D696E6563726166743A6C6F79616C747900196D696E6563726166743A6C75636B5F6F665F7468655F736561000E6D696E6563726166743A6C75726500116D696E6563726166743A6D656E64696E6700136D696E6563726166743A6D756C746973686F7400126D696E6563726166743A7069657263696E67000F6D696E6563726166743A706F776572001F6D696E6563726166743A70726F6A656374696C655F70726F74656374696F6E00146D696E6563726166743A70726F74656374696F6E000F6D696E6563726166743A70756E636800166D696E6563726166743A717569636B5F63686172676500156D696E6563726166743A7265737069726174696F6E00116D696E6563726166743A7269707469646500136D696E6563726166743A73686172706E65737300146D696E6563726166743A73696C6B5F746F756368000F6D696E6563726166743A736D69746500146D696E6563726166743A736F756C5F737065656400176D696E6563726166743A7377656570696E675F6564676500156D696E6563726166743A73776966745F736E65616B00106D696E6563726166743A74686F726E7300146D696E6563726166743A756E627265616B696E6700196D696E6563726166743A76616E697368696E675F637572736500146D696E6563726166743A77696E645F627572737400").unwrap()),
            RawPacket::new(0x07, hex::decode("166D696E6563726166743A6A756B65626F785F736F6E67130C6D696E6563726166743A3131000C6D696E6563726166743A3133000B6D696E6563726166743A3500106D696E6563726166743A626C6F636B73000D6D696E6563726166743A636174000F6D696E6563726166743A636869727000116D696E6563726166743A63726561746F72001B6D696E6563726166743A63726561746F725F6D757369635F626F78000D6D696E6563726166743A666172000E6D696E6563726166743A6D616C6C00116D696E6563726166743A6D656C6C6F686900136D696E6563726166743A6F746865727369646500116D696E6563726166743A7069677374657000136D696E6563726166743A707265636970696365000F6D696E6563726166743A72656C6963000E6D696E6563726166743A7374616C000F6D696E6563726166743A7374726164000E6D696E6563726166743A77616974000E6D696E6563726166743A7761726400").unwrap()),
        ])
    }
}
