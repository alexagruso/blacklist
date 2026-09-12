use avian2d::prelude::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct Obstacle;

#[derive(Resource)]
pub struct ObstacleAssets {
    mesh: Handle<Mesh>,
    material: Handle<ColorMaterial>,
}

// TODO: consider making a general `IntoBundle` trait
impl ObstacleAssets {
    pub fn into_bundle(&self, position: Vec2, size: Vec2, rotation: f32) -> impl Bundle {
        (
            Obstacle,
            RigidBody::Static,
            Collider::rectangle(1.0, 1.0),
            Transform::from_translation(position.extend(0.0))
                .with_scale(size.extend(1.0))
                .with_rotation(Quat::from_rotation_z(rotation.to_radians())),
            Mesh2d(self.mesh.clone()),
            MeshMaterial2d(self.material.clone()),
        )
    }
}

pub struct ObstaclePlugin;

impl Plugin for ObstaclePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, obstacle_setup);
    }
}

fn obstacle_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.insert_resource(ObstacleAssets {
        mesh: meshes.add(Rectangle::new(1.0, 1.0)),
        material: materials.add(Color::srgb(0.1, 0.4, 0.3)),
    });
}
