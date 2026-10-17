//! common imports for this project, such as preludes from dependencies
#![allow(unused_imports)]

pub use bevy::{
    anti_alias::{fxaa::Fxaa, smaa::Smaa, taa::TemporalAntiAliasing},
    app::PluginGroupBuilder,
    asset::embedded_asset,
    camera::CameraProjection,
    core_pipeline::prepass::DepthPrepass,
    image::{ImageFilterMode, ImageSamplerDescriptor},
    input::common_conditions::*,
    light::{Atmosphere, atmosphere::ScatteringMedium},
    pbr::{AtmosphereSettings, ExtendedMaterial, MaterialExtension},
    post_process::bloom::Bloom,
    prelude::*,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
    tasks::{AsyncComputeTaskPool, ComputeTaskPool, IoTaskPool, block_on},
};
pub use std::{collections::HashMap, marker::PhantomData, time::Duration};
pub use tiny_bail::prelude::*;
