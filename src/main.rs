use avian2d::{math::Scalar, prelude::*};
use bevy::{app::PluginGroupBuilder, log::LogPlugin, prelude::*};

fn main() {
    App::new()
        // TODO: at some point I'd like to remove the default plugin group and manually add all necessary plugins.
        .add_plugins(DefaultPlugins.build().disable::<LogPlugin>())
        .add_plugins((
            // TODO: figure out whether another value would be ideal
            PhysicsPlugins::default().with_length_unit(20.0),
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
        app.add_systems(Startup, (spawn_camera, spawn_player));
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
        Mesh2d(meshes.add(Rectangle::new(50.0, 50.0))),
        MeshMaterial2d(materials.add(Color::srgb(0.25, 0.9, 0.35))),
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
    let mut angle = 0.0;

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
        angle -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyE) {
        angle += 1.0;
    }

    let angle = (angle as Scalar).to_radians() * player.rotation_speed;
    character_movements.write(CharacterMovement::rotation(player_entity, angle));
}

/// Marker for any entity that responds to character controller messages
#[derive(Component)]
#[require(Transform, LinearVelocity, AngularVelocity)]
struct Character;

enum MovementType {
    Translation(Vec2),
    Rotation(Scalar),
}

#[derive(Message)]
struct CharacterMovement {
    entity: Entity,
    movement_type: MovementType,
}

impl CharacterMovement {
    fn translation(entity: Entity, velocity: Vec2) -> Self {
        Self {
            entity,
            movement_type: MovementType::Translation(velocity),
        }
    }

    fn rotation(entity: Entity, angle: Scalar) -> Self {
        Self {
            entity,
            movement_type: MovementType::Rotation(angle),
        }
    }
}

struct CharacterControllerPlugin;

impl Plugin for CharacterControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<CharacterMovement>()
            .add_systems(FixedUpdate, character_movement);
    }
}

fn character_movement(
    mut characters: Query<(&mut LinearVelocity, &mut AngularVelocity), With<Character>>,
    mut character_movements: MessageReader<CharacterMovement>,
) {
    for message in character_movements.read() {
        let (mut linear_velocity, mut angular_velocity) =
            characters.get_mut(message.entity).expect(&format!(
                "character movement message sent for non-existent entity.\nID: {}",
                message.entity
            ));

        match message.movement_type {
            MovementType::Translation(velocity) => {
                **linear_velocity = velocity;
            }
            MovementType::Rotation(angle) => **angular_velocity = angle,
        }
    }
}

struct GameDebugPlugins;

impl PluginGroup for GameDebugPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(SystemDebugPlugin)
            .add(GizmoDebugPlugin)
    }
}

struct SystemDebugPlugin;

impl Plugin for SystemDebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, exit_on_esc);
    }
}

fn exit_on_esc(keyboard: Res<ButtonInput<KeyCode>>, mut app_exit: MessageWriter<AppExit>) {
    if keyboard.just_pressed(KeyCode::Escape) {
        app_exit.write(AppExit::Success);
    }
}

struct GizmoDebugPlugin;

impl Plugin for GizmoDebugPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DebugGridDimensions {
            spacing: 100.0,
            rows: 15,
            cols: 15,
            point_radius: 5.0,
        })
        .add_systems(Update, draw_debug_grid);
    }
}

#[derive(Resource)]
struct DebugGridDimensions {
    spacing: f32,
    rows: u32,
    cols: u32,
    point_radius: f32,
}

fn draw_debug_grid(debug_grid: Res<DebugGridDimensions>, mut gizmos: Gizmos) {
    let horizontal_offset = (debug_grid.cols - 1) as f32 * debug_grid.spacing / 2.0;
    let vertical_offset = (debug_grid.rows - 1) as f32 * debug_grid.spacing / 2.0;

    for row in 0..debug_grid.rows {
        for col in 0..debug_grid.cols {
            let position = vec2(
                col as f32 * debug_grid.spacing - horizontal_offset,
                row as f32 * debug_grid.spacing - vertical_offset,
            );
            gizmos.circle_2d(
                position,
                debug_grid.point_radius,
                Color::srgb(0.9, 0.4, 0.2),
            );
        }
    }
}
