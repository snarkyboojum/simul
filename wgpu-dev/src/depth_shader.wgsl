//
// Depth shader
// 

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {

    var out: VertexOutput;

    out.tex_coords = model.tex_coords;
    out.clip_position = vec4<f32>(model.position, 1.0);

    return out;
}

@group(0)@binding(0)
// var t_depth: texture_2d<f32>;
var t_depth: texture_depth_2d;
@group(0)@binding(1)
var s_depth: sampler;
// var s_depth: sampler_comparison;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {

    // TODO: I don't know what offset does or why it's required for a texture_depth_2d type
    //       See, https://www.w3.org/TR/WGSL/#texturesample
    let value = textureSample(t_depth, s_depth, in.tex_coords, vec2<i32>(0,0));
    let pixel = vec4<f32>(value, value, value, 1);

    return pixel;
}
