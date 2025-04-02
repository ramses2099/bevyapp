use bevy::prelude::*;
mod tutorial;

// #region Entity

// #endregion

// #region Component
use tutorial::plugins::HelloPlugin;
// #endregion

// #region System

// #endregion

fn main() {
    App::new().add_plugins(DefaultPlugins).add_plugins(HelloPlugin).run();
}
