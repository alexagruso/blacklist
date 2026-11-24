use bevy::{input::common_conditions, prelude::*};

use crate::player::Player;

pub struct AimingPlugin;

impl Plugin for AimingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            toggle_aiming_zoom.run_if(
                common_conditions::input_just_pressed(MouseButton::Right)
                    .or_else(common_conditions::input_just_released(MouseButton::Right)),
            ),
        );
    }
}

#[derive(Component, Default)]
pub struct Aiming;

#[derive(Resource)]
pub struct AimingZoomConfig {
    pub in_time_secs: f32,
    pub out_time_secs: f32,
    pub scale_modifier: f32,
}

impl Default for AimingZoomConfig {
    fn default() -> Self {
        Self {
            in_time_secs: 0.1,
            out_time_secs: 0.05,
            scale_modifier: 0.1,
        }
    }
}

fn toggle_aiming_zoom(player: Single<(Entity, Has<Aiming>), With<Player>>, mut commands: Commands) {
    let (player_entity, is_aiming) = player.into_inner();
    if is_aiming {
        commands.entity(player_entity).remove::<Aiming>();
    } else {
        commands.entity(player_entity).insert(Aiming);
    }
}
