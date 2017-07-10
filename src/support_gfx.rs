use gfx;
use gfx::Device;
use gfx::traits::FactoryExt;
use gfx_window_glutin;
use glutin;
use glutin::{ElementState, MouseButton, MouseScrollDelta, VirtualKeyCode, TouchPhase, WindowEvent};
use imgui::{ImGui, Ui, ImGuiKey};
use imgui_gfx_renderer::Renderer;
use std::time::Instant;

use state::State;

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

#[derive(Copy, Clone, PartialEq, Debug, Default)]
struct MouseState {
    pos: (i32, i32),
    pressed: (bool, bool, bool),
    wheel: f32
}

macro_rules! process_event {
    ($event:ident, $imgui:ident, $window:ident, $renderer:ident, $mouse_state:ident, $game:ident, $main_color:ident, $main_depth:ident) => (
        match $event {
            WindowEvent::Resized(_, _) => {
                gfx_window_glutin::update_views(&$window, &mut $main_color, &mut $main_depth);
                $renderer.update_render_target($main_color.clone());
            }
            WindowEvent::Closed => $game.quit = true,
            WindowEvent::KeyboardInput(state, _, code, _) => {
                let pressed = state == ElementState::Pressed;
                match code {
                    Some(VirtualKeyCode::Tab) => $imgui.set_key(0, pressed),
                    Some(VirtualKeyCode::Left) => $imgui.set_key(1, pressed),
                    Some(VirtualKeyCode::Right) => $imgui.set_key(2, pressed),
                    Some(VirtualKeyCode::Up) => $imgui.set_key(3, pressed),
                    Some(VirtualKeyCode::Down) => $imgui.set_key(4, pressed),
                    Some(VirtualKeyCode::PageUp) => $imgui.set_key(5, pressed),
                    Some(VirtualKeyCode::PageDown) => $imgui.set_key(6, pressed),
                    Some(VirtualKeyCode::Home) => $imgui.set_key(7, pressed),
                    Some(VirtualKeyCode::End) => $imgui.set_key(8, pressed),
                    Some(VirtualKeyCode::Delete) => $imgui.set_key(9, pressed),
                    Some(VirtualKeyCode::Back) => $imgui.set_key(10, pressed),
                    Some(VirtualKeyCode::Return) => {
                        // 1. Tell imgui the key was pressed.
                        $imgui.set_key(11, pressed);

                        // 2. Update our state w/regard to chat input.
                        $game.chat_window_state.user_editing = state == ElementState::Released;
                    },
                    Some(VirtualKeyCode::Escape) => $game.quit = true,
                    Some(VirtualKeyCode::A) => $imgui.set_key(13, pressed),
                    Some(VirtualKeyCode::C) => $imgui.set_key(14, pressed),
                    Some(VirtualKeyCode::V) => $imgui.set_key(15, pressed),
                    Some(VirtualKeyCode::X) => $imgui.set_key(16, pressed),
                    Some(VirtualKeyCode::Y) => $imgui.set_key(17, pressed),
                    Some(VirtualKeyCode::Z) => $imgui.set_key(18, pressed),
                    Some(VirtualKeyCode::LControl) |
                    Some(VirtualKeyCode::RControl) => $imgui.set_key_ctrl(pressed),
                    Some(VirtualKeyCode::LShift) |
                    Some(VirtualKeyCode::RShift) => $imgui.set_key_shift(pressed),
                    Some(VirtualKeyCode::LAlt) |
                    Some(VirtualKeyCode::RAlt) => $imgui.set_key_alt(pressed),
                    Some(VirtualKeyCode::LWin) |
                    Some(VirtualKeyCode::RWin) => $imgui.set_key_super(pressed),
                    _ => {}
                }
            }
            WindowEvent::MouseMoved(x, y) => $mouse_state.pos = (x, y),
            WindowEvent::MouseInput(state, MouseButton::Left) => {
                $mouse_state.pressed.0 = state == ElementState::Pressed
            }
            WindowEvent::MouseInput(state, MouseButton::Right) => {
                $mouse_state.pressed.1 = state == ElementState::Pressed
            }
            WindowEvent::MouseInput(state, MouseButton::Middle) => {
                $mouse_state.pressed.2 = state == ElementState::Pressed
            }
            WindowEvent::MouseWheel(MouseScrollDelta::LineDelta(_, y), TouchPhase::Moved) => {
                $mouse_state.wheel = y
            }
            WindowEvent::MouseWheel(MouseScrollDelta::PixelDelta(_, y), TouchPhase::Moved) => {
                $mouse_state.wheel = y
            }
            WindowEvent::ReceivedCharacter(c) => $imgui.add_input_character(c),
            _ => ()
        }
    )
}

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

