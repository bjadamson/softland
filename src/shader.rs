use gfx;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

pub type OutColor<R: gfx::Resources> = gfx::handle::RenderTargetView<R,
                                                                     (gfx::format::R8_G8_B8_A8,
                                                                      gfx::format::Unorm)>;
pub type OutDepth<R: gfx::Resources> = gfx::handle::DepthStencilView<R, DepthFormat>;


gfx_defines!{
    vertex Vertex {
        pos: [f32; 4] = "a_pos",
        color: [f32; 4] = "a_color",
        normal: [f32; 3] = "a_normal",
    }

    constant Locals {
        model: [[f32; 4]; 4] = "u_model",
        ambient: [f32; 4] = "u_ambient",
        lightcolor: [f32; 4] = "u_lightcolor",
        lightpos: [f32; 3] = "u_lightpos",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        locals: gfx::ConstantBuffer<Locals> = "Locals",
        model: gfx::Global<[[f32; 4]; 4]> = "u_model",
        ambient: gfx::Global<[f32; 4]> = "u_ambient",
        lightpos: gfx::Global<[f32; 3]> = "u_lightpos",
        lightcolor: gfx::Global<[f32; 4]> = "u_lightcolor",
        out: gfx::RenderTarget<ColorFormat> = "target_0",
        depth: gfx::DepthTarget<DepthFormat> = gfx::state::Depth {
            fun: gfx::state::Comparison::Less,
            write: true,
        },
    }
}

pub const SHADER_V: &[u8] = include_bytes!("shader/triangle.glslv");
pub const SHADER_F: &[u8] = include_bytes!("shader/triangle.glslf");
