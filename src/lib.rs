use bevy::{app::PluginGroupBuilder, prelude::*};

use crate::{
    character::CharacterControllerPlugin,
    obstacle::{ObstacleAssets, ObstaclePlugin},
    player::{PlayerAssets, PlayerPlugin},
    precompute::CursorWorldPositionPlugin,
};

pub mod character;
pub mod debug;
pub mod obstacle;
pub mod player;

pub struct GamePlugins;

impl PluginGroup for GamePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(SpawnPlugin)
            .add(PlayerPlugin)
            .add(ObstaclePlugin)
            .add(CharacterControllerPlugin)
    }
}

// TODO: find a home for all of this world generation code even if I have to label it debug
struct SpawnPlugin;

impl Plugin for SpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_camera, spawn_player, spawn_obstacles));

        // NOTE: debug/testing stuff should be added here
        app.add_plugins(CursorWorldPositionPlugin);
    }
}

// TODO: there should be an active camera marker provided by bevy
#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct MainCamera;

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2d, MainCamera));
}

fn spawn_player(player_assets: Res<PlayerAssets>, mut commands: Commands) {
    commands.spawn(player_assets.into_bundle());
}

fn spawn_obstacles(obstacle_assets: Res<ObstacleAssets>, mut commands: Commands) {
    commands.spawn(obstacle_assets.into_bundle(vec2(250.0, 0.0), vec2(100.0, 500.0), 15.0));
    commands.spawn(obstacle_assets.into_bundle(vec2(-250.0, 0.0), vec2(100.0, 500.0), -15.0));

    for i in 0..10 {
        commands.spawn(obstacle_assets.into_bundle(
            vec2(0.0, 100.0 + i as f32 * 25.0),
            vec2(250.0, 10.0),
            i as f32 * 18.0,
        ));
    }

    commands.spawn(obstacle_assets.into_bundle(vec2(-50.0, -100.0), vec2(20.0, 100.0), 0.0));
    commands.spawn(obstacle_assets.into_bundle(vec2(50.0, -100.0), vec2(20.0, 100.0), 0.0));
}

/// Various plugins related to computations that are used often but only need to be performed once per frame
pub mod precompute {
    use bevy::{prelude::*, window::PrimaryWindow};

    use crate::MainCamera;

    /// Contains `Some(Vec2)` if the cursor is contained in the window, and `None` otherwise.
    #[derive(Resource, Default, Deref, DerefMut)]
    pub struct CursorWorldPosition(Option<Vec2>);

    pub struct CursorWorldPositionPlugin;

    impl Plugin for CursorWorldPositionPlugin {
        fn build(&self, app: &mut App) {
            app.init_resource::<CursorWorldPosition>();
            app.add_systems(PreUpdate, update_cursor_world_position);
        }
    }

    fn update_cursor_world_position(
        camera: Single<(&Camera, &GlobalTransform), With<MainCamera>>,
        window: Single<&Window, With<PrimaryWindow>>,
        mut cursor_world_position: ResMut<CursorWorldPosition>,
    ) {
        **cursor_world_position = None;

        let window_position = match window.cursor_position() {
            Some(position) => position,
            None => return,
        };

        let (camera, camera_transform) = camera.into_inner();
        let world_position = match camera.viewport_to_world_2d(camera_transform, window_position) {
            Ok(position) => position,
            _ => return,
        };

        **cursor_world_position = Some(world_position);
    }
}
