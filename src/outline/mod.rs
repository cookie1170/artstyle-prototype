mod render;

/// The [`RenderLayers`] for an outline mesh.
const OUTLINE_LAYER: RenderLayers = RenderLayers::layer(15);

/// Configures the outline rendering.
#[derive(AsBindGroup, Resource, ExtractResource, Reflect, PartialEq, Debug, Clone)]
#[reflect(Resource)]
#[extract_app(RenderApp)]
pub struct OutlineConfig {
    #[uniform(3)]
    pub width: f32,
}

impl Default for OutlineConfig {
    fn default() -> Self {
        Self { width: 4.0 }
    }
}

/// Marks this entity and all its child entities as being outlined.
///
/// All entities that are chilren of the same `Outline` will be outlined together.
#[derive(Component, Reflect, PartialEq, Debug, Default, Clone)]
#[reflect(Component, Default)]
pub struct Outline;

fn on_outline_added(
    ev: On<Add<Outline>>,
    mut id: ResMut<NextOutlineId>,
    mut materials: ResMut<Assets<OutlineMaterial>>,
    mut cmd: Commands,
) {
    let id = id.next();
    cmd.entity(ev.entity).try_insert(OutlineId {
        id,
        material: materials.add(OutlineMaterial { id }),
    });
}

fn on_outline_removed(ev: On<Remove<Outline>>, mut cmd: Commands) {
    cmd.entity(ev.entity).try_remove::<OutlineId>();
}

/// A unique ID for each [`Outline`] entity, which determines which objects get outlined together.
///
/// These IDs are rendered to a separate texture by [outline cameras](OutlineCameraOf) using [outline meshes](OutlineMeshOf)
#[derive(Component, Reflect, PartialEq, Debug, Default, Clone)]
#[reflect(Component, Default)]
#[component(immutable)]
struct OutlineId {
    id: u32,
    material: Handle<OutlineMaterial>,
}

fn on_outline_id_removed(
    ev: On<Remove<OutlineId>>,
    child_of: Query<&ChildOf>,
    ids: Query<(), With<OutlineId>>,
    users: Query<&OutlineIdFor>,
    mut cmd: Commands,
) {
    let users = rq!(users.get(ev.entity));
    let mut next_id = None;
    for parent in child_of.iter_ancestors(ev.entity) {
        cq!(ids.get(parent));
        next_id = Some(parent);
    }

    if let Some(id) = next_id {
        cmd.entity(id)
            .add_related::<UsesOutlineId>(users.collection());
    } else {
        for user in users.iter() {
            cmd.entity(user).try_despawn();
        }
    }
}

fn on_outline_id_added(
    ev: On<Add<OutlineId>>,
    children: Query<&Children>,
    id: Query<&OutlineId>,
    meshes: Query<&Mesh3d, Without<OutlineMeshOf>>,
    mut cmd: Commands,
) {
    let id = rq!(id.get(ev.entity));
    for child in std::iter::once(ev.entity).chain(children.iter_descendants(ev.entity)) {
        let mesh = cq!(meshes.get(child));
        cmd.spawn((
            mesh.clone(),
            MeshMaterial3d(id.material.clone()),
            UsesOutlineId(ev.entity),
            OUTLINE_LAYER.clone(),
            ChildOf(child),
            OutlineMeshOf(child),
        ));
    }
}

/// Determines which entity's [`OutlineId`] is used for this entity.
#[derive(Component, Reflect, PartialEq, Debug, Clone)]
#[reflect(Component)]
#[relationship(relationship_target = OutlineIdFor)]
struct UsesOutlineId(pub Entity);

/// Determines which entities use this entity's [`OutlineId`]
#[derive(Component, Reflect, PartialEq, Debug, Clone)]
#[reflect(Component)]
#[relationship_target(relationship = UsesOutlineId)]
struct OutlineIdFor(Vec<Entity>);

/// Determines which mesh this mesh is an outline of.
///
/// Outline meshes are drawn by an [outline camera](OutlineCameraOf)
/// to a separate texture containing just their IDs, which are then used by the outline fullscreen shader.
#[derive(Component, Reflect, PartialEq, Debug, Clone)]
#[reflect(Component)]
#[relationship(relationship_target = WithOutlineMesh)]
struct OutlineMeshOf(pub Entity);

/// Determines which outline mesh is used by this mesh.
///
/// See [`OutlineMeshOf`].
#[derive(Component, Deref, Reflect, PartialEq, Debug, Clone)]
#[reflect(Component)]
#[relationship_target(relationship = OutlineMeshOf)]
struct WithOutlineMesh(Entity);

fn update_changed_meshes(
    changed: Query<(&Mesh3d, &WithOutlineMesh), Changed<Mesh3d>>,
    mut outline_meshes: Query<&mut Mesh3d, Without<WithOutlineMesh>>,
) {
    for (mesh, outline) in changed {
        let mut outline = cq!(outline_meshes.get_mut(**outline));
        outline.0 = mesh.0.clone();
    }
}

