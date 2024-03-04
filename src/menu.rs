use bevy::{app::PluginGroupBuilder, prelude::*};

pub struct MenuPlugin;

impl PluginGroup for MenuPlugin {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
    }
}
