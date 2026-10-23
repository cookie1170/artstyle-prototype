import bevy_pbr::render::forward_io::VertexOutput;

@group(constants::MATERIAL_BIND_GROUP) @binding(0)
var<uniform> id: u32;

@fragment
fn fragment() -> @location(0) vec4<f32> {
    // the mesh pipeline doesn't support the `Rgba8Uint` texture format,
    // so instead we use `Rgba8Unorm`, which means we need to convert this to 4 floats using the unpacking function.
    return unpack4x8unorm(id);
}
