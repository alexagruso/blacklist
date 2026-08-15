#![allow(dead_code)]

use avian2d::prelude::*;
use bevy::{app::PluginGroupBuilder, log::LogPlugin, prelude::*};

use blacklist::character::{Character, CharacterControllerPlugin, CharacterMovement};
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

struct GamePlugins;

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

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
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
        Collider::rectangle(50.0, 50.0),
        Mesh2d(meshes.add(Rectangle::new(50.0, 50.0))),
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

#[derive(Component)]
struct Player {
    pub movement_speed: f32,
    pub rotation_speed: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            movement_speed: 300.0,
            rotation_speed: 180.0,
        }
    }
}

struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, player_input);
    }
}

fn player_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    player: Query<(&Player, Entity)>,
    mut character_movements: MessageWriter<CharacterMovement>,
) {
    let (player, player_entity) = player
        .single()
        .expect("there should never be multiple player entities");
    let mut velocity = Vec2::ZERO;
    let mut angle: f32 = 0.0;

    if keyboard.pressed(KeyCode::KeyA) {
        velocity.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        velocity.x += 1.0;
    }

    if keyboard.pressed(KeyCode::KeyW) {
        velocity.y += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        velocity.y -= 1.0;
    }

    let velocity =
        vec2(velocity.x as f32, velocity.y as f32).normalize_or_zero() * player.movement_speed;
    character_movements.write(CharacterMovement::translation(player_entity, velocity));

    if keyboard.pressed(KeyCode::KeyQ) {
        angle += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyE) {
        angle -= 1.0;
    }

    let angle = angle.to_radians() * player.rotation_speed;
    character_movements.write(CharacterMovement::rotation(player_entity, angle));
}
