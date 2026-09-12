use avian2d::prelude::*;
use bevy::{log::LogPlugin, prelude::*};

use blacklist::GamePlugins;
#[cfg(debug_assertions)]
use blacklist::debug::GameDebugPlugins;

fn main() {
    App::new()
        // TODO: at some point I'd like to remove the default plugin group and manually add all necessary plugins.
        .add_plugins(DefaultPlugins.build().disable::<LogPlugin>())
        .add_plugins((
            // Most character entities (player, enemies, etc) are about 100 units in size, so we use 100 units per meter for physics
            PhysicsPlugins::default().with_length_unit(100.0),
            #[cfg(debug_assertions)]
            PhysicsDebugPlugin,
        ))
        .add_plugins((
            GamePlugins,
            #[cfg(debug_assertions)]
            GameDebugPlugins,
        ))
        .run();
}
