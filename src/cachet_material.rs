use bevy::{
    asset::Asset,
    color::LinearRgba,
    prelude::{Handle, Image},
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
pub struct CachetMaterial {
    // Uniform bindings must implement `ShaderType`, which will be used to convert the value to
    // its shader-compatible equivalent. Most core math types already implement `ShaderType`.
    #[uniform(0)]
    pub color: LinearRgba,
    // Images can be bound as textures in shaders. If the Image's sampler is also needed, just
    // add the sampler attribute with a different binding index.
    #[texture(1)]
    #[sampler(2)]
    pub color_texture: Option<Handle<Image>>,
}

// All functions on `Material2d` have default impls. You only need to implement the
// functions that are relevant for your material.
impl Material2d for CachetMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/cachet_material.wgsl".into()
    }
}
