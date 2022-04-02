//! Provides functionality for the [`server::EntityMetadata`] packet.

use std::collections::HashMap;

use bevy::ecs::query::WorldQuery;
use bevy::prelude::*;

use minecrevy_net::socket::{Play, Socket};
use minecrevy_protocol_latest::types::MetadataValue;
use minecrevy_protocol_latest::{server, types};
use minecrevy_util::MainHand;

use crate::entity::living::*;
use crate::entity::player::*;
use crate::entity::*;

/// See [`EntityBundle`] for documentation.
#[derive(WorldQuery)]
pub struct EntityMetadataQuery<'w> {
    pub flags: Option<&'w EntityFlags>,
    pub fire: Option<&'w Fire>,
    pub air_rem: Option<&'w AirRemaining>,
    pub cname: Option<&'w CustomName>,
    pub silent: Option<&'w Silent>,
    pub gravity: Option<&'w Gravity>,
    pub pose: Option<&'w Pose>,
    pub frozen: Option<&'w Frozen>,
}

/// See [`EntityBundle`] for documentation.
pub type EntityMetadataFilter = Or<(
    Changed<EntityFlags>,
    Changed<Fire>,
    Changed<AirRemaining>,
    Changed<CustomName>,
    Changed<Pose>,
    Changed<Frozen>,
)>;

/// See [`LivingEntityBundle`] for documentation.
#[derive(WorldQuery)]
pub struct LivingEntityMetadataQuery<'w> {
    pub health: Option<&'w Health>,
    pub arrows: Option<&'w Arrows>,
    pub stingers: Option<&'w Stingers>,
    pub sleeping: Option<&'w Sleeping>,
}

/// See [`LivingEntityBundle`] for documentation.
pub type LivingEntityMetadataFilter = Or<(
    Changed<Health>,
    Changed<Arrows>,
    Changed<Stingers>,
    Changed<Sleeping>,
)>;

/// See [`PlayerBundle`] for documentation.
#[derive(WorldQuery)]
pub struct PlayerMetadataQuery<'w> {
    pub absorption: Option<&'w AbsorptionHearts>,
    pub kills: Option<&'w Kills>,
    pub skin_parts: Option<&'w DisplayedSkinParts>,
    pub main_hand: Option<&'w MainHand>,
    pub shoulder: Option<&'w ShoulderEntities>,
}

/// See [`PlayerBundle`] for documentation.
pub type PlayerMetadataFilter = Or<(
    Changed<AbsorptionHearts>,
    Changed<Kills>,
    Changed<DisplayedSkinParts>,
    Changed<MainHand>,
    Changed<ShoulderEntities>,
)>;

fn update_player_metadata(
    mut clients: Query<(Socket<Play>, &VisibleEntities)>,
    players: Query<
        (
            &NetworkId,
            EntityMetadataQuery,
            LivingEntityMetadataQuery,
            PlayerMetadataQuery,
        ),
        Or<(
            EntityMetadataFilter,
            LivingEntityMetadataFilter,
            PlayerMetadataFilter,
        )>,
    >,
) {
    for (id, entity, living, player) in players.iter() {
        // Prepare a map for storing collected metadata.
        let mut metadata = HashMap::<u8, MetadataValue>::new();
        let mut push = |idx: u8, v: MetadataValue| {
            metadata.insert(idx, v);
        };

        // Collect the metadata from the components.
        entity.collect_metadata(&mut push);
        living.collect_metadata(&mut push);
        player.collect_metadata(&mut push);

        // Build the packet.
        let packet = server::EntityMetadata {
            entity_id: id.val(),
            metadata: types::Metadata(metadata),
        };

        // Send the packet off to players that can see this player.
        for (mut socket, visible) in clients.iter_mut() {
            if visible.0.contains(id) {
                socket.send(packet.clone());
            }
        }
    }
}

trait CollectMetadata {
    fn collect_metadata<F: FnMut(u8, MetadataValue)>(self, push: F);
}

