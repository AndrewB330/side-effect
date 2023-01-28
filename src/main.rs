#![allow(dead_code)]

use crate::game::ShiftsNShapesPlugin;
use bevy::prelude::*;

mod core;
mod game;
mod states;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);
    app.add_plugin(ShiftsNShapesPlugin);

    app.run();
}
