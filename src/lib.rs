use bevy::{
    asset::{AssetEvent, AssetLoader, Assets, LoadContext, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
    utils::BoxedFuture,
};
use serde::de::DeserializeOwned;
use std::collections::HashMap;

#[derive(Default)]
pub struct SettingsPlugin;

#[derive(Default)]
pub struct Settings(
    HashMap<HandleUntyped, Box<dyn FnMut(String, &mut Commands) + 'static + Send + Sync>>,
);

impl Settings {
    pub fn add<T: Send + Sync + Default + DeserializeOwned + 'static>(
        &mut self,
        path: &str,
        commands: &mut Commands,
        asset_server: &AssetServer,
    ) {
        commands.insert_resource(T::default());
        let handle = asset_server.load_untyped(path);
        asset_server.watch_for_changes().unwrap();
        self.0.insert(
            handle,
            Box::new(move |string, commands| {
                let settings = ron::de::from_str::<T>(&string);
                commands.insert_resource(settings.unwrap());
            }),
        );
    }
}

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_asset::<SettingsString>()
            .init_asset_loader::<SettingsLoader>()
            .init_resource::<Settings>()
            .add_system_to_stage(CoreStage::PostUpdate, setting_strings_system.system());
    }
}

#[derive(Debug, TypeUuid)]
#[uuid = "d86ef3f5-13f2-49e6-85c0-604882c24dd2"]
#[derive(Default)]
struct SettingsString(String);

#[derive(Default)]
struct SettingsLoader;
impl AssetLoader for SettingsLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            let string = std::str::from_utf8(bytes)?;
            load_context.set_default_asset(LoadedAsset::new(SettingsString(string.to_string())));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["settings"]
    }
}

fn setting_strings_system(
    mut commands: Commands,
    mut reader: EventReader<AssetEvent<SettingsString>>,
    settings_strings: Res<Assets<SettingsString>>,
    mut settings: ResMut<Settings>,
) {
    for event in reader.iter() {
        if let AssetEvent::Modified { handle } | AssetEvent::Created { handle } = event {
            for (target_handle, resource_constructor) in &mut settings.0 {
                if target_handle.id == handle.id {
                    if let Some(string) = settings_strings.get(handle) {
                        resource_constructor(string.0.clone(), &mut commands);
                    }
                    break;
                }
            }
        }
    }
}
