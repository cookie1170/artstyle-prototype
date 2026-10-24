struct OutlineConfig {
    width: f32,
}

@group(0) @binding(3)
var<uniform> config: OutlineConfig;

fn uv_to_pos(uv: vec2f, dim: vec2u) -> vec2u {
    return vec2u(uv * vec2f(dim));
}
