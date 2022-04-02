//! Miscellaneous utilities, currently generic bevy queries.

use bevy::ecs::query::WorldQuery;
use bevy::prelude::{ChangeTrackers, Component};

/// A [`WorldQuery`] for combining a component's current value with whether it's changed or not.
#[derive(WorldQuery)]
pub struct ChangeTracked<'w, T: Component> {
    /// The current value of the component.
    pub value: &'w T,
    /// The [`ChangeTrackers`] for the component.
    pub tracker: ChangeTrackers<T>,
}

impl<'w, T: Component> ChangeTrackedItem<'w, T> {
    /// See [`ChangeTrackers::is_changed`] for documentation.
    pub fn is_changed(&self) -> bool {
        self.tracker.is_changed()
    }

    /// See [`ChangeTrackers::is_added`] for documentation.
    pub fn is_added(&self) -> bool {
        self.tracker.is_added()
    }
}
