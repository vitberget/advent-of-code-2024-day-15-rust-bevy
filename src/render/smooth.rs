use std::time::Duration;

use bevy::prelude::*;

use crate::PuzzleState;

use super::player::RenderPlayerLight;

#[derive(Component)]
pub struct SmoothObject {
    pub from: Transform,
    pub to: Transform,
    pub timer: Timer,
    pub good: bool
}

impl SmoothObject {
    pub fn new(from: Transform, to: Transform, time: u64, good: bool) -> Self {
        SmoothObject {
            from,
            to,
            timer: Timer::new(Duration::from_millis(time), TimerMode::Once),
            good
        }
    }
}

#[derive(Component)]
pub struct TurnOffTheLight {
    pub timer: Timer,
}

impl TurnOffTheLight {
    pub fn new(time: u64) -> Self { 
        TurnOffTheLight {
            timer: Timer::new(Duration::from_millis(time), TimerMode::Once),
        }
    }
}

pub fn smooth_objects(
    mut commands: Commands,
    time: Res<Time>,
    mut smooth_query: Query<(Entity, &mut Transform, &mut SmoothObject)>,
    next_puzzle_state: Res<NextState<PuzzleState>>,
) {
    if !next_puzzle_state.is_added() {
        for (entity, mut transform, mut smooth) in &mut smooth_query {
            smooth.timer.tick(time.delta());
            let elapsed = smooth.timer.elapsed().as_millis();
            let duration = smooth.timer.duration().as_millis();

            if smooth.timer.is_finished() || duration < 10 || !smooth.good && elapsed > duration / 2 {
                *transform = if smooth.good { smooth.to } else { smooth.from };
                commands.entity(entity).remove::<SmoothObject>();
            } else {
                let elapsed = match (smooth.good, elapsed > duration / 4) {
                    (true, _) => elapsed,
                    (false, false) => elapsed,
                    (false, true) => duration / 2 - elapsed / 2
                };

                let d = (elapsed as f32) / (duration as f32);
                transform.translation = smooth.from.translation +  (smooth.to.translation - smooth.from.translation) * d;
            }
        }
    }
}

pub fn turn_off_the_light(
    mut commands: Commands,
    time: Res<Time>,
    mut light_query: Query<(Entity, &mut PointLight, &mut TurnOffTheLight), With<RenderPlayerLight>>,
    next_puzzle_state: Res<NextState<PuzzleState>>,
) {
    if !next_puzzle_state.is_added() {
        for (entity, mut light, mut turn_off_timer) in &mut light_query {
            turn_off_timer.timer.tick(time.delta());
            if turn_off_timer.timer.is_finished() {
                commands.entity(entity).remove::<TurnOffTheLight>();
                light.color = Color::srgb(1.0, 1.0, 1.0);
            }
        }
    }
}
