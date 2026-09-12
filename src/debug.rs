use bevy::{
    app::PluginGroupBuilder,
    camera::Projection::Orthographic,
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
};

use crate::MainCamera;

pub struct GameDebugPlugins;

impl PluginGroup for GameDebugPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(SystemDebugPlugin)
            .add(GizmoDebugPlugin)
            .add(WorldDebugPlugin)
    }
}

pub struct SystemDebugPlugin;

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

pub struct GizmoDebugPlugin;

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

struct WorldDebugPlugin;

impl Plugin for WorldDebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, camera_zoom_scroll)
            .init_resource::<CameraZoomConfig>();
    }
}

#[derive(Resource)]
struct CameraZoomConfig {
    sensitivity: f32,
}

impl Default for CameraZoomConfig {
    fn default() -> Self {
        Self { sensitivity: 0.015 }
    }
}

fn camera_zoom_scroll(
    camera_zoom_config: Res<CameraZoomConfig>,
    mut mouse_scroll: MessageReader<MouseWheel>,
    mut camera_projection: Query<&mut Projection, With<MainCamera>>,
) {
    let mut projection = camera_projection
        .single_mut()
        .expect("There should always be an active camera");

    for scroll in mouse_scroll.read() {
        if let MouseScrollUnit::Line = scroll.unit
            && let Orthographic(projection) = &mut *projection
        {
            projection.scale -= scroll.y * camera_zoom_config.sensitivity;
        }
    }
}
