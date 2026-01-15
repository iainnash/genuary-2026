pub const DIFFERENCE_SHADER: &str = r#"
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

// Fragment shader - simple passthrough
@group(0) @binding(0) var current_texture: texture_2d<f32>;
@group(0) @binding(1) var s: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Simple passthrough - just sample and return the current texture
    let color = textureSample(current_texture, s, in.tex_coords);
    return vec4<f32>(color.rgb, 1.0);
}
"#;
