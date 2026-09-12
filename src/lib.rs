use avian2d::prelude::*;
use bevy::{app::PluginGroupBuilder, prelude::*};

use crate::{
    character::{Character, CharacterControllerPlugin},
    player::{Player, PlayerPlugin},
};

pub mod character;
pub mod debug;
pub mod player;

pub struct GamePlugins;

impl PluginGroup for GamePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(SpawnPlugin)
            .add(PlayerPlugin)
            .add(CharacterControllerPlugin)
    }
}

struct SpawnPlugin;

impl Plugin for SpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_camera, spawn_player, spawn_obstacles));
    }
}

// TODO: there should be an active camera marker provided by bevy
#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct MainCamera;

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2d, MainCamera));
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Player::default(),
        Character,
        RigidBody::Kinematic,
        Collider::capsule(25.0, 50.0),
        Mesh2d(meshes.add(Capsule2d::new(25.0, 50.0))),
        MeshMaterial2d(materials.add(Color::srgb(0.25, 0.9, 0.35))),
    ));
}

#[derive(Component)]
struct Obstacle;

fn spawn_obstacles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Obstacle,
        Collider::rectangle(100.0, 500.0),
        Transform::from_translation(vec2(250.0, 0.0).extend(0.0))
            .with_rotation(Quat::from_rotation_z(15.0f32.to_radians())),
        Mesh2d(meshes.add(Rectangle::new(100.0, 500.0))),
        MeshMaterial2d(materials.add(Color::srgb(0.1, 0.4, 0.3))),
    ));
}
