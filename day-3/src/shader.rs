pub const VIDEO_SHADER: &str = r#"
// Vertex shader
struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) tex_coords: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
};

@vertex
fn vs_main(vert: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(vert.position, 0.0, 1.0);
    out.tex_coords = vert.tex_coords;
    return out;
}

// Fragment shader
@group(0) @binding(0) var video_texture: texture_2d<f32>;
@group(0) @binding(1) var s: sampler;
@group(0) @binding(2) var<uniform> time: f32;

// Determine which Fibonacci square we're in and return local UV coordinates
fn get_fibonacci_square(uv: vec2<f32>) -> vec2<f32> {
    // Normalize coordinates
    let total_size = 13.0; // Fibonacci(7) for layout size
    var x = uv.x * total_size;
    var y = uv.y * total_size;

    // Layout of Fibonacci squares (spiral pattern)
    // Starting with largest square and working down

    // Square 8x8 (bottom-right)
    if (x >= 5.0 && y < 8.0) {
        let local_x = (x - 5.0) / 8.0;
        let local_y = y / 8.0;
        return vec2<f32>(local_x, local_y);
    }

    // Square 5x5 (top-right)
    if (x >= 5.0 && y >= 8.0) {
        let local_x = (x - 5.0) / 5.0;
        let local_y = (y - 8.0) / 5.0;
        return vec2<f32>(local_x, local_y);
    }

    // Square 3x3 (bottom-left quadrant, top part)
    if (x < 5.0 && x >= 2.0 && y >= 5.0 && y < 8.0) {
        let local_x = (x - 2.0) / 3.0;
        let local_y = (y - 5.0) / 3.0;
        return vec2<f32>(local_x, local_y);
    }

    // Square 5x5 (left side)
    if (x < 2.0 && y >= 3.0 && y < 8.0) {
        let local_x = x / 2.0;
        let local_y = (y - 3.0) / 5.0;
        return vec2<f32>(local_x, local_y);
    }

    // Square 3x3 (bottom-left)
    if (x >= 2.0 && x < 5.0 && y >= 2.0 && y < 5.0) {
        let local_x = (x - 2.0) / 3.0;
        let local_y = (y - 2.0) / 3.0;
        return vec2<f32>(local_x, local_y);
    }

    // Square 2x2
    if (x < 2.0 && y >= 1.0 && y < 3.0) {
        let local_x = x / 2.0;
        let local_y = (y - 1.0) / 2.0;
        return vec2<f32>(local_x, local_y);
    }

    // Square 1x1 (top)
    if (x < 1.0 && y < 1.0) {
        return vec2<f32>(x, y);
    }

    // Square 1x1 (bottom)
    if (x >= 1.0 && x < 2.0 && y < 1.0) {
        return vec2<f32>(x - 1.0, y);
    }

    // Square 2x2 (bottom)
    if (x >= 2.0 && x < 4.0 && y < 2.0) {
        let local_x = (x - 2.0) / 2.0;
        let local_y = y / 2.0;
        return vec2<f32>(local_x, local_y);
    }

    // Default fallback
    return uv;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = in.tex_coords;

    // Get local UV coordinates within the Fibonacci square
    let local_uv = get_fibonacci_square(uv);

    // Sample the video texture using the local coordinates
    // This renders the full video in each square
    let color = textureSample(video_texture, s, local_uv);

    return vec4<f32>(color.rgb, 1.0);
}
"#;
