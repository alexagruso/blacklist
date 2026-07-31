use avian2d::math::Scalar;
use bevy::{app::PluginGroupBuilder, log::LogPlugin, prelude::*};

fn main() {
    App::new()
        // TODO: at some point I'd like to remove the default plugin group and manually add all necessary plugins.
        .add_plugins(DefaultPlugins.build().disable::<LogPlugin>())
        .add_plugins((
            GamePlugins,
            #[cfg(debug_assertions)]
            GameDebugPlugin,
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
        Mesh2d(meshes.add(Rectangle::new(50.0, 50.0))),
        MeshMaterial2d(materials.add(Color::srgb(0.25, 0.9, 0.35))),
    ));
}

#[derive(Component)]
#[require(Transform)]
struct Player {
    movement_speed: f32,
    rotation_speed: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            movement_speed: 150.0,
            rotation_speed: 180.0,
        }
    }
}

struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, player_input);
    }
}

fn player_input(
    time: Res<Time<Fixed>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    player: Query<(&Player, Entity)>,
    mut character_movement: MessageWriter<CharacterMovement>,
) {
    let (player, player_entity) = player.single().expect("multiple player entities exist");
    let delta_time = time.delta_secs();

    // We use integers here to avoid floating point comparison issues below
    let mut velocity = IVec2::ZERO;
    let mut angle = 0;

    if keyboard.pressed(KeyCode::KeyA) {
        velocity.x -= 1;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        velocity.x += 1;
    }

    if keyboard.pressed(KeyCode::KeyW) {
        velocity.y += 1;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        velocity.y -= 1;
    }

    if velocity != IVec2::ZERO {
        let velocity = vec2(velocity.x as f32, velocity.y as f32).normalize_or_zero()
            * player.movement_speed
            * delta_time;
        character_movement.write(CharacterMovement::translation(player_entity, velocity));
    }

    if keyboard.pressed(KeyCode::KeyQ) {
        angle -= 1;
    }
    if keyboard.pressed(KeyCode::KeyE) {
        angle += 1;
    }

    if angle != 0 {
        let angle = (angle as Scalar).to_radians() * player.rotation_speed * delta_time;
        character_movement.write(CharacterMovement::rotation(player_entity, angle));
    }
}

/// Marker for any entity that responds to character controller messages
#[derive(Component)]
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
    fn translation(entity: Entity, vec: Vec2) -> Self {
        Self {
            entity,
            movement_type: MovementType::Translation(vec),
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
    mut characters: Query<&mut Transform, With<Character>>,
    mut character_movement: MessageReader<CharacterMovement>,
) {
    for message in character_movement.read() {
        let mut transform = characters.get_mut(message.entity).expect(&format!(
            "character movement message sent for non-existent entity.\nID: {}",
            message.entity
        ));

        match message.movement_type {
            MovementType::Translation(vec) => transform.translation += vec.extend(0.0),
            MovementType::Rotation(angle) => transform.rotate_z(angle),
        }
    }
}

struct GameDebugPlugin;

impl Plugin for GameDebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, exit_on_esc);
    }
}

fn exit_on_esc(keyboard: Res<ButtonInput<KeyCode>>, mut app_exit: MessageWriter<AppExit>) {
    if keyboard.just_pressed(KeyCode::Escape) {
        app_exit.write(AppExit::Success);
    }
}
