struct OutlineConfig {
    width: f32,
}

@group(1) @binding(0)
var<uniform> config: OutlineConfig;

fn uv_to_pos(uv: vec2f, dim: vec2u) -> vec2u {
    return vec2u(uv * vec2f(dim));
}

fn pack_id(id: u32) -> vec2u {
    let top = (id & 0xffff0000) >> 16;
    let bottom = id & 0xffff;
    return vec2(top, bottom);
}

fn unpack_id(id: vec2u) -> u32 {
    let top = id.x << 16;
    let bottom = id.y;
    return top | bottom;
}
