import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput;
import super::outline_common::null_pos;

@group(0) @binding(0)
var depth_texture: texture_depth_2d;

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4f {
    let pos = vec2u(vec2f(textureDimensions(depth_texture)) * in.uv);
    var depth = textureLoad(depth_texture, pos, 0);
    depth = depth;
    let out = select(in.uv.rg, null_pos, depth < 0.0001);
    return vec4(vec3(out, 0), 1);
}
