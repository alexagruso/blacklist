use avian2d::prelude::*;
use bevy::{prelude::*, window::PrimaryWindow};

use crate::{
    MainCamera,
    character::{Character, CharacterMovement},
    player::aiming::{Aiming, AimingPlugin, AimingZoomConfig},
};

pub mod aiming;

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
            .add_systems(PreUpdate, (player_keyboard_input, player_mouse_rotation))
            .add_systems(Update, player_camera);

        app.init_resource::<AimingZoomConfig>();
        app.add_plugins(AimingPlugin);
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

fn player_keyboard_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    player: Query<(&Player, Entity)>,
    mut character_movements: MessageWriter<CharacterMovement>,
) {
    let (player, player_entity) = player
        .single()
        .expect("there should never be multiple player entities");
    let mut velocity = Vec2::ZERO;

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
    character_movements.write(CharacterMovement::linear_velocity(player_entity, velocity));
}

fn player_mouse_rotation(
    camera: Single<(&Camera, &GlobalTransform), With<MainCamera>>,
    window: Single<&Window, With<PrimaryWindow>>,
    player: Single<(&Transform, &mut Rotation), With<Player>>,
    // TODO: make this write a movement message
    // mut character_movements: MessageWriter<CharacterMovement>,
) {
    // TODO: extract this into a general cursor world position
    let cursor_position = match window.cursor_position() {
        Some(position) => position,
        None => return,
    };
    let (camera, camera_transform) = camera.into_inner();
    let cursor_position = match camera.viewport_to_world_2d(camera_transform, cursor_position) {
        Ok(position) => position,
        _ => return,
    };

    let (player_transform, mut player_rotation) = player.into_inner();
    let cursor_direction = cursor_position - player_transform.translation.xy();
    let cursor_angle = cursor_direction.y.atan2(cursor_direction.x);
    *player_rotation = Rotation::radians(cursor_angle);
}

/// Performs the following modifications to the camera:
///
/// - Copies the player's translation to the camera's translation
/// - Applies the aiming effect if the player is aiming
fn player_camera(
    zoom_config: Res<AimingZoomConfig>,
    time: Res<Time>,
    player: Single<(&Transform, Has<Aiming>), With<Player>>,
    camera: Single<(&mut Transform, &mut Projection), (With<MainCamera>, Without<Player>)>,
) {
    let (player_transform, is_aiming) = player.into_inner();
    let (mut camera_transform, mut projection) = camera.into_inner();

    camera_transform.translation = player_transform.translation;

    let Projection::Orthographic(projection) = projection.as_mut() else {
        return;
    };
    let current_scale = projection.scale;
    let delta = time.delta_secs();
    if is_aiming {
        projection.scale = (current_scale
            - delta * (1.0 / zoom_config.in_time_secs) * zoom_config.scale_modifier)
            .max(1.0 - zoom_config.scale_modifier);
    } else {
        projection.scale = (current_scale
            + delta * (1.0 / zoom_config.out_time_secs) * zoom_config.scale_modifier)
            .min(1.0);
    }
}
