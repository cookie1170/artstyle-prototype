import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput;
import super::outline_common::{config, pack_id, unpack_id, uv_to_pos};

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
    let starting_diff = pos - value.rg;
    let starting_dist = 1000000u;

    var best_dist = starting_dist;
    var best_pos = vec4(0u);
    for(var u = -1; u <= 1; u++) {
        for(var v = -1; v <= 1; v++) {
            var sample_pos = vec2i(pos) + vec2(u, v) * step_width;
            sample_pos = clamp(sample_pos, vec2(0), vec2i(dim) - 1);

            let test_value = textureLoad(texture, sample_pos, 0);
            let test_pos = test_value.rg;
            let test_id = unpack_id(test_value.ba);

            let diff = test_pos - pos;
            let dist = dot(diff, diff);
            if id != test_id && dist < best_dist {
                best_dist = dist;
                best_pos = test_value;
            }
        }
    }

    return select(value, best_pos, best_dist != starting_dist);
}
