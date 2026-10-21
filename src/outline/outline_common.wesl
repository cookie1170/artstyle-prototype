const null_pos: vec2u = vec2(0);

fn uv_to_pos(uv: vec2f, dim: vec2u) -> vec2u {
    return vec2u(uv * vec2f(dim));
}
