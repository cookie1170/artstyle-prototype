fn render_outline(
    view: ViewQuery<(&ViewTarget, &OutlineRenderTarget)>,
    pipelines: If<Res<OutlinePipelines>>,
    pipeline_cache: Res<PipelineCache>,
    images: Res<RenderAssets<GpuImage>>,
    mut ctx: RenderContext,
) -> Result {
    let (view_target, outline_render_target) = view.into_inner();
    let desc_a = TextureDescriptor {
        label: Some("outline_a"),
        size: view_target.main_texture().size(),
        mip_level_count: 1,
        sample_count: 1,
        dimension: TextureDimension::D2,
        format: TextureFormat::Rgba16Uint,
        usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING,
        view_formats: &[TextureFormat::Rgba16Uint],
    };
    let texture_a = ctx.render_device().create_texture(&desc_a);
    let texture_b = ctx.render_device().create_texture(&TextureDescriptor {
        label: Some("outline_b"),
        ..desc_a
    });

    let texture_a_view = texture_a.create_view(&TextureViewDescriptor {
        label: Some("outline_a_view"),
        ..default()
    });
    let texture_b_view = texture_b.create_view(&TextureViewDescriptor {
        label: Some("outline_b_view"),
        ..default()
    });

    let size = view_target.main_texture().size();

    let layout = &pipeline_cache.get_bind_group_layout(&pipelines.init.layout);
    let uniform_buffer = ctx
        .render_device()
        .create_buffer_with_data(&BufferInitDescriptor {
            label: Some("step_width_buffer"),
            contents: &[size.width.to_ne_bytes(), size.height.to_ne_bytes()].concat(),
            usage: BufferUsages::UNIFORM,
        });

    let entries = BindGroupEntries::single(BufferBinding {
        buffer: &uniform_buffer,
        offset: 0,
        size: None,
    })
    .to_vec();

    let bind_group =
        ctx.render_device()
            .create_bind_group("outline_init_bind_group", layout, &entries);
    let mut init_pass = ctx
        .command_encoder()
        .begin_render_pass(&RenderPassDescriptor {
            label: Some("outline_init_pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: &texture_a_view,
                depth_slice: None,
                resolve_target: None,
                ops: Operations::default(),
            })],
            ..default()
        });

    let pipeline = rq!(
        Ok(()),
        pipeline_cache.get_render_pipeline(pipelines.init.id)
    );

    init_pass.set_pipeline(pipeline);
    init_pass.set_bind_group(0, &bind_group, &[]);
    init_pass.draw(0..3, 0..1);
    drop(init_pass);

    const STEPS: u32 = 2;
    let mut read_buffer = &texture_a_view;
    let mut write_buffer = &texture_b_view;
    let outline_ids = rq!(Ok(()), images.get(&outline_render_target.handle));
    for i in (0..STEPS).rev() {
        let layout = &pipeline_cache.get_bind_group_layout(&pipelines.jumpflood.layout);
        let uniform_buffer = ctx
            .render_device()
            .create_buffer_with_data(&BufferInitDescriptor {
                label: Some("step_width_buffer"),
                contents: &2u32.pow(i).to_ne_bytes(),
                usage: BufferUsages::UNIFORM,
            });

        let entries = BindGroupEntries::sequential((
            read_buffer,
            &outline_ids.texture_view,
            BufferBinding {
                buffer: &uniform_buffer,
                offset: 0,
                size: None,
            },
        ))
        .to_vec();

        let bind_group =
            ctx.render_device()
                .create_bind_group("outline_jumpflood_bind_group", layout, &entries);
        let mut jumpflood_pass = ctx
            .command_encoder()
            .begin_render_pass(&RenderPassDescriptor {
                label: Some("outline_jumpflood_pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: write_buffer,
                    depth_slice: None,
                    resolve_target: None,
                    ops: Operations::default(),
                })],
                ..default()
            });
        let pipeline = rq!(
            Ok(()),
            pipeline_cache.get_render_pipeline(pipelines.jumpflood.id)
        );
        jumpflood_pass.set_pipeline(pipeline);
        jumpflood_pass.set_bind_group(0, &bind_group, &[]);
        jumpflood_pass.draw(0..3, 0..1);
        std::mem::swap(&mut read_buffer, &mut write_buffer);
    }

    let layout = &pipeline_cache.get_bind_group_layout(&pipelines.draw.layout);

    let write = view_target.post_process_write();
    let entries = BindGroupEntries::sequential((read_buffer, write.source)).to_vec();

    let bind_group =
        ctx.render_device()
            .create_bind_group("outline_draw_bind_group", layout, &entries);

    let mut draw_pass = ctx
        .command_encoder()
        .begin_render_pass(&RenderPassDescriptor {
            label: Some("outline_draw_pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: write.destination,
                depth_slice: None,
                resolve_target: None,
                ops: Operations::default(),
            })],
            ..default()
        });
    let pipeline = rq!(
        Ok(()),
        pipeline_cache.get_render_pipeline(pipelines.draw.id)
    );
    draw_pass.set_pipeline(pipeline);
    draw_pass.set_bind_group(0, &bind_group, &[]);
    draw_pass.draw(0..3, 0..1);

    Ok(())
}

#[derive(Resource, Debug, Clone)]
struct OutlinePipelines {
    init: OutlinePipeline,
    jumpflood: OutlinePipeline,
    draw: OutlinePipeline,
}

#[derive(Debug, Clone)]
struct OutlinePipeline {
    layout: BindGroupLayoutDescriptor,
    id: CachedRenderPipelineId,
}

