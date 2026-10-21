import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput;
import super::outline_common::{null_pos, uv_to_pos};

@group(0) @binding(0)
var depth_texture: texture_depth_2d;

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4u {
    let pos = uv_to_pos(in.uv, textureDimensions(depth_texture));
    var depth = textureLoad(depth_texture, pos, 0);
    depth = depth;
    let out = select(pos, null_pos, depth < 0.0001);
    return vec4(out, 0, 1);
}
