#[derive(Component, Reflect, PartialEq, Debug, Default, Clone)]
#[reflect(Component, Default)]
pub struct Outline;

#[derive(AsBindGroup, Asset, Reflect, PartialEq, Debug, Clone)]
struct InverseHullMaterial {}

impl Material for InverseHullMaterial {
    fn vertex_shader() -> ShaderRef {
        "embedded://artstyle_prototype/outline/outline.wesl".into()
    }

    fn fragment_shader() -> ShaderRef {
        "embedded://artstyle_prototype/outline/outline.wesl".into()
    }

    fn specialize(
        _pipeline: &MaterialPipeline,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayoutRef,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor.primitive.cull_mode = Some(Face::Front);
        Ok(())
    }
}

fn on_add_outline(
    ev: On<Add<Outline>>,
    mesh: Query<&Mesh3d>,
    mut material: Local<Option<Handle<InverseHullMaterial>>>,
    mut materials: ResMut<Assets<InverseHullMaterial>>,
    mut cmd: Commands,
) -> Result {
    let mat = material
        .get_or_insert_with(|| materials.add(InverseHullMaterial {}))
        .clone();
    let mesh = mesh
        .get(ev.entity)
        .context("entity with outline should have a mesh")?;

    cmd.entity(ev.entity).with_child((
        Name::new("Outline"),
        Mesh3d(mesh.0.clone()),
        MeshMaterial3d(mat),
    ));

    Ok(())
}

pub(super) fn plugin(app: &mut App) {
    embedded_asset!(app, "outline.wesl");
    app.add_plugins(MaterialPlugin::<InverseHullMaterial>::default())
        .add_observer(on_add_outline);
}

use bevy::{
    material::{descriptor::RenderPipelineDescriptor, specialize::SpecializedMeshPipelineError},
    mesh::MeshVertexBufferLayoutRef,
    pbr::{MaterialPipeline, MaterialPipelineKey},
    render::render_resource::Face,
};

use crate::prelude::*;
