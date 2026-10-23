mod render;

#[derive(AsBindGroup, Resource, ExtractResource, Reflect, Default, PartialEq, Debug, Clone)]
#[reflect(Resource)]
#[extract_app(RenderApp)]
pub struct OutlineConfig {}

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<OutlineConfig>()
        .add_plugins(render::plugin);
}

use crate::prelude::*;
use bevy::render::{RenderApp, extract_resource::ExtractResource};
