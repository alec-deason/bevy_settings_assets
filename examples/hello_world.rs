use bevy::{app::ScheduleRunnerSettings, asset::AssetPlugin, prelude::*, utils::Duration};
use bevy_settings_assets::{Settings, SettingsPlugin};
use serde::Deserialize;

fn main() {
    App::build()
        .insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs_f64(
            1.0 / 60.0,
        )))
        .add_plugins(MinimalPlugins)
        .add_plugin(AssetPlugin)
        .add_plugin(SettingsPlugin::default())
        .add_startup_system(setup.system())
        .add_system(hello_world_system.system())
        .run();
}

#[derive(Default, Deserialize)]
struct MySettingsA {
    a: u32,
}

#[derive(Default, Deserialize)]
struct MySettingsB {
    b: String,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut settings: ResMut<Settings>) {
    settings.add::<MySettingsA>("a.settings", &mut commands, &asset_server);
    settings.add::<MySettingsB>("b.settings", &mut commands, &asset_server);
    asset_server.watch_for_changes().unwrap();
}

fn hello_world_system(settings_a: Res<MySettingsA>, settings_b: Res<MySettingsB>) {
    println!("a: {}", settings_a.a);
    println!("b: {}", settings_b.b);
}
