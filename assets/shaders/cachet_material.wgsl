#import bevy_sprite::mesh2d_vertex_output::VertexOutput

struct TeamMaterial {
    color: vec4<f32>,
}

@group(2) @binding(0)
var<uniform> material: TeamMaterial;
@group(2) @binding(1)
var color_texture: texture_2d<f32>;
@group(2) @binding(2)
var color_sampler: sampler;


@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    let texture_color = textureSample(color_texture, color_sampler, mesh.uv);
    let ratio = 0.5;
    return  texture_color *  material.color ;
}