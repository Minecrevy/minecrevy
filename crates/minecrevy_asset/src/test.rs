use std::any::TypeId;

pub trait Plugin {
    fn dependencies(&self) -> &[Dependency];
    fn provides(&self) -> &[Feature];
}

pub struct Dependency {
    /// The plugin or feature that this dependency is for.
    pub target: DependencyTarget,
    /// The type of dependency.
    pub dependency_type: DependencyType,
    /// The reason for the dependency.
    pub reason: Option<&'static str>,
}

pub enum DependencyType {
    /// The dependency is required for the plugin to function.
    Required,
    /// The dependency is optional, but the plugin will function without it.
    Optional,
    /// The dependency is incompatible with the plugin.
    Incompatible,
    /// The dependency is discouraged, but the plugin will function with it.
    Discouraged,
}

/// The target of a dependency, which can be a specific [`Plugin`] or a generic [`Feature`].
pub enum DependencyTarget {
    /// A specific plugin is depended on.
    Plugin(TypeId, &'static str),
    /// A generic feature is depended on.
    Feature(Feature),
}

impl DependencyTarget {
    pub fn plugin<T: Plugin + 'static>() -> Self {
        Self::Plugin(TypeId::of::<T>(), std::any::type_name::<T>())
    }

    pub fn feature(namespace: &'static str, path: &'static str) -> Self {
        Self::Feature(Feature { namespace, path })
    }
}

pub struct Feature {
    pub namespace: &'static str,
    pub path: &'static str,
}

pub const BEVY_RENDER: Feature = Feature {
    namespace: "bevy",
    path: "render",
};
