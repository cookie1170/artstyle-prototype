import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput;
import super::outline_common::{is_null, null_pos, unpack_depth, uv_to_pos};

@group(0) @binding(0)
var texture: texture_2d<u32>;

@group(0) @binding(1)
var<uniform> step_width: i32;

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4f {
    let dim = textureDimensions(texture);
    let pos = uv_to_pos(in.uv, dim);
    let value = textureLoad(texture, pos, 0);
    let current_pos = value.rg;
    let depth = unpack_depth(value.ba);
    // only visualise the pixels added by the flood
    if !is_null(current_pos) {
        return vec4(vec3(depth), 1.0);
    }

    // big number
    var best_dist = 1000000u;
    var best_pos = null_pos;
    for(var u = -1; u <= 1; u += 1) {
        for(var v = -1; v <= 1; v += 1) {
            let offset_pos = vec2i(pos) + vec2(u, v) * step_width;
            let sample_pos = clamp(offset_pos, vec2(0), vec2i(dim));
            let test_pos = textureLoad(texture, sample_pos, 0).rg;
            let diff = pos - test_pos;
            let sqr_dist = dot(diff, diff);
            if !is_null(test_pos) && sqr_dist < best_dist {
                best_pos = test_pos;
                best_dist = sqr_dist;
            }
        }
    }

    // TEMP: divide it by `dim` because currently we're just displaying it to the screen.
    return vec4(vec2f(best_pos) / vec2f(dim), 0.0, 1.0);
}
