use cgmath::*;

use gfx;
use gfx::traits::FactoryExt;

use shape;
use shader;
use shader::{SHADER_V, SHADER_F, Vertex, Locals, pipe};
use std::marker::PhantomData;

fn construct_cube<'a>(dimensions: &'a (f32, f32, f32),
                      colors: &[[f32; 4]; 8])
                      -> ([Vertex; 8], &'a [u16]) {
    let vertices = shape::make_cube_vertices(dimensions);
    let a = Vertex {
        pos: [vertices[2][0], vertices[2][1], vertices[2][2], vertices[2][3]],
        color: colors[0],
    };
    let b = Vertex {
        pos: [vertices[3][0], vertices[3][1], vertices[3][2], vertices[3][3]],
        color: colors[1],
    };
    let c = Vertex {
        pos: [vertices[6][0], vertices[6][1], vertices[6][2], vertices[6][3]],
        color: colors[2],
    };
    let d = Vertex {
        pos: [vertices[7][0], vertices[7][1], vertices[7][2], vertices[7][3]],
        color: colors[3],
    };
    let e = Vertex {
        pos: [vertices[1][0], vertices[1][1], vertices[1][2], vertices[1][3]],
        color: colors[4],
    };
    let f = Vertex {
        pos: [vertices[0][0], vertices[0][1], vertices[0][2], vertices[0][3]],
        color: colors[5],
    };
    let g = Vertex {
        pos: [vertices[4][0], vertices[4][1], vertices[4][2], vertices[4][3]],
        color: colors[6],
    };
    let h = Vertex {
        pos: [vertices[5][0], vertices[5][1], vertices[5][2], vertices[5][3]],
        color: colors[7],
    };
    const INDICES: &[u16] = &[3, 2, 6, 7, 4, 2, 0, 3, 1, 6, 5, 4, 1, 0];
    ([a, b, c, d, e, f, g, h], &INDICES)
}

fn make_triangle2d(length: f32, colors: &[[f32; 4]; 3]) -> [Vertex; 3] {
    let vertices = shape::make_triangle_vertices(length);
    let a = Vertex {
        pos: [vertices[0][0], vertices[0][1], vertices[0][2], vertices[0][3]],
        color: colors[0],
    };
    let b = Vertex {
        pos: [vertices[1][0], vertices[1][1], vertices[1][2], vertices[1][3]],
        color: colors[1],
    };
    let c = Vertex {
        pos: [vertices[2][0], vertices[2][1], vertices[2][2], vertices[2][3]],
        color: colors[2],
    };
    [a, b, c]
}

type OutColor<R: gfx::Resources> = gfx::handle::RenderTargetView<R,
                                                                 (gfx::format::R8_G8_B8_A8,
                                                                  gfx::format::Unorm)>;

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

macro_rules! copy_vertices {
    ($factory:ident, $encoder:ident, $out_color:ident, $pso:ident, $model_m:ident, $vertices:ident, $indices:ident) => {{
        let (vertex_buffer, slice) = $factory.create_vertex_buffer_with_slice(&$vertices, $indices);
        let data = pipe::Data {
            vbuf: vertex_buffer,
            locals: $factory.create_constant_buffer(1),
            model: $model_m.into(),
            out: $out_color.clone()
        };
        let locals = Locals {
            model: data.model
        };
        $encoder.update_buffer(&data.locals, &[locals], 0).unwrap();
        $encoder.draw(&slice, &$pso, &data);
    }};
}

pub struct Gpu<'a, R, F, C>
    where R: gfx::Resources,
          F: gfx::Factory<R> + 'a,
          C: gfx::CommandBuffer<R> + 'a
{
    factory: &'a mut F,
    encoder: &'a mut gfx::Encoder<R, C>,
    out_color: &'a OutColor<R>,
}

impl<'z, R, F, C> Gpu<'z, R, F, C>
    where R: gfx::Resources,
          F: gfx::Factory<R> + 'z,
          C: gfx::CommandBuffer<R> + 'z
{
    pub fn new(f: &'z mut F,
               e: &'z mut gfx::Encoder<R, C>,
               out_color: &'z OutColor<R>)
               -> Gpu<'z, R, F, C> {
        Gpu {
            factory: f,
            encoder: e,
            out_color: out_color,
        }
    }

    pub fn draw_cube(&mut self,
                     pso: &gfx::PipelineState<R, pipe::Meta>,
                     dimensions: &(f32, f32, f32),
                     colors: &[[f32; 4]; 8],
                     model_m: Matrix4<f32>) {
        let (vertices, indices) = construct_cube(dimensions, &colors);

        let factory = &mut self.factory;
        let encoder = &mut self.encoder;
        let out_color = &mut self.out_color;

        copy_vertices!(factory, encoder, out_color, pso, model_m, vertices, indices)
    }

    pub fn draw_triangle(&mut self,
                         pso: &gfx::PipelineState<R, pipe::Meta>,
                         radius: f32,
                         colors: &[[f32; 4]; 3],
                         model_m: Matrix4<f32>) {
        let vertices = make_triangle2d(radius, &colors);
        let indices = ();

        let factory = &mut self.factory;
        let encoder = &mut self.encoder;
        let out_color = &mut self.out_color;

        copy_vertices!(factory, encoder, out_color, pso, model_m, vertices, indices)
    }

    pub fn draw_triangle_from_vertices(&mut self,
                                       pso: &gfx::PipelineState<R, pipe::Meta>,
                                       vertices: &[shader::Vertex],
                                       indices: &[u32],
                                       model_m: Matrix4<f32>) {
        let factory = &mut self.factory;
        let encoder = &mut self.encoder;
        let out_color = &mut self.out_color;

        copy_vertices!(factory, encoder, out_color, pso, model_m, vertices, indices)
    }
}
