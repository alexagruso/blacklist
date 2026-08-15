use avian2d::prelude::*;
use bevy::{math::InvalidDirectionError, prelude::*};

/// Marker for any entity that responds to character controller messages
#[derive(Component)]
#[require(Transform, LinearVelocity, AngularVelocity)]
pub struct Character;

pub enum MovementType {
    Translation(Vec2),
    Rotation(f32),
}

#[derive(Message)]
pub struct CharacterMovement {
    entity: Entity,
    movement_type: MovementType,
}

impl CharacterMovement {
    pub fn translation(entity: Entity, offset: Vec2) -> Self {
        Self {
            entity,
            movement_type: MovementType::Translation(offset),
        }
    }

    pub fn rotation(entity: Entity, angle: f32) -> Self {
        Self {
            entity,
            movement_type: MovementType::Rotation(angle),
        }
    }
}

pub struct CharacterControllerPlugin;

impl Plugin for CharacterControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<CharacterMovement>().add_systems(
            FixedUpdate,
            (character_movement, character_collision_response).chain(),
        );
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
            MovementType::Translation(offset) => **linear_velocity = offset,
            MovementType::Rotation(angle) => **angular_velocity = angle,
        }
    }
}

fn character_collision_response(
    spatial_query: SpatialQuery,
    characters: Query<(Entity, &Collider, &Transform), With<Character>>,
    mut gizmos: Gizmos,
) {
    const GRID_RADIUS: i32 = 3;
    const GRID_SPACING: f32 = 15.0;

    for (entity, collider, transform) in &characters {
        let position = transform.translation.xy();
        for col in -GRID_RADIUS..=GRID_RADIUS {
            for row in -GRID_RADIUS..=GRID_RADIUS {
                let point = position
                    + vec2(col as f32 * GRID_SPACING, row as f32 * GRID_SPACING).rotate(
                        Vec2::from_angle(transform.rotation.to_euler(EulerRot::XYZ).2),
                    );
                let color = if !spatial_query
                    .point_intersections(
                        point,
                        &SpatialQueryFilter::from_excluded_entities([entity]),
                    )
                    .is_empty()
                {
                    Color::srgb(0.0, 1.0, 0.0)
                } else {
                    Color::srgb(1.0, 0.0, 0.0)
                };

                gizmos.circle_2d(point, 2.0, color);
            }
        }
    }
}
