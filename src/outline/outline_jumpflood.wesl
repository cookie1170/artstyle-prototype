import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput;
import bevy_render::view::{View, depth_ndc_to_view_z};
import super::outline_common::{is_null, null_pos, unpack_depth, uv_to_pos};

@group(0) @binding(0)
var texture: texture_2d<u32>;

@group(0) @binding(1)
var<uniform> step_width: i32;

@group(0) @binding(2)
var<uniform> view: View;

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4u {
    let dim = textureDimensions(texture);
    let pos = uv_to_pos(in.uv, dim);
    let value = textureLoad(texture, pos, 0);
    let current_pos = value.rg;
    let depth = -depth_ndc_to_view_z(
            unpack_depth(value.ba),
            view.clip_from_view,
            view.view_from_clip,
        );

    // big number
    var best_dist = 1000000u;
    var best_depth = depth;
    var best_pos = vec4(null_pos, 0, 0);
    for(var u = -1; u <= 1; u += 1) {
        for(var v = -1; v <= 1; v += 1) {
            let offset_pos = vec2i(pos) + vec2(u, v) * step_width;
            let sample_pos = clamp(offset_pos, vec2(0), vec2i(dim));
            let test_pos = textureLoad(texture, sample_pos, 0);
            let other_depth = -depth_ndc_to_view_z(
                    unpack_depth(test_pos.ba),
                    view.clip_from_view,
                    view.view_from_clip,
                );
            let diff = pos - test_pos.rg;
            let sqr_dist = dot(diff, diff);
            if !is_null(test_pos.rg) && sqr_dist < best_dist && other_depth - depth < -0.1 {
                best_pos = test_pos;
                best_dist = sqr_dist;
                best_depth = other_depth;
            }
        }
    }

    return best_pos;
}