fn init_outline_pipeline(
    assets: Res<AssetServer>,
    fullscreen_shader: Res<FullscreenShader>,
    cache: ResMut<PipelineCache>,
    mut cmd: Commands,
) {
    let init = {
        let entries =
            BindGroupLayoutEntries::single(ShaderStages::FRAGMENT, uniform_buffer::<UVec2>(false))
                .to_vec();

        let layout = BindGroupLayoutDescriptor {
            label: "outline_init_bind_group_layout".into(),
            entries,
        };

        let shader = load_embedded_asset!(&*assets, "outline_init.wesl");
        let vertex = fullscreen_shader.to_vertex_state();
        let id = cache.queue_render_pipeline(RenderPipelineDescriptor {
            label: Some("outline_init_pipeline".into()),
            layout: vec![layout.clone()],
            vertex,
            fragment: Some(FragmentState {
                shader,
                targets: vec![Some(ColorTargetState {
                    format: TextureFormat::Rgba16Uint,
                    blend: None,
                    write_mask: ColorWrites::ALL,
                })],
                ..default()
            }),
            ..default()
        });
        OutlinePipeline { layout, id }
    };

    let jumpflood = {
        let entries = BindGroupLayoutEntries::sequential(
            ShaderStages::FRAGMENT,
            (
                texture_2d(TextureSampleType::Uint),
                texture_2d(TextureSampleType::Float { filterable: false }),
                uniform_buffer::<i32>(false),
            ),
        )
        .to_vec();

        let layout = BindGroupLayoutDescriptor {
            label: "outline_jumpflood_bind_group_layout".into(),
            entries,
        };

        let shader = load_embedded_asset!(&*assets, "outline_jumpflood.wesl");
        let vertex = fullscreen_shader.to_vertex_state();
        let id = cache.queue_render_pipeline(RenderPipelineDescriptor {
            label: Some("outline_jumpflood_pipeline".into()),
            layout: vec![layout.clone()],
            vertex,
            fragment: Some(FragmentState {
                shader,
                targets: vec![Some(ColorTargetState {
                    format: TextureFormat::Rgba16Uint,
                    blend: None,
                    write_mask: ColorWrites::ALL,
                })],
                ..default()
            }),
            ..default()
        });

        OutlinePipeline { layout, id }
    };

    let draw = {
        let entries = BindGroupLayoutEntries::sequential(
            ShaderStages::FRAGMENT,
            (
                texture_2d(TextureSampleType::Uint),
                texture_2d(TextureSampleType::Float { filterable: false }),
            ),
        )
        .to_vec();

        let layout = BindGroupLayoutDescriptor {
            label: "outline_draw_bind_group_layout".into(),
            entries,
        };

        let shader = load_embedded_asset!(&*assets, "outline_draw.wesl");
        let vertex = fullscreen_shader.to_vertex_state();
        let id = cache.queue_render_pipeline(RenderPipelineDescriptor {
            label: Some("outline_draw_pipeline".into()),
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

        OutlinePipeline { layout, id }
    };

    cmd.insert_resource(OutlinePipelines {
        init,
        jumpflood,
        draw,
    });
}

#[derive(AsBindGroup, Asset, Reflect, PartialEq, Debug, Clone)]
pub struct OutlineMaterial {
    #[uniform(0)]
    pub id: u32,
}

impl Material for OutlineMaterial {
    fn fragment_shader() -> ShaderRef {
        "embedded://artstyle_prototype/outline/render/outline_mesh.wesl".into()
    }
}

pub(super) fn plugin(app: &mut App) {
    load_shader_library!(app, "outline_common.wesl");
    embedded_asset!(app, "outline_init.wesl");
    embedded_asset!(app, "outline_jumpflood.wesl");
    embedded_asset!(app, "outline_draw.wesl");
    embedded_asset!(app, "outline_mesh.wesl");

    app.add_plugins((
        MaterialPlugin::<OutlineMaterial>::default(),
        ExtractResourcePlugin::<OutlineConfig, RenderApp>::default(),
        ExtractComponentPlugin::<OutlineRenderTarget, RenderApp>::default(),
    ));

    let render_app = app.sub_app_mut(RenderApp);
    render_app
        .add_systems(RenderStartup, init_outline_pipeline)
        .add_systems(
            Core3d,
            render_outline.in_set(Core3dSystems::EarlyPostProcess),
        );
}

use super::OutlineConfig;
use bevy::{
    asset::load_embedded_asset,
    core_pipeline::{Core3d, Core3dSystems, FullscreenShader},
    extract::{extract_component::ExtractComponentPlugin, extract_resource::ExtractResourcePlugin},
    material::descriptor::{
        BindGroupLayoutDescriptor, CachedRenderPipelineId, FragmentState, RenderPipelineDescriptor,
    },
    render::{
        RenderApp, RenderStartup,
        render_asset::RenderAssets,
        render_resource::{
            BindGroupEntries, BindGroupLayoutEntries, BufferBinding, BufferInitDescriptor,
            BufferUsages, ColorTargetState, ColorWrites, Operations, PipelineCache,
            RenderPassColorAttachment, RenderPassDescriptor, ShaderStages, TextureDescriptor,
            TextureDimension, TextureFormat, TextureSampleType, TextureUsages,
            TextureViewDescriptor,
            binding_types::{texture_2d, uniform_buffer},
        },
        renderer::{RenderContext, ViewQuery},
        texture::GpuImage,
        view::ViewTarget,
    },
    shader::load_shader_library,
};

use crate::{outline::OutlineRenderTarget, prelude::*};
