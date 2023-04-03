#![allow(dead_code)]

use crate::game::SideEffectGamePlugin;
use bevy::prelude::*;

mod core;
mod game;
mod states;

fn main() {
    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(AssetPlugin {
                // Tell the asset server to watch for asset changes on disk:
                watch_for_changes: true,
                ..default()
            }),
    );
    app.add_plugin(SideEffectGamePlugin);

    app.run();
}
