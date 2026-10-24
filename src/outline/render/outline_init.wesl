import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput;
import super::outline_common::{pack_id, uv_to_pos};

@group(0) @binding(0)
var ids: texture_2d<f32>;

@group(0) @binding(1)
var<uniform> dim: vec2u;

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4u {
    let pos = uv_to_pos(in.uv, dim);
    return vec4(pos, pack_id(pack4x8unorm(textureLoad(ids, pos, 0))));
}
