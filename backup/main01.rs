use bevy::prelude::*;

// #region Entity

// #endregion

// #region Component

// #endregion

// #region System

// #endregion

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: String::from("Test Window"),
                    position: WindowPosition::Centered(MonitorSelection::Primary),
                    resolution: Vec2::new(512., 512.).into(),
                    ..Default::default()
                }),
                ..Default::default()
            }).set(ImagePlugin::default_nearest())
        )
        .run();
}
