fn setup(mut cmd: Commands) {
    cmd.spawn_scene_list(bsn_list! {
        @camera()
        --
        @atmosphere()
        --
        @light()
        --
        #Cube
        Transform {
            translation: Vec3 {
                y: 1.5
            }
        }
        Mesh3d(asset_value(Cuboid::new(3.0, 3.0, 3.0)))
        MeshMaterial3d<StandardMaterial>(asset_value(StandardMaterial::default()))
        Outline
        --
        #Ground
        Mesh3d(asset_value(Plane3d::new(Vec3::Y, vec2(16.0, 16.0))))
        MeshMaterial3d<StandardMaterial>(asset_value(StandardMaterial::default()))
        Outline
    });
}

fn camera() -> impl Scene {
    bsn! {
        #Camera
        Camera3d
        Projection::Perspective(PerspectiveProjection {
            fov: {70f32.to_radians()},
        })
        AtmosphereSettings
        Transform::from_xyz(0.0, 6.0, 12.0).looking_to(vec3(0.0, -0.333, -1.0).normalize(), Vec3::Y)
        FreeCamera
    }
}

fn atmosphere() -> impl Scene {
    bsn! {
        #Atmosphere
        template(|ctx| {
            Ok(Atmosphere::earth(
                ctx.resource_mut::<Assets<ScatteringMedium>>().add(ScatteringMedium::default())
            ))
        })
    }
}

fn light() -> impl Scene {
    bsn! {
        #Sun
        DirectionalLight
        Transform::default().looking_to(vec3(-1.0, -2.0, -0.5).normalize(), Vec3::Y)
    }
}

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(FreeCameraPlugin);
    app.add_systems(Startup, setup);
}

use bevy::camera_controller::free_camera::{FreeCamera, FreeCameraPlugin};

use crate::{outline::Outline, prelude::*};
