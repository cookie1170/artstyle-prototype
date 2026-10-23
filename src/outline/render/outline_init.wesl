import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput;
import super::outline_common::{null_pos, pack_depth, uv_to_pos};

@group(0) @binding(0)
var depth_texture: texture_depth_2d;

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4u {
    let pos = uv_to_pos(in.uv, textureDimensions(depth_texture));
    let depth = textureLoad(depth_texture, pos, 0);
    let out = select(pos, null_pos, depth < 0.0001);
    // pack the depth into the `ba` chanels
    let depth_packed = pack_depth(depth);

    return vec4(out, depth_packed);
}
