import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput;
import super::outline_common::{is_null, null_pos, unpack_depth, uv_to_pos};

@group(0) @binding(0)
var texture: texture_2d<u32>;

@group(0) @binding(1)
var screen_texture: texture_2d<f32>;

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4f {
    let dim = textureDimensions(texture);
    let pos = uv_to_pos(in.uv, dim);
    let nearest = textureLoad(texture, pos, 0);
    let dist = distance(vec2f(nearest.xy), vec2f(pos));
    if dist < 4.0 && dist > 0.0 {
        return vec4(0);
    } else {
        return textureLoad(screen_texture, pos, 0);
    }
}
