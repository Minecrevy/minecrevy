use bevy::prelude::*;
use minecrevy_auth::Profile;

use minecrevy_ecs::label::Networking;
use minecrevy_net::socket::{Play, Socket};
use minecrevy_protocol_latest::{client, server};
use minecrevy_text::{Style, Text, TextPosition};

use crate::entity::player::{Message, Messages, Titles};

/// Handles outbound [`Messages`] and [`Titles`] sent to clients.
pub struct TextPlugin;

impl Plugin for TextPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(Self::receive_messages.label(Networking));
        app.add_system(Self::send_texts.label(Networking));
    }
}

impl TextPlugin {
    fn receive_messages(mut players: Query<(Socket<Play>, &Profile)>, mut receivers: Query<&mut Messages>) {
        let mut messages = Vec::new();

        for (mut socket, profile) in players.iter_mut() {
            if let Some(packet) = socket.recv::<client::ChatMessage>() {
                let message = Text::string(format!("<{}> ", profile.name), Style::empty())
                    .with_child(Text::string(packet.message, Style::empty()));

                messages.push(Message::chat_from(message, profile.id()));
            }
        }

        for mut receiver in receivers.iter_mut() {
            receiver.extend(messages.clone().into_iter());
        }
    }

    fn send_texts(mut players: Query<(Socket<Play>, Option<&mut Messages>, Option<&mut Titles>), Or<(Changed<Messages>, Changed<Titles>)>>) {
        for (mut socket, messages, titles) in players.iter_mut() {
            if let Some(mut messages) = messages {
                for message in messages.drain() {
                    if let TextPosition::ActionBar = message.position {
                        socket.send(server::ActionBarMessage(message.message));
                    } else {
                        socket.send(server::ChatMessage {
                            message: message.message,
                            position: message.position,
                            sender: message.sender,
                        });
                    }
                }
            }

            if let Some(mut titles) = titles {
                if titles.clear {
                    // TODO: don't always reset?
                    socket.send(server::ClearTitles { reset: true });
                } else {
                    for title in titles.drain() {
                        if let Some(title) = title.title {
                            socket.send(server::Title(title));
                        } else if let Some(subtitle) = title.subtitle {
                            socket.send(server::SubTitle(subtitle));
                        } else if let Some(times) = title.times {
                            socket.send(server::TitleTimes {
                                fade_in: times.fade_in.into(),
                                stay: times.stay.into(),
                                fade_out: times.fade_out.into(),
                            });
                        }
                    }
                }
            }
        }
    }
}
