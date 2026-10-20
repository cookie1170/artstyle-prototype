import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput;
import super::outline_common::null_pos;

@group(0) @binding(0)
var texture: texture_2d<f32>;

@group(0) @binding(1)
var<uniform> step_width: i32;

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4f {
    let dim = textureDimensions(texture);
    let scaled_pos = in.uv * vec2f(dim);
    let pos = vec2i(scaled_pos);
    let current_pos = textureLoad(texture, pos, 0);
    // only visualise the pixels added by the flood
    if all(current_pos.xy != null_pos) {
        return vec4(0);
    }

    // big number
    var best_dist = 10e7;
    var best_pos = null_pos;
    for(var u = -1; u <= 1; u += 1) {
        for(var v = -1; v <= 1; v += 1) {
            let offset_pos = pos + vec2(u, v) * step_width;
            let sample_pos = clamp(offset_pos, vec2(0), vec2i(dim));
            let test_pos = textureLoad(texture, sample_pos, 0).rg * vec2f(dim);
            let diff = scaled_pos - test_pos;
            let sqr_dist = dot(diff, diff);
            if all(test_pos != null_pos) && sqr_dist < best_dist {
                best_pos = test_pos;
                best_dist = sqr_dist;
            }
        }
    }

    return vec4(best_pos / vec2f(dim), 0, 0);
}
