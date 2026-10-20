#[derive(AsBindGroup, Resource, ExtractResource, Reflect, Default, PartialEq, Debug, Clone)]
#[reflect(Resource)]
#[extract_app(RenderApp)]
pub struct OutlineConfig {}

fn render_outline(
    view: ViewQuery<&ViewTarget>,
    pipeline: If<Res<OutlinePipeline>>,
    pipeline_cache: Res<PipelineCache>,
    data: If<Res<OutlineConfig>>,
    fallback_buffer: Res<FallbackBuffer>,
    shader_buffer_assets: Res<RenderAssets<GpuShaderBuffer>>,
    mut param: SystemParamItem<<OutlineConfig as AsBindGroup>::Param>,
    mut ctx: RenderContext,
) -> Result {
    let view_target = view.into_inner();
    let write = view_target.post_process_write();
    let layout = &pipeline_cache.get_bind_group_layout(&pipeline.layout);
    let mut bind_group_builder = BindGroupBuilder::default();
    match data.build_bind_group(
        layout,
        ctx.render_device(),
        &mut param,
        false,
        &mut bind_group_builder,
    ) {
        Ok(()) => {}
        Err(AsBindGroupError::RetryNextUpdate) => return Ok(()),
        e => e.context("failed to create outline bind group")?,
    }

    let mut entries = BindGroupEntries::sequential((
        // the screen texture
        write.source,
        &pipeline.sampler,
    ))
    .to_vec();

    entries.extend(
        bind_group_builder
            .binding_resources
            .iter()
            .map(|(index, binding)| BindGroupEntry {
                binding: *index,
                resource: binding.get_binding(&fallback_buffer, &shader_buffer_assets),
            }),
    );

    let bind_group =
        ctx.render_device()
            .create_bind_group(OutlineConfig::label(), layout, &entries);

    let mut pass = ctx
        .command_encoder()
        .begin_render_pass(&RenderPassDescriptor {
            label: Some("outline_render_pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: write.destination,
                depth_slice: None,
                resolve_target: None,
                ops: Operations::default(),
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });

    let pipeline = rq!(Ok(()), pipeline_cache.get_render_pipeline(pipeline.id));

    pass.set_pipeline(pipeline);
    pass.set_bind_group(0, &bind_group, &[]);
    pass.draw(0..3, 0..1);

    Ok(())
}

fn init_outline_pipeline(
    ctx: RenderContext,
    assets: Res<AssetServer>,
    fullscreen_shader: Res<FullscreenShader>,
    cache: ResMut<PipelineCache>,
    mut cmd: Commands,
) {
    let fullscreen_entries = BindGroupLayoutEntries::sequential(
        ShaderStages::FRAGMENT,
        (
            // the screen texture
            texture_2d(TextureSampleType::Float { filterable: true }),
            sampler(SamplerBindingType::Filtering),
        ),
    )
    .to_vec();
    let entries = OutlineConfig::bind_group_layout_entries(ctx.render_device(), false);
    let entries =
        combined_bind_group::combine_bind_group_layout_entries(fullscreen_entries, entries);

    let layout = BindGroupLayoutDescriptor {
        label: OutlineConfig::label().into(),
        entries,
    };
    let sampler = ctx
        .render_device()
        .create_sampler(&SamplerDescriptor::default());

    let shader = assets.load("embedded://artstyle_prototype/outline/outline.wesl");
    let vertex = fullscreen_shader.to_vertex_state();
    let pipeline_id = cache.queue_render_pipeline(RenderPipelineDescriptor {
        label: Some("outline_pipeline".into()),
        layout: vec![layout.clone()],
        vertex,
        fragment: Some(FragmentState {
            shader,
            targets: vec![Some(ColorTargetState {
                format: TextureFormat::Rgba16Float,
                blend: None,
                write_mask: ColorWrites::ALL,
            })],
            ..default()
        }),
        ..default()
    });

    cmd.insert_resource(OutlinePipeline {
        layout,
        sampler,
        id: pipeline_id,
    });
}

#[derive(Resource, Debug, Clone)]
struct OutlinePipeline {
    layout: BindGroupLayoutDescriptor,
    sampler: Sampler,
    id: CachedRenderPipelineId,
}

pub(super) fn plugin(app: &mut App) {
    embedded_asset!(app, "outline.wesl");

    app.init_resource::<OutlineConfig>()
        .add_plugins(ExtractResourcePlugin::<OutlineConfig, RenderApp>::default());

    let render_app = app.sub_app_mut(RenderApp);
    render_app
        .add_systems(RenderStartup, init_outline_pipeline)
        .add_systems(
            Core3d,
            render_outline.in_set(Core3dSystems::EarlyPostProcess),
        );
}

use bevy::{
    core_pipeline::{Core3d, Core3dSystems, FullscreenShader},
    ecs::system::SystemParamItem,
    extract::extract_resource::ExtractResourcePlugin,
    material::descriptor::{
        BindGroupLayoutDescriptor, CachedRenderPipelineId, FragmentState, RenderPipelineDescriptor,
    },
    render::{
        RenderApp, RenderStartup, combined_bind_group,
        extract_resource::ExtractResource,
        material_bind_groups::FallbackBuffer,
        render_asset::RenderAssets,
        render_resource::{
            AsBindGroupError, BindGroupBuilder, BindGroupEntries, BindGroupEntry,
            BindGroupLayoutEntries, ColorTargetState, ColorWrites, Operations, PipelineCache,
            RenderPassColorAttachment, RenderPassDescriptor, Sampler, SamplerBindingType,
            SamplerDescriptor, ShaderStages, TextureFormat, TextureSampleType,
            binding_types::{sampler, texture_2d},
        },
        renderer::{RenderContext, ViewQuery},
        storage::GpuShaderBuffer,
        view::ViewTarget,
    },
};

use crate::prelude::*;
