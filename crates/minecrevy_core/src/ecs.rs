use std::marker::PhantomData;

use bevy::{
    ecs::system::Command,
    prelude::{Commands, Resource, World},
};

pub trait CommandExt {
    fn replace_resource<A: Resource, B: Resource, F: Send + Sync + 'static + FnOnce(A) -> B>(
        &mut self,
        replace: F,
    );
}

impl CommandExt for Commands<'_, '_> {
    fn replace_resource<A: Resource, B: Resource, F: Send + Sync + 'static + FnOnce(A) -> B>(
        &mut self,
        replace: F,
    ) {
        self.add(ReplaceResource {
            replace,
            _resources: PhantomData,
        });
    }
}

pub struct ReplaceResource<A, B, F> {
    replace: F,
    _resources: PhantomData<fn(A) -> B>,
}

impl<A: Resource, B: Resource, F: Send + Sync + 'static + FnOnce(A) -> B> Command
    for ReplaceResource<A, B, F>
{
    fn write(self, world: &mut World) {
        let Some(old) = world.remove_resource::<A>() else {
            panic!(
                "Requested resource {} does not exist in the `World`.
                Did you forget to add it using `app.insert_resource` / `app.init_resource`?",
                std::any::type_name::<A>()
            );
        };

        let new = (self.replace)(old);
        world.insert_resource(new);
    }
}