/// Determines which camera this camera is an outline camera of.
///
/// An outline camera draws all [outline meshes](OutlineMeshOf), and only those,
/// to a separate texture that the outline shader later samples from.
#[derive(Component, Reflect, PartialEq, Debug, Clone)]
#[reflect(Component)]
#[relationship(relationship_target = WithOutlineCamera)]
struct OutlineCameraOf(pub Entity);

/// Determines which outline camera this camera uses.
///
/// See [`OutlineCameraOf`].
#[derive(Component, Deref, Reflect, PartialEq, Debug, Clone)]
#[reflect(Component)]
#[relationship_target(relationship = OutlineCameraOf)]
struct WithOutlineCamera(Entity);

/// The [`Image`] that outline IDs will be rendered to by the [outline camera](OutlineCameraOf)
#[derive(Component, ExtractComponent, Reflect, PartialEq, Debug, Default, Clone)]
#[reflect(Component, Default)]
#[extract_app(RenderApp)]
struct OutlineRenderTarget {
    handle: Handle<Image>,
    last_size: UVec2,
}

fn on_camera_added(
    ev: On<Add<Camera>>,
    cam: Query<(&Camera, &Projection), Without<OutlineCameraOf>>,
    mut cmd: Commands,
) {
    let (camera, projection) = rq!(cam.get(ev.entity));
    cmd.spawn((
        Camera3d::default(),
        Camera {
            order: camera.order - 1,
            msaa_writeback: MsaaWriteback::Off,
            clear_color: ClearColorConfig::Custom(Color::LinearRgba(LinearRgba::from_u8_array(
                [0; 4],
            ))),
            ..default()
        },
        // this is temporarily set to `None` because we don't know the size of the camera yet
        // (it hasn't been computed when the camera is added!), so we can't create an image for it
        RenderTarget::None {
            size: uvec2(1920, 1080),
        },
        projection.clone(),
        OutlineCameraOf(ev.entity),
        ChildOf(ev.entity),
        Msaa::Off,
        OUTLINE_LAYER.clone(),
    ));
}

fn update_outline_cameras(
    changed: Query<
        (
            Entity,
            &Projection,
            &Camera,
            Option<&mut OutlineRenderTarget>,
            &WithOutlineCamera,
        ),
        Or<(Changed<Camera>, Changed<Projection>)>,
    >,
    mut outline_cameras: Query<(&mut Projection, &mut RenderTarget), Without<WithOutlineCamera>>,
    mut images: ResMut<Assets<Image>>,
    mut cmd: Commands,
) {
    for (entity, projection, camera, outline_target, outline_camera) in changed {
        let size = cq!(camera.physical_target_size());
        let (mut outline_projection, mut target) = cq!(outline_cameras.get_mut(**outline_camera));
        if outline_target.is_none_or(|t| t.last_size != size) {
            debug!("creating outline render target");
            let mut image = Image::new_uninit(
                Extent3d {
                    width: size.x as u32,
                    height: size.y as u32,
                    depth_or_array_layers: 1,
                },
                TextureDimension::D2,
                // `Rgba8Uint` here would make more sense, however, the mesh pipeline only supports float formats.
                // so in the shader we use the packing functions to convert them to and from u32s
                TextureFormat::Rgba8Unorm,
                RenderAssetUsages::RENDER_WORLD,
            );
            image.texture_descriptor.usage |= TextureUsages::RENDER_ATTACHMENT;
            let handle = images.add(image);
            let new_target = OutlineRenderTarget {
                handle: handle.clone(),
                last_size: size,
            };
            cmd.entity(entity).insert(new_target);
            *target = RenderTarget::Image(ImageRenderTarget {
                handle,
                scale_factor: 1.0,
            });
        }
        *outline_projection = projection.clone();
    }
}

/// The last [`OutlineId`] that's been used.
#[derive(Resource, Reflect, PartialEq, Debug, Default, Clone)]
#[reflect(Resource)]
struct NextOutlineId(u32);

impl NextOutlineId {
    /// Returns the next ID that should be used and updates the current one, wrapping once [`u32::MAX`] has been reached.
    pub fn next(&mut self) -> u32 {
        // 0 is reserved for the background
        if self.0 == 0 {
            self.0 = 1;
        }
        let current = self.0;
        self.0 = self.0.wrapping_add(1);
        current
    }
}

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<OutlineConfig>()
        .init_resource::<NextOutlineId>()
        .add_observer(on_outline_added)
        .add_observer(on_outline_removed)
        .add_observer(on_outline_id_added)
        .add_observer(on_outline_id_removed)
        .add_observer(on_camera_added)
        .add_systems(
            PostUpdate,
            (
                update_changed_meshes.ambiguous_with_all(),
                update_outline_cameras.ambiguous_with_all(),
            ),
        )
        .add_plugins(render::plugin);
}

use crate::{outline::render::OutlineMaterial, prelude::*};
use bevy::{
    asset::RenderAssetUsages,
    camera::{ImageRenderTarget, RenderTarget, visibility::RenderLayers},
    render::{
        RenderApp,
        extract_component::ExtractComponent,
        extract_resource::ExtractResource,
        render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages},
    },
};
