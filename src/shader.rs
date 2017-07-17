use gfx;

gfx_defines!{
    vertex Vertex {
        pos: [f32; 4] = "a_pos",
        color: [f32; 4] = "a_color",
    }

    constant Locals {
        model: [[f32; 4]; 4] = "u_model",
        ambient: [f32; 4] = "u_ambient",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        locals: gfx::ConstantBuffer<Locals> = "Locals",
        model: gfx::Global<[[f32; 4]; 4]> = "u_model",
        ambient: gfx::Global<[f32; 4]> = "u_ambient",
        out: gfx::RenderTarget<gfx::format::Rgba8> = "target_0",
    }
}

pub const SHADER_V: &[u8] = include_bytes!("shader/triangle.glslv");
pub const SHADER_F: &[u8] = include_bytes!("shader/triangle.glslf");
