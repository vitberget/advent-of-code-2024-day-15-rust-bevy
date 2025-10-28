use bevy::prelude::*;

pub fn escape_forever(
    keys: Res<ButtonInput<KeyCode>>,
    mut app_exit_events: ResMut<Messages<bevy::app::AppExit>>
) {
    if keys.just_pressed(KeyCode::Escape) {
        app_exit_events.write(bevy::app::AppExit::default());
    }
}
