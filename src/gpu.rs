use cgmath::*;

use gfx;
use gfx::traits::FactoryExt;

use shader;
use shader::*;
use std::marker::PhantomData;

pub struct PsoFactory<'a, R, F>
    where R: gfx::Resources,
          F: gfx::Factory<R> + 'a
{
    factory: &'a mut F,
    phantom: PhantomData<R>,
}

macro_rules! triangle_strip {
        ($vshader:ident, $fshader:ident, $factory:ident, $pipe:ident) => ({
            let set = $factory.create_shader_set($vshader, $fshader).unwrap();
            let primitive = gfx::Primitive::TriangleStrip;
            let rasterizer = gfx::state::Rasterizer::new_fill().with_cull_back();
            $factory
                .create_pipeline_state(&set, primitive, rasterizer, $pipe)
                .unwrap()
        })
    }

macro_rules! triangle_list {
        ($vshader:ident, $fshader:ident, $factory:ident, $pipe:ident) => ({
            let set = $factory.create_shader_set($vshader, $fshader).unwrap();
            let primitive = gfx::Primitive::TriangleList;
            let rasterizer = gfx::state::Rasterizer::new_fill().with_cull_back();
            $factory
                .create_pipeline_state(&set, primitive, rasterizer, $pipe)
                .unwrap()
            })
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

    pub fn triangle_strip_colors(&mut self) -> gfx::PipelineState<R, ColorPipe::Meta> {
        let pipe = ColorPipe::new();
        let factory = &mut self.factory;
        triangle_strip!(COLOR_CUBE_SHADER_V, COLOR_CUBE_SHADER_F, factory, pipe)
    }

    pub fn triangle_list_colors(&mut self) -> gfx::PipelineState<R, ColorPipe::Meta> {
        let pipe = ColorPipe::new();
        let factory = &mut self.factory;
        triangle_list!(COLOR_CUBE_SHADER_V, COLOR_CUBE_SHADER_F, factory, pipe)
    }

    pub fn triangle_strip_uv(&mut self) -> gfx::PipelineState<R, UvPipe::Meta> {
        let pipe = UvPipe::new();
        let factory = &mut self.factory;
        triangle_strip!(UV_CUBE_SHADER_V, UV_CUBE_SHADER_F, factory, pipe)
    }

    pub fn triangle_list_uv(&mut self) -> gfx::PipelineState<R, UvPipe::Meta> {
        let pipe = UvPipe::new();
        let factory = &mut self.factory;
        triangle_list!(UV_CUBE_SHADER_V, UV_CUBE_SHADER_F, factory, pipe)
    }
}
