const null_pos: vec2u = vec2(0);
const u32_max = 4294967295; // 2 ** 32 - 1

fn uv_to_pos(uv: vec2f, dim: vec2u) -> vec2u {
    return vec2u(uv * vec2f(dim));
}

fn is_null(pos: vec2u) -> bool {
    return all(pos == null_pos);
}

/// packs the `depth` value into two `u32`s ranging from `0` to `65535`
fn pack_depth(depth: f32) -> vec2u {
    let as_u32 = u32(depth * u32_max);
    let bottom = as_u32 & 0xffff;
    let top = (as_u32 & 0xffff0000) >> 16;
    return vec2(bottom, top);
}

/// unpacks the `packed_depth` value from two `u32`s ranging from `0` to `65535`
fn unpack_depth(packed_depth: vec2u) -> f32 {
    let bottom = packed_depth.x;
    let top = packed_depth.y << 16;
    let combined = bottom | top;
    return f32(combined) / u32_max;
}
