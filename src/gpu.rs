use cgmath::*;

use gfx;
use gfx::traits::FactoryExt;

use shader;
use shader::{SHADER_V, SHADER_F, Vertex, Locals, pipe};
use std::marker::PhantomData;

pub struct PsoFactory<'a, R, F>
    where R: gfx::Resources,
          F: gfx::Factory<R> + 'a
{
    factory: &'a mut F,
    phantom: PhantomData<R>,
}

impl<'a, R, F> PsoFactory<'a, R, F>
    where R: gfx::Resources,
          F: gfx::Factory<R> + 'a
{
    pub fn new(factory: &'a mut F) -> PsoFactory<'a, R, F> {
        PsoFactory {
            factory: factory,
            phantom: PhantomData,
        }
    }

    pub fn triangle_strip(&mut self) -> gfx::PipelineState<R, pipe::Meta> {
        let set = self.factory.create_shader_set(SHADER_V, SHADER_F).unwrap();
        let primitive = gfx::Primitive::TriangleStrip;
        let rasterizer = gfx::state::Rasterizer::new_fill().with_cull_back();
        let pipe = pipe::new();
        self.factory
            .create_pipeline_state(&set, primitive, rasterizer, pipe)
            .unwrap()
    }

    pub fn triangle_list(&mut self) -> gfx::PipelineState<R, pipe::Meta> {
        let set = self.factory.create_shader_set(SHADER_V, SHADER_F).unwrap();
        let primitive = gfx::Primitive::TriangleList;
        let rasterizer = gfx::state::Rasterizer::new_fill().with_cull_back();
        let pipe = pipe::new();
        self.factory
            .create_pipeline_state(&set, primitive, rasterizer, pipe)
            .unwrap()
    }
}
