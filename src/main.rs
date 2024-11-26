//! Test

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    commands.spawn(Text::new("no default font, expected to not be visible"));

    let font = asset_server.load("FiraSans-Bold.ttf");

    commands
        .spawn((
            Text::default(),
            // Uncommenting this makes things work
            // TextFont {
            //     font: font.clone(),
            //     ..default()
            // },
        ))
        .with_children(|parent| {
            parent.spawn((
                TextSpan::new("span, should be visible"),
                TextFont { font, ..default() },
            ));
        });
}
