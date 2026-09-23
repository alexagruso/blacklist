use avian2d::prelude::*;
use bevy::prelude::*;

/// Marker for any entity that responds to character controller messages
#[derive(Component)]
#[require(Position, Rotation, CustomPositionIntegration)]
pub struct Character;

pub enum MovementType {
    LinearVelocity(Vec2),
    AngularVelocity(f32),
}

#[derive(Message)]
pub struct CharacterMovement {
    entity: Entity,
    movement_type: MovementType,
}

impl CharacterMovement {
    pub fn linear_velocity(entity: Entity, offset: Vec2) -> Self {
        Self {
            entity,
            movement_type: MovementType::LinearVelocity(offset),
        }
    }

    pub fn angular_velocity(entity: Entity, angle: f32) -> Self {
        Self {
            entity,
            movement_type: MovementType::AngularVelocity(angle),
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
            MovementType::LinearVelocity(offset) => **linear_velocity = offset,
            MovementType::AngularVelocity(angle) => **angular_velocity = angle,
        }
    }
}

// fn rotate_shape_cast(
//     collider: &Collider,
//     position: Vec2,
//     rotation: f32,
//     angular_velocity: f32,
//     // _rotate_shape_cast_config: RotateShapeCastConfig,
//     spatial_query: &SpatialQuery,
//     filter: &SpatialQueryFilter,
// ) -> f32 {
//     let mut start = rotation;
//     let mut offset = angular_velocity;
//     const MAX_ITERATIONS: u32 = 20;

//     for _ in 0..MAX_ITERATIONS {
//         if !spatial_query
//             .shape_intersections(&collider, position, start + offset, filter)
//             .is_empty()
//         {
//             offset = (start + offset) / 2.0;
//         } else {
//             start = (start + offset) / 2.0;
//         }
//     }

//     offset
// }

fn character_collision_response(
    time: Res<Time<Fixed>>,
    move_and_slide: MoveAndSlide,
    _spatial_query: SpatialQuery,
    mut characters: Query<
        (
            Entity,
            &Collider,
            &Position,
            &Rotation,
            &LinearVelocity,
            &AngularVelocity,
        ),
        With<Character>,
    >,
    mut commands: Commands,
) {
    for (entity, collider, position, rotation, linear_velocity, _angular_velocity) in
        &mut characters
    {
        let velocity = **linear_velocity;
        if !velocity.is_finite() {
            panic!("Entity {entity} has infinite or NaN velocity");
        }

        let output = move_and_slide.move_and_slide(
            &collider,
            position.0,
            rotation.as_radians(),
            velocity,
            time.delta(),
            &MoveAndSlideConfig::default(),
            &SpatialQueryFilter::from_excluded_entities([entity]),
            |_hit| MoveAndSlideHitResponse::Accept,
        );

        // Overwriting the component avoids having to mutably borrow in the character query
        commands.entity(entity).insert((
            Position(output.position),
            LinearVelocity(output.projected_velocity),
        ));

        // TODO: implement in place rotation collision
        // if **angular_velocity != 0.0 {
        //     let max_angle = rotate_shape_cast(
        //         &collider,
        //         **position,
        //         rotation.as_radians(),
        //         **angular_velocity,
        //         &spatial_query,
        //         &SpatialQueryFilter::from_excluded_entities([entity]),
        //     );

        //     let new_rotation =
        //         Rotation::radians(rotation.as_radians() + max_angle * time.delta_secs());
        //     commands.entity(entity).insert(new_rotation);
        // }
    }
}