impl CollectMetadata for (Option<&EntityFlags>, Option<&Fire>) {
    fn collect_metadata<F: FnMut(u8, MetadataValue)>(self, mut push: F) {
        let mut value: u8 = 0;
        if let Some(fire) = self.1 {
            if fire.0.finished() {
                value |= 1 << 0
            }
        }
        if let Some(flags) = self.0 {
            if flags.crouching {
                value |= 1 << 1
            }
            //                 { value |= 1 << 3 } // unused, formerly riding flag
            if flags.sprinting {
                value |= 1 << 3
            }
            if flags.swimming {
                value |= 1 << 4
            }
            if flags.invisible {
                value |= 1 << 5
            }
            if flags.glowing {
                value |= 1 << 6
            }
            if flags.elytra_flying {
                value |= 1 << 7
            }
        }
        if value != 0 {
            push(0, MetadataValue::Byte(value as i8));
        }
    }
}

impl<'w> CollectMetadata for EntityMetadataQueryItem<'w> {
    fn collect_metadata<F: FnMut(u8, MetadataValue)>(self, mut push: F) {
        (self.flags, self.fire).collect_metadata(&mut push);
        if let Some(_) = self.air_rem {
            // TODO: how to convert a Timer into a tick amount?
        }
        if let Some(cname) = self.cname {
            push(2, MetadataValue::OptText(cname.name.clone()));
            push(3, MetadataValue::Bool(cname.visible));
        }
        if let Some(Silent(silent)) = self.silent {
            push(4, MetadataValue::Bool(*silent));
        }
        if let Some(Gravity(gravity)) = self.gravity {
            push(5, MetadataValue::Bool(!*gravity));
        }
        if let Some(&pose) = self.pose {
            push(6, MetadataValue::Pose(pose.into()));
        }
        if let Some(_) = self.frozen {
            // TODO: same reason as above
        }
    }
}

impl<'w> CollectMetadata for LivingEntityMetadataQueryItem<'w> {
    fn collect_metadata<F: FnMut(u8, MetadataValue)>(self, mut push: F) {
        if let Some(health) = self.health {
            push(9, MetadataValue::Float(health.value));
        }
        // TODO: potion metadata
        if let Some(Arrows(arrows)) = self.arrows {
            push(12, MetadataValue::VarInt(*arrows));
        }
        if let Some(Stingers(stingers)) = self.stingers {
            push(13, MetadataValue::VarInt(*stingers));
        }
        if let Some(Sleeping(pos)) = self.sleeping {
            push(14, MetadataValue::OptPosition(*pos));
        }
    }
}

impl CollectMetadata for &DisplayedSkinParts {
    fn collect_metadata<F: FnMut(u8, MetadataValue)>(self, mut push: F) {
        let mut value: u8 = 0;
        if self.cape {
            value |= 1 << 0
        }
        if self.jacket {
            value |= 1 << 1
        }
        if self.left_sleeve {
            value |= 1 << 2
        }
        if self.right_sleeve {
            value |= 1 << 3
        }
        if self.left_pants {
            value |= 1 << 4
        }
        if self.right_pants {
            value |= 1 << 5
        }
        if self.hat {
            value |= 1 << 6
        }
        push(17, MetadataValue::Byte(value as i8))
    }
}

impl<'w> CollectMetadata for PlayerMetadataQueryItem<'w> {
    fn collect_metadata<F: FnMut(u8, MetadataValue)>(self, mut push: F) {
        if let Some(AbsorptionHearts(absorption)) = self.absorption {
            push(15, MetadataValue::Float(*absorption));
        }
        if let Some(Kills(kills)) = self.kills {
            push(16, MetadataValue::VarInt(*kills));
        }
        if let Some(parts) = self.skin_parts {
            parts.collect_metadata(&mut push);
        }
        if let Some(hand) = self.main_hand {
            push(
                18,
                MetadataValue::Byte(match hand {
                    MainHand::Left => 0,
                    MainHand::Right => 1,
                }),
            );
        }
        if let Some(shoulder) = self.shoulder {
            if let Some(left) = &shoulder.left {
                push(19, MetadataValue::Nbt(left.clone()));
            }
            if let Some(right) = &shoulder.right {
                push(20, MetadataValue::Nbt(right.clone()));
            }
        }
    }
}
