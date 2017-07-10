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
        pos: [f32; 2] = "a_Pos",
        color: [f32; 3] = "a_Color",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        out: gfx::RenderTarget<ColorFormat> = "Target0",
    }
}

const TRIANGLE: [Vertex; 3] = [
    Vertex { pos: [ -0.25, -0.25 ], color: [1.0, 0.0, 0.0] },
    Vertex { pos: [  0.25, -0.25 ], color: [0.0, 1.0, 0.0] },
    Vertex { pos: [  0.0,  0.25 ], color: [0.0, 0.0, 1.0] }
];

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
                    Some(VirtualKeyCode::Return) => $imgui.set_key(11, pressed),
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

pub fn run<F: FnMut(&Ui, &mut State)>(title: &str, clear_color: [f32; 4], game: &mut State, mut run_ui: F) {
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

        let pso = factory.create_pipeline_simple(
            include_bytes!("shader/triangle_150.glslv"),
            include_bytes!("shader/triangle_150.glslf"),
            pipe::new()
        ).unwrap();
        let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&TRIANGLE, ());
        let data = pipe::Data {
            vbuf: vertex_buffer,
            out: main_color.clone()
        };

        run_ui(&ui, game);
        encoder.clear(&mut main_color, clear_color);
        encoder.draw(&slice, &pso, &data);
        renderer.render(ui, &mut factory, &mut encoder).expect("Rendering failed");
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