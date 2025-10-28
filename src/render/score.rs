use std::time::Duration;

use bevy::prelude::*;

use crate::PuzzleState;
use crate::warehouse::structs::warehouse::Warehouse;

use super::objects::RenderObject;
use super::puzzle::PuzzleSolvingTicker;

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct Shrinker {
    pub timer: Timer
}

impl Shrinker {
    pub fn new(time: u64) -> Self {
        Self {
            timer: Timer::new(Duration::from_millis(time), TimerMode::Once)
        }
    }
}

#[derive(Resource)]
pub struct Score { pub score: usize }

pub fn setup_score(
    mut commands: Commands, 
    mut puzzle_ticker: ResMut<PuzzleSolvingTicker>,
) {
    commands.insert_resource(Score { score: 0});
    commands.spawn((
            Text::new(""),
            TextFont {
                font_size: 42.0,
                ..default()
            },
            ScoreText
    ));
    puzzle_ticker.timer.set_duration(Duration::from_millis(200));
    puzzle_ticker.timer.reset();
}

pub fn shrinking(
    time: Res<Time>,
    mut commands: Commands, 
    mut shrink_query: Query<(Entity, &mut Transform, &mut Shrinker)>
) {
    let delta = time.delta();
   for (entity, mut transform, mut shrinker) in shrink_query.iter_mut() {
        shrinker.timer.tick(delta);
        if shrinker.timer.is_finished() {
            commands.entity(entity).despawn();
        } else {
            let scale: f32 = shrinker.timer.elapsed().as_millis() as f32 / shrinker.timer.duration().as_millis() as f32;
            transform.scale = Vec3::new(1.0 - scale, 1.0 - scale, 1.0 - scale);
            transform.translation.z = 0.5 + 10.0 * scale;
        }
   } 
}

#[allow(clippy::too_many_arguments)]
pub fn score_trigger(
    mut commands: Commands,
    time: Res<Time>,
    mut score: ResMut<Score>,
    mut warehouse: ResMut<Warehouse>,
    mut text_query: Query<&mut Text, With<ScoreText>>,
    mut next_puzzle_state: ResMut<NextState<PuzzleState>>,
    objects_query: Query<(Entity, &RenderObject)>,
    mut puzzle_ticker: ResMut<PuzzleSolvingTicker>,
) {
    puzzle_ticker.timer.tick(time.delta());

    if puzzle_ticker.timer.is_finished() && !warehouse.objects.is_empty() {
        let anim = puzzle_ticker.timer.duration().as_millis();
        if anim > 4 { puzzle_ticker.timer.set_duration(Duration::from_millis(anim as u64 - 1)); }

        if let Some((entity, object)) = objects_query.iter().next() {
            if let Some(pos) = warehouse.objects.remove(&object.index) {
                score.score += 100 * pos.y as usize + pos.x as usize;
            }
            commands.entity(entity).insert(Shrinker::new(400));
        }

        for mut text in &mut text_query { **text = format!("{:08}", score.score); }

        if warehouse.objects.is_empty() { next_puzzle_state.set(PuzzleState::Completed); }
    }
}
