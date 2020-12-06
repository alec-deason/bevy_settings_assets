use bevy::prelude::*;
use bevy_settings_assets::{SettingsPlugin, Settings};
use serde::Deserialize;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(SettingsPlugin::default())
        .add_startup_system(setup)
        .add_system(hello_world_system)
        .run();
}

#[derive(Default, Deserialize)]
struct MySettingsA {
    a: u32
}

#[derive(Default, Deserialize)]
struct MySettingsB {
    b: String
}

fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut settings: ResMut<Settings>,
) {
    settings.add::<MySettingsA>("a.settings", commands, &asset_server);
    settings.add::<MySettingsB>("b.settings", commands, &asset_server);
    asset_server.watch_for_changes().unwrap();
}

fn hello_world_system(settings_a: Res<MySettingsA>, settings_b: Res<MySettingsB>) {
    println!("a: {}", settings_a.a);
    println!("b: {}", settings_b.b);
}
