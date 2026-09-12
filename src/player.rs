use avian2d::prelude::*;
use bevy::prelude::*;

use crate::character::{Character, CharacterMovement};

#[derive(Component)]
pub struct Player {
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

#[derive(Resource)]
pub struct PlayerAssets {
    mesh: Handle<Mesh>,
    material: Handle<ColorMaterial>,
}

impl PlayerAssets {
    pub fn into_bundle(&self) -> impl Bundle {
        (
            Player::default(),
            Character,
            RigidBody::Kinematic,
            Collider::capsule(25.0, 50.0),
            Mesh2d(self.mesh.clone()),
            MeshMaterial2d(self.material.clone()),
        )
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, player_setup)
            .add_systems(PreUpdate, player_input);
    }
}

fn player_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.insert_resource(PlayerAssets {
        mesh: meshes.add(Capsule2d::new(25.0, 50.0)),
        material: materials.add(Color::srgb(0.25, 0.9, 0.35)),
    });
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
