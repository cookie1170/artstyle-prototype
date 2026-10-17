import bevy_pbr::render::{mesh_functions as mesh, forward_io::{Vertex, decompress_vertex}};

struct VertexOutput {
    @builtin(position) position: vec4f,
}

@vertex
fn vertex(in: Vertex) -> VertexOutput {
    let i = in.instance_index;
    let vertex = decompress_vertex(in, i);

    let normal = vertex.normal;
    var pos = vertex.position;
    pos *= 1.05;
    let world_from_local = mesh::get_world_from_local(i);
    let clip_pos = mesh::mesh_position_local_to_clip(world_from_local, vec4(pos, 1));
    var out: VertexOutput;
    out.position = clip_pos;

    return out;
}

@fragment
fn fragment() -> @location(0) vec4f {
    return vec4(vec3(0), 1);
}
