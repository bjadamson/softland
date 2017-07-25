use gfx;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

pub type OutColor<R: gfx::Resources> = gfx::handle::RenderTargetView<R,
                                                                     (gfx::format::R8_G8_B8_A8,
                                                                      gfx::format::Unorm)>;
pub type OutDepth<R: gfx::Resources> = gfx::handle::DepthStencilView<R, DepthFormat>;

pub struct CubeTextureData<R: gfx::Resources> {
    pub front: gfx::handle::ShaderResourceView<R, [f32; 4]>,
    pub back: gfx::handle::ShaderResourceView<R, [f32; 4]>,
    pub top: gfx::handle::ShaderResourceView<R, [f32; 4]>,
    pub bottom: gfx::handle::ShaderResourceView<R, [f32; 4]>,
    pub left: gfx::handle::ShaderResourceView<R, [f32; 4]>,
    pub right: gfx::handle::ShaderResourceView<R, [f32; 4]>,
}

struct CubeSideFront {}
struct CubeSideBack {}
struct CubeSideTop {}
struct CubeSideBottom {}
struct CubeSideLeft {}
struct CubeSideRight {}

enum CubeVertices {
    Front(CubeSideFront),
    Back(CubeSideBack),
    Top(CubeSideTop),
    Bottom(CubeSideBottom),
    Left(CubeSideLeft),
    Right(CubeSideRight),
}

gfx_defines!{
    vertex ColorVertex {
        pos: [f32; 4] = "a_pos",
        color: [f32; 4] = "a_color",
        normal: [f32; 3] = "a_normal",
    }

    vertex UvVertex {
        pos: [f32; 4] = "a_pos",
        normal: [f32; 3] = "a_normal",
        uv: [f32; 2] = "a_uv",
    }

    constant Locals {
        model: [[f32; 4]; 4] = "u_model",
        ambient: [f32; 4] = "u_ambient",
        lightcolor: [f32; 4] = "u_lightcolor",

        viewpos: [f32; 3] = "u_viewpos",
        lightpos: [f32; 3] = "u_lightpos",
    }

    pipeline ColorPipe {
        vbuf: gfx::VertexBuffer<ColorVertex> = (),
        locals: gfx::ConstantBuffer<Locals> = "Locals",
        model: gfx::Global<[[f32; 4]; 4]> = "u_model",
        ambient: gfx::Global<[f32; 4]> = "u_ambient",
        lightcolor: gfx::Global<[f32; 4]> = "u_lightcolor",

        viewpos: gfx::Global<[f32; 3]> = "u_viewpos",
        lightpos: gfx::Global<[f32; 3]> = "u_lightpos",
        out: gfx::RenderTarget<ColorFormat> = "target_0",
        depth: gfx::DepthTarget<DepthFormat> = gfx::state::Depth {
            fun: gfx::state::Comparison::Less,
            write: true,
        },
    }

    pipeline UvPipe {
        vbuf: gfx::VertexBuffer<UvVertex> = (),
        locals: gfx::ConstantBuffer<Locals> = "Locals",
        uv_front: gfx::TextureSampler<[f32; 4]> = "uv_front",
        uv_back: gfx::TextureSampler<[f32; 4]> = "uv_back",
        uv_top: gfx::TextureSampler<[f32; 4]> = "uv_top",
        uv_bottom: gfx::TextureSampler<[f32; 4]> = "uv_bottom",
        uv_left: gfx::TextureSampler<[f32; 4]> = "uv_left",
        uv_right: gfx::TextureSampler<[f32; 4]> = "uv_right",

        model: gfx::Global<[[f32; 4]; 4]> = "u_model",
        ambient: gfx::Global<[f32; 4]> = "u_ambient",
        lightcolor: gfx::Global<[f32; 4]> = "u_lightcolor",

        viewpos: gfx::Global<[f32; 3]> = "u_viewpos",
        lightpos: gfx::Global<[f32; 3]> = "u_lightpos",
        out: gfx::RenderTarget<ColorFormat> = "target_0",
        depth: gfx::DepthTarget<DepthFormat> = gfx::state::Depth {
            fun: gfx::state::Comparison::Less,
            write: true,
        },
    }
}

pub const COLOR_CUBE_SHADER_V: &[u8] = include_bytes!("shader/cube_color.glslv");
pub const COLOR_CUBE_SHADER_F: &[u8] = include_bytes!("shader/cube_color.glslf");

pub const UV_CUBE_SHADER_V: &[u8] = include_bytes!("shader/cube_uv.glslv");
pub const UV_CUBE_SHADER_F: &[u8] = include_bytes!("shader/cube_uv.glslf");
