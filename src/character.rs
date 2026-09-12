use avian2d::prelude::*;
use bevy::prelude::*;

/// Marker for any entity that responds to character controller messages
#[derive(Component)]
#[require(Position, Rotation, CustomPositionIntegration)]
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
    time: Res<Time<Fixed>>,
    move_and_slide: MoveAndSlide,
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
    for (entity, collider, position, rotation, linear_velocity, angular_velocity) in &mut characters
    {
        let velocity = **linear_velocity;
        if !velocity.is_finite() {
            panic!("Entity {entity} has infinite or NaN velocity");
        }

        if velocity != Vec2::ZERO {
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

            commands.entity(entity).insert((
                Position(output.position),
                LinearVelocity(output.projected_velocity),
            ));
        }

        let new_rotation =
            Rotation::radians(rotation.as_radians() + **angular_velocity * time.delta_secs());
        commands.entity(entity).insert(new_rotation);
    }
}
