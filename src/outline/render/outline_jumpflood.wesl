import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput;
import super::outline_common::uv_to_pos;

@group(0) @binding(0)
var texture: texture_2d<u32>;

@group(0) @binding(1)
var ids: texture_2d<f32>;

@group(0) @binding(2)
var<uniform> step_width: i32;

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4u {
    let dim = textureDimensions(texture);
    let pos = uv_to_pos(in.uv, dim);
    let value = textureLoad(texture, pos, 0);
    let id = pack4x8unorm(textureLoad(ids, pos, 0));
    let current_pos = value.rg;

    // big number
    var best_dist = 1000000u;
    var best_pos = vec4(65535u, 65535, 65535, 65535);
    for(var u = -1; u <= 1; u += 1) {
        for(var v = -1; v <= 1; v += 1) {
            let offset_pos = vec2i(pos) + vec2(u, v) * step_width;
            let sample_pos = clamp(offset_pos, vec2(0), vec2i(dim));
            let test_pos = textureLoad(texture, sample_pos, 0);
            let test_id = pack4x8unorm(textureLoad(ids, sample_pos, 0));
            let diff = pos - test_pos.rg;
            let sqr_dist = dot(diff, diff);
            if id != test_id && sqr_dist < best_dist {
                best_pos = test_pos;
                best_dist = sqr_dist;
            }
        }
    }
    if all(best_pos == vec4(65535u, 65535, 65535, 65535)) {
        return value;
    }
    return best_pos;
}