fn make_cube(dimensions: (f32, f32, f32), colors: &[[f32; 4]; 8]) -> ([Vertex; 8], &[u16]) {
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

pub fn run<F: FnMut(&Ui, &mut State)>(title: &str, clear_color: [f32; 4], game: &mut State, mut render_ui: F) {
    let mut imgui = ImGui::init();

    let (w, h) = game.window_dimensions;
    let events_loop = glutin::EventsLoop::new();
    let builder = glutin::WindowBuilder::new()
        .with_title(title)
        .with_dimensions(w, h)
        .with_vsync();
    let (window, mut device, mut factory, mut main_color, mut main_depth) =
        gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder, &events_loop);
    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();
    let mut renderer = Renderer::init(&mut imgui, &mut factory, main_color.clone())
        .expect("Failed to initialize renderer");

    configure_keys(&mut imgui);

    let mut last_frame = Instant::now();
    let mut mouse_state = MouseState::default();

    const SIZE: f32 = 0.25;
    const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
    const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
    const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
    const PURPLE: [f32; 4] = [1.0, 0.0, 1.0, 1.0];
    const YELLOW: [f32; 4] = [1.0, 1.0, 0.0, 1.0];
    const BLUE_GREEN: [f32; 4] = [0.0, 1.0, 1.0, 1.0];

    let CUBE_COLORS: [[f32; 4]; 8] = [RED, GREEN, BLUE, PURPLE, YELLOW, BLUE_GREEN, RED, GREEN];
    let (cube_vertices, cube_indices) = make_cube((0.25, 0.25, 0.25), &CUBE_COLORS);

    let TRIANGLE_COLORS: [[f32; 4]; 3] = [BLUE, YELLOW, PURPLE];
    let triangle_vertices = make_triangle2d(0.35, &TRIANGLE_COLORS);

    loop {
        events_loop.poll_events(|glutin::Event::WindowEvent{event, ..}| {
            process_event!(event, imgui, window, renderer, mouse_state, game, main_color, main_depth);
        });

        let now = Instant::now();
        let delta = now - last_frame;
        let delta_s = delta.as_secs() as f32 + delta.subsec_nanos() as f32 / 1_000_000_000.0;
        last_frame = now;

        update_mouse(&mut imgui, &mut mouse_state);

        let size_points = window.get_inner_size_points().unwrap();
        let size_pixels = window.get_inner_size_pixels().unwrap();

        let ui = imgui.frame(size_points, size_pixels, delta_s);

        let set = factory.create_shader_set(
            include_bytes!("shader/triangle_150.glslv"),
            include_bytes!("shader/triangle_150.glslf")
        ).unwrap();


        // 1) Draw the UI.
        encoder.clear(&mut main_color, clear_color);
        render_ui(&ui, game);

        // 2) Draw our scene
        let pso = factory.create_pipeline_state(&set, gfx::Primitive::TriangleStrip, gfx::state::Rasterizer::new_fill(), pipe::new()).unwrap();
        let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&cube_vertices, cube_indices);
        let data = pipe::Data {
            vbuf: vertex_buffer,
            out: main_color.clone()
        };
        encoder.draw(&slice, &pso, &data);

        let pso = factory.create_pipeline_simple(
            include_bytes!("shader/triangle_150.glslv"),
            include_bytes!("shader/triangle_150.glslf"),
            pipe::new()
        ).unwrap();
        let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&triangle_vertices, ());
        let data = pipe::Data {
            vbuf: vertex_buffer,
            out: main_color.clone()
        };
        encoder.draw(&slice, &pso, &data);

        //encoder.draw()
        renderer.render(ui, &mut factory, &mut encoder).expect("Rendering failed");

        // 3) Flush our device and swap the buffers. Cleanup the device too?
        encoder.flush(&mut device);
        window.swap_buffers().unwrap();
        device.cleanup();

        if game.quit { break }
    };
}

fn configure_keys(imgui: &mut ImGui) {
    imgui.set_imgui_key(ImGuiKey::Tab, 0);
    imgui.set_imgui_key(ImGuiKey::LeftArrow, 1);
    imgui.set_imgui_key(ImGuiKey::RightArrow, 2);
    imgui.set_imgui_key(ImGuiKey::UpArrow, 3);
    imgui.set_imgui_key(ImGuiKey::DownArrow, 4);
    imgui.set_imgui_key(ImGuiKey::PageUp, 5);
    imgui.set_imgui_key(ImGuiKey::PageDown, 6);
    imgui.set_imgui_key(ImGuiKey::Home, 7);
    imgui.set_imgui_key(ImGuiKey::End, 8);
    imgui.set_imgui_key(ImGuiKey::Delete, 9);
    imgui.set_imgui_key(ImGuiKey::Backspace, 10);
    imgui.set_imgui_key(ImGuiKey::Enter, 11);
    imgui.set_imgui_key(ImGuiKey::Escape, 12);
    imgui.set_imgui_key(ImGuiKey::A, 13);
    imgui.set_imgui_key(ImGuiKey::C, 14);
    imgui.set_imgui_key(ImGuiKey::V, 15);
    imgui.set_imgui_key(ImGuiKey::X, 16);
    imgui.set_imgui_key(ImGuiKey::Y, 17);
    imgui.set_imgui_key(ImGuiKey::Z, 18);
}

fn update_mouse(imgui: &mut ImGui, mouse_state: &mut MouseState) {
    let scale = imgui.display_framebuffer_scale();
    imgui.set_mouse_pos(mouse_state.pos.0 as f32 / scale.0,
                        mouse_state.pos.1 as f32 / scale.1);
    imgui.set_mouse_down(&[mouse_state.pressed.0,
                              mouse_state.pressed.1,
                              mouse_state.pressed.2,
                              false,
                              false]);
    imgui.set_mouse_wheel(mouse_state.wheel / scale.1);
    mouse_state.wheel = 0.0;
}
