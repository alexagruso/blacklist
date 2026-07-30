use bevy::{log::LogPlugin, prelude::*};

fn main() {
    App::new()
        // TODO: at some point I'd like to remove the default plugin group and add manually add all necessary plugins.
        .add_plugins(DefaultPlugins.build().disable::<LogPlugin>())
        .add_plugins((
            GamePlugin,
            #[cfg(debug_assertions)]
            GameDebugPlugin,
        ))
        .run();
}

struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_camera, spawn_player));
    }
}

#[derive(Component)]
#[require(Transform)]
struct Player;

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Player,
        Mesh2d(meshes.add(Rectangle::new(50.0, 50.0))),
        MeshMaterial2d(materials.add(Color::srgb(0.25, 0.9, 0.35))),
    ));
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
