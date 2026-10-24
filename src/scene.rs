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
        Transform {
            translation: Vec3 {
                y: -0.05
            }
        }
        Mesh3d(asset_value(Cuboid::new(16.0, 0.1, 16.0)))
        MeshMaterial3d<StandardMaterial>(asset_value(StandardMaterial::default()))
        Outline
    });

    cmd.spawn_scene(bsn! {
        @FeathersSlider {
            @min: 1.0,
            @max: 64.0
        }
        Node {
            width: px(100),
            margin: UiRect::all(px(4))
        }
        SliderValue(4.0)
        SliderPrecision(0)
        on(slider_self_update)
        on(|ev: On<ValueChange<f32>>, mut config: ResMut<OutlineConfig>| {
            config.width = ev.value;
        })
    });
}

fn camera() -> impl Scene {
    bsn! {
        #Camera
        Camera3d
        AtmosphereSettings
        Smaa
        Msaa::Off

        Hdr
        Bloom {
            intensity: 0.05,
            ..Bloom::NATURAL
        }
        Projection::Perspective(PerspectiveProjection {
            fov: {70f32.to_radians()},
        })


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
    app.add_plugins((FreeCameraPlugin, FeathersPlugins))
        .insert_resource(UiTheme(create_dark_theme()));
    app.add_systems(Startup, setup);
}

use bevy::{
    camera::Hdr,
    camera_controller::free_camera::{FreeCamera, FreeCameraPlugin},
    feathers::{
        FeathersPlugins, controls::FeathersSlider, dark_theme::create_dark_theme, theme::UiTheme,
    },
    ui_widgets::{SliderPrecision, SliderValue, ValueChange, slider_self_update},
};

use crate::{
    outline::{Outline, OutlineConfig},
    prelude::*,
};
