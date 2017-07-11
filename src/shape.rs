use gfx;
use gfx::traits::FactoryExt;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

gfx_defines!{
    vertex Vertex {
        pos: [f32; 4] = "a_pos",
        color: [f32; 4] = "a_color",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        out: gfx::RenderTarget<ColorFormat> = "target_0",
    }
}

const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
const PURPLE: [f32; 4] = [1.0, 0.0, 1.0, 1.0];
const YELLOW: [f32; 4] = [1.0, 1.0, 0.0, 1.0];
const BLUE_GREEN: [f32; 4] = [0.0, 1.0, 1.0, 1.0];

const SHADER_V: &[u8] = include_bytes!("shader/triangle_150.glslv");
const SHADER_F: &[u8] = include_bytes!("shader/triangle_150.glslf");

fn calculate_cube_vertices(dimensions: (f32, f32, f32)) -> [[f32; 4]; 8] {
    let (w, h, l) = dimensions;

    [[-w, -h, l,  1.0], // front bottom-left
    [w,   -h, l,  1.0], // front bottom-right
    [w,   h, l,   1.0], // front top-right
    [-w,  h, l,   1.0], // front top-left

    [-w,  -h, -l, 1.0], // back bottom-left
    [w,   -h, -l, 1.0], // back bottom-right
    [w,   h, -l,  1.0], // back top-right
    [-w,  h,  -l, 1.0]] // back top-left
}

fn construct_cube(dimensions: (f32, f32, f32), colors: &[[f32; 4]; 8]) -> ([Vertex; 8], &[u16]) {
    let vertices = calculate_cube_vertices(dimensions);
    let a = Vertex {
        pos: [vertices[2][0], vertices[2][1], vertices[2][2], vertices[2][3]],
        color: colors[0]
    };
    let b = Vertex {
        pos: [vertices[3][0], vertices[3][1], vertices[3][2], vertices[3][3]],
        color: colors[1]
    };
    let c = Vertex {
        pos: [vertices[6][0], vertices[6][1], vertices[6][2], vertices[6][3]],
        color: colors[2]
    };
    let d = Vertex {
        pos: [vertices[7][0], vertices[7][1], vertices[7][2], vertices[7][3]],
        color: colors[3]
    };
    let e = Vertex {
        pos: [vertices[1][0], vertices[1][1], vertices[1][2], vertices[1][3]],
        color: colors[4]
    };
    let f = Vertex {
        pos: [vertices[0][0], vertices[0][1], vertices[0][2], vertices[0][3]],
        color: colors[5]
    };
    let g = Vertex {
        pos: [vertices[4][0], vertices[4][1], vertices[4][2], vertices[4][3]],
        color: colors[6]
    };
    let h = Vertex {
        pos: [vertices[5][0], vertices[5][1], vertices[5][2], vertices[5][3]],
        color: colors[7]
    };
    const INDICES: &[u16] = &[3, 2, 6, 7, 4, 2, 0, 3, 1, 6, 5, 4, 1, 0];
    ([a, b, c, d, e, f, g, h], &INDICES)
}

fn make_triangle2d(length: f32, colors: &[[f32; 4]; 3]) -> [Vertex; 3] {
    let vertices = [[-length, -length, 0.0, 1.0], [length, -length, 0.0, 1.0], [0.0, length, 0.0, 1.0]];
    let a = Vertex {
        pos: [vertices[0][0], vertices[0][1], vertices[0][2], vertices[0][3]],
        color: colors[0]
    };
    let b = Vertex {
        pos: [vertices[1][0], vertices[1][1], vertices[1][2], vertices[1][3]],
        color: colors[1]
    };
    let c = Vertex {
        pos: [vertices[2][0], vertices[2][1], vertices[2][2], vertices[2][3]],
        color: colors[2]
    };
    [a, b, c]
}

pub fn make_cube<R: gfx::Resources, F: gfx::Factory<R>>(factory: &mut F, out_color: &gfx::handle::RenderTargetView<R, (gfx::format::R8_G8_B8_A8, gfx::format::Unorm)>)
        -> (gfx::Slice<R>, gfx::PipelineState<R, pipe::Meta>, pipe::Data<R>) {
    let cube_set = factory.create_shader_set(SHADER_V, SHADER_F).unwrap();
    
    let pso = factory.create_pipeline_state(&cube_set, gfx::Primitive::TriangleStrip, gfx::state::Rasterizer::new_fill(),
        pipe::new()).unwrap();

    let colors: [[f32; 4]; 8] = [RED, GREEN, BLUE, PURPLE, YELLOW, BLUE_GREEN, RED, GREEN];
    let (vertices, indices) = construct_cube((0.25, 0.25, 0.25), &colors);

    let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&vertices, indices);
    let data = pipe::Data {
        vbuf: vertex_buffer,
        out: out_color.clone()
    };
    (slice, pso, data)
}
                 
pub fn make_triangle<R: gfx::Resources, F: gfx::Factory<R>>(factory: &mut F, out_color: &gfx::handle::RenderTargetView<R, (gfx::format::R8_G8_B8_A8, gfx::format::Unorm)>)
        -> (gfx::Slice<R>, gfx::PipelineState<R, pipe::Meta>, pipe::Data<R>) {
    let colors: [[f32; 4]; 3] = [BLUE, YELLOW, PURPLE];
    let vertices = make_triangle2d(0.35, &colors);
    let indices = ();

    let pso = factory.create_pipeline_simple(SHADER_V, SHADER_F, pipe::new()).unwrap();
    let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&vertices, indices);
    let data = pipe::Data {
        vbuf: vertex_buffer,
        out: out_color.clone()
    };
    (slice, pso, data)
}
