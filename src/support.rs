use cgmath;
use cgmath::*;

use gfx;
use gfx::Device;
use gfx::traits::FactoryExt;

use gfx_window_glutin;
use glutin;
use glutin::{ElementState, MouseButton, MouseScrollDelta, VirtualKeyCode, TouchPhase, WindowEvent};
use image;
use imgui::{ImGui, Ui, ImGuiKey};
use imgui_gfx_renderer::Renderer;
use std::time::Instant;

use game_time::{GameClock, FrameCounter, FrameCount};
use game_time::framerate::RunningAverageSampler;
use game_time::step;

use genmesh::generators::*;
use genmesh::{Vertices, Triangulate};

use color;
use gpu;

use noise::{Perlin, NoiseModule, Seedable};
use rand;
use rand::*;
use specs::*;

use shape;
use shader;
use state;
use std::error::Error;
use state::*;
use toml;

struct TestSystem;

impl<'a> System<'a> for TestSystem {
    type SystemData = WriteStorage<'a, state::Model>;

    fn run(&mut self, mut model: Self::SystemData) {
        for model in (&mut model).join() {
            model.count += 5.0;
            let angle = -cgmath::Deg(model.count);
            let axis = Vector3::new(1.0, 1.0, 1.0).normalize();
            model.rotation = Quaternion::from_axis_angle(axis, angle);
            // model.rotation = Quaternion::from_angle_x(cgmath::Deg(model.count));
        }
    }
}

struct UpdateMouseStateSystem;

impl<'a> System<'a> for UpdateMouseStateSystem {
    type SystemData = FetchMut<'a, state::State>;

    fn run(&mut self, mut state: Self::SystemData) {
        // println!("updating 'previous' mouse state.");

        // for state in (&mut state).join() {
        // state.mouse.prev = state.mouse.current;
        // }
    }
}

fn process_event<'a, R>(event: &glutin::WindowEvent,
                        imgui: &mut ImGui,
                        window: &glutin::Window,
                        renderer: &mut Renderer<R>,
                        mouse: &mut state::MouseState,
                        game_state: &mut state::State,
                        main_color: &'a mut shader::OutColor<R>,
                        main_depth: &'a mut shader::OutDepth<R>)
    where R: gfx::Resources + 'a
{
    match event {
        &WindowEvent::Resized(_, _) => {
            // TODO: This used to work, when process_event() was a macro, before I turned it into
            // this fn. Should figure out we can't call update_views anymore. But for now I prefer
            // this as a function over the functionality being fixed. (not even sure what it did,
            // didn't look into it after gfx example code showed how it is to be used).
            // gfx_window_glutin::update_views(window, main_color, main_depth);
            renderer.update_render_target(main_color.clone());
        }
        &WindowEvent::Closed => game_state.quit = true,
        &WindowEvent::KeyboardInput(state, _, code, _) => {
            let pressed = state == ElementState::Pressed;
            let player = &mut game_state.player;
            let camera = &mut player.camera;

            if code == Some(VirtualKeyCode::Return) {
                if !pressed {
                    game_state.chat_window_state.user_editing ^= true;
                }
                imgui.set_key(11, pressed);
            }
            let editing = game_state.chat_window_state.user_editing;
            let editing = editing && (code != Some(VirtualKeyCode::Return));
            let editing = editing && (code != Some(VirtualKeyCode::Escape));

            macro_rules! guard {
                () => {
                    if editing {
                        return;
                    }
                }
            }
            match code {
                Some(VirtualKeyCode::Tab) => {
                    imgui.set_key(0, pressed);
                    guard!();
                }
                Some(VirtualKeyCode::Left) => {
                    imgui.set_key(1, pressed);

                    let x = game_state.diffuse_color_pos[0];
                    let y = game_state.diffuse_color_pos[1];
                    let z = game_state.diffuse_color_pos[2];
                    game_state.diffuse_color_pos = [x + 1.0, y, z];
                }
                Some(VirtualKeyCode::Right) => {
                    imgui.set_key(2, pressed);
                    guard!();
                    let x = game_state.diffuse_color_pos[0];
                    let y = game_state.diffuse_color_pos[1];
                    let z = game_state.diffuse_color_pos[2];
                    game_state.diffuse_color_pos = [x - 1.0, y, z];
                }
                Some(VirtualKeyCode::Up) => {
                    imgui.set_key(3, pressed);
                    guard!();
                    let x = game_state.diffuse_color_pos[0];
                    let y = game_state.diffuse_color_pos[1];
                    let z = game_state.diffuse_color_pos[2];
                    game_state.diffuse_color_pos = [x, y + 1.0, z];
                }
                Some(VirtualKeyCode::Down) => {
                    imgui.set_key(4, pressed);
                    guard!();
                    let x = game_state.diffuse_color_pos[0];
                    let y = game_state.diffuse_color_pos[1];
                    let z = game_state.diffuse_color_pos[2];
                    game_state.diffuse_color_pos = [x, y - 1.0, z];
                }
                Some(VirtualKeyCode::PageUp) => {
                    imgui.set_key(5, pressed);
                    guard!();
                }
                Some(VirtualKeyCode::PageDown) => {
                    imgui.set_key(6, pressed);
                    guard!();
                }
                Some(VirtualKeyCode::Home) => {
                    imgui.set_key(7, pressed);
                    guard!();
                }
                Some(VirtualKeyCode::End) => {
                    imgui.set_key(8, pressed);
                    guard!();
                }
                Some(VirtualKeyCode::Delete) => {
                    imgui.set_key(9, pressed);
                    guard!();
                }
                Some(VirtualKeyCode::Back) => {
                    imgui.set_key(10, pressed);
                    guard!();
                }
                Some(VirtualKeyCode::Escape) => {
                    imgui.set_key(12, pressed);
                    // If the user is currently editing text, then close the editing field
                    // without submission.
                    //
                    // Otherwise if the user is pushing down escape, we quit.
                    if game_state.chat_window_state.user_editing {
                        game_state.chat_window_state.user_editing ^= true;
                    } else if pressed {
                        game_state.quit = true;
                    }
                }
                Some(VirtualKeyCode::A) => {
                    imgui.set_key(13, pressed);
                    guard!();
                    camera.move_left(player.move_speed);
                }
                Some(VirtualKeyCode::C) => {
                    imgui.set_key(14, pressed);
                }
                Some(VirtualKeyCode::D) => {
                    guard!();
                    camera.move_right(player.move_speed);
                }
                Some(VirtualKeyCode::S) => {
                    guard!();
                    camera.move_backward(player.move_speed);
                }
                Some(VirtualKeyCode::V) => {
                    guard!();
                    imgui.set_key(15, pressed);
                }
                Some(VirtualKeyCode::W) => {
                    guard!();
                    camera.move_forward(player.move_speed);
                }
                Some(VirtualKeyCode::X) => {
                    imgui.set_key(16, pressed);
                }
                Some(VirtualKeyCode::Y) => {
                    imgui.set_key(17, pressed);
                    guard!();
                }
                Some(VirtualKeyCode::Z) => {
                    imgui.set_key(18, pressed);
                    guard!();
                }
                Some(VirtualKeyCode::LControl) |
                Some(VirtualKeyCode::RControl) => {
                    imgui.set_key_ctrl(pressed);
                    guard!();
                }
                Some(VirtualKeyCode::LShift) |
                Some(VirtualKeyCode::RShift) => {
                    imgui.set_key_shift(pressed);
                    guard!();
                }
                Some(VirtualKeyCode::LAlt) |
                Some(VirtualKeyCode::RAlt) => {
                    imgui.set_key_alt(pressed);
                    guard!();
                }
                Some(VirtualKeyCode::LWin) |
                Some(VirtualKeyCode::RWin) => {
                    imgui.set_key_super(pressed);
                    guard!();
                }
                _ => {}
            }
        }
        &WindowEvent::MouseMoved(x, y) => {
            // Don't process mouse movements if the user is typing.
            if game_state.chat_window_state.user_editing {
                return;
            }

            let (x, y) = (x as f32, y as f32);
            if mouse.cursor_pos.is_none() {
                let pos = (x, y);
                mouse.cursor_pos = Some(pos);
            } else {
                let pos = mouse.cursor_pos.unwrap();
                game_state.player.camera.rotate_to_mouse((x, y), pos, mouse.sensitivity);
            }
            mouse.cursor_pos = Some((x, y));
        }
        &WindowEvent::MouseInput(state, MouseButton::Left) => {
            mouse.pressed.0 = state == ElementState::Pressed
        }
        &WindowEvent::MouseInput(state, MouseButton::Right) => {
            mouse.pressed.1 = state == ElementState::Pressed
        }
        &WindowEvent::MouseInput(state, MouseButton::Middle) => {
            mouse.pressed.2 = state == ElementState::Pressed
        }
        &WindowEvent::MouseWheel(MouseScrollDelta::LineDelta(_, y), TouchPhase::Moved) => {
            mouse.wheel = y
        }
        &WindowEvent::MouseWheel(MouseScrollDelta::PixelDelta(_, y), TouchPhase::Moved) => {
            mouse.wheel = y
        }
        &WindowEvent::ReceivedCharacter(c) => imgui.add_input_character(c),
        _ => (),
    }
}

fn calculate_color(height: f32) -> [f32; 4] {
    if height > 0.9 {
        color::WHITE
    } else if height > 0.7 {
        color::GRAY
    } else if height > 0.5 {
        color::GREEN
    } else if height > 0.3 {
        color::BROWN
    } else {
        color::BLUE
    }
}

fn make_geometry(n: usize) -> (Vec<shader::ColorVertex>, Vec<u32>) {
    let seed = rand::thread_rng().gen();
    let plane = Plane::subdivide(16, 16);
    let perlin = Perlin::new().set_seed(seed);
    let vertexes: Vec<shader::ColorVertex> = plane.shared_vertex_iter()
        .take(n)
        .map(|v| {
            let pos = v.pos;
            let value = perlin.get(pos);

            let pos = [pos[0], pos[1], value, 1.0];
            shader::ColorVertex {
                pos: pos,
                color: calculate_color(value),
                normal: v.normal,
            }
        })
        .collect();

    let indices = plane.indexed_polygon_iter()
        .take(n)
        .triangulate()
        .vertices()
        .map(|i| i as u32)
        .collect();
    (vertexes, indices)
}

#[inline(always)]
fn copy_uv_vertices<'a, R, F, C, B>(factory: &mut F,
                                    encoder: &mut gfx::Encoder<R, C>,
                                    ambient: [f32; 4],
                                    light_color: [f32; 4],
                                    light_pos: [f32; 3],
                                    out_color: &'a shader::OutColor<R>,
                                    depth: &shader::OutDepth<R>,
                                    pso: &gfx::PipelineState<R, shader::UvPipe::Meta>,
                                    sampler: &gfx::handle::Sampler<R>,
                                    cube_texture_data: &shader::CubeTextureData<R>,
                                    model_m: Matrix4<f32>,
                                    viewpos: [f32; 3],
                                    vertices: &[shader::UvVertex],
                                    indices: B)
    where R: gfx::Resources + PartialEq<R>,
          F: gfx::Factory<R> + 'a,
          C: gfx::CommandBuffer<R> + 'a,
          B: gfx::IntoIndexBuffer<R> + 'a
{
    // println!("copy begin");
    let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&vertices, indices);
    let cbuf = factory.create_constant_buffer(1);
    let front_uvmap = (cube_texture_data.front.clone(), sampler.clone());
    let back_uvmap = (cube_texture_data.back.clone(), sampler.clone());
    let top_uvmap = (cube_texture_data.top.clone(), sampler.clone());
    let bottom_uvmap = (cube_texture_data.bottom.clone(), sampler.clone());
    let left_uvmap = (cube_texture_data.left.clone(), sampler.clone());
    let right_uvmap = (cube_texture_data.right.clone(), sampler.clone());
    let data = shader::UvPipe::Data {
        vbuf: vertex_buffer,
        locals: cbuf,
        uv_front: front_uvmap,
        uv_back: back_uvmap,
        uv_top: top_uvmap,
        uv_bottom: bottom_uvmap,
        uv_left: left_uvmap,
        uv_right: right_uvmap,
        model: model_m.into(),
        viewpos: viewpos.into(),
        ambient: ambient,
        lightcolor: light_color,
        lightpos: light_pos,
        out: out_color.clone(),
        depth: depth.clone(),
    };
    let locals = shader::Locals {
        model: data.model,
        viewpos: data.viewpos,
        ambient: data.ambient,
        lightcolor: data.lightcolor,
        lightpos: data.lightpos,
    };
    encoder.update_buffer(&data.locals, &[locals], 0).unwrap();
    encoder.draw(&slice, &pso, &data);
    // println!("copy end");
}

#[inline(always)]
fn copy_color_vertices<'a, R, F, C, B>(factory: &mut F,
                                       encoder: &mut gfx::Encoder<R, C>,
                                       ambient: [f32; 4],
                                       light_color: [f32; 4],
                                       light_pos: [f32; 3],
                                       out_color: &'a shader::OutColor<R>,
                                       depth: &shader::OutDepth<R>,
                                       pso: &gfx::PipelineState<R, shader::ColorPipe::Meta>,
                                       model_m: Matrix4<f32>,
                                       viewpos: [f32; 3],
                                       vertices: &[shader::ColorVertex],
                                       indices: B)
    where R: gfx::Resources + PartialEq<R>,
          F: gfx::Factory<R> + 'a,
          C: gfx::CommandBuffer<R> + 'a,
          B: gfx::IntoIndexBuffer<R> + 'a
{
    let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&vertices, indices);
    let cbuf = factory.create_constant_buffer(1);
    let data = shader::ColorPipe::Data {
        vbuf: vertex_buffer,
        locals: cbuf,
        model: model_m.into(),
        viewpos: viewpos.into(),
        ambient: ambient,
        lightcolor: light_color,
        lightpos: light_pos,
        out: out_color.clone(),
        depth: depth.clone(),
    };
    let locals = shader::Locals {
        model: data.model,
        viewpos: data.viewpos,
        ambient: data.ambient,
        lightcolor: data.lightcolor,
        lightpos: data.lightpos,
    };
    encoder.update_buffer(&data.locals, &[locals], 0).unwrap();
    encoder.draw(&slice, &pso, &data);
}

fn load_texture<R, F>(factory: &mut F,
                      data: &[u8])
                      -> Result<gfx::handle::ShaderResourceView<R, [f32; 4]>, Box<Error>>
    where R: gfx::Resources,
          F: gfx::Factory<R>
{
    use std::io::Cursor;
    use gfx::texture as t;
    let img = image::load(Cursor::new(data), image::PNG)?.to_rgba();
    let (width, height) = img.dimensions();
    let kind = t::Kind::D2(width as t::Size, height as t::Size, t::AaMode::Single);
    let (_, view) = factory.create_texture_immutable_u8::<shader::ColorFormat>(kind, &[&img])?;
    Ok(view)
}

pub fn run_game<F: FnMut(&Ui, &mut State)>(title: &str,
                                           clear_color: [f32; 4],
                                           mut state: State,
                                           file_contents: &str,
                                           mut build_ui: F)
                                           -> Result<(), Box<Error>> {
    let mut imgui = ImGui::init();

    let (w, h) = state.window_dimensions;
    let events_loop = glutin::EventsLoop::new();

    let monitor_id = glutin::get_available_monitors().nth(0).expect("Could not find a monitor.");
    let builder = glutin::WindowBuilder::new()
        .with_title(title)
        .with_dimensions(w, h)
        .with_vsync()
        .with_fullscreen(monitor_id);
    let (window, mut device, mut factory, mut main_color, mut main_depth) =
        gfx_window_glutin::init::<shader::ColorFormat, shader::DepthFormat>(builder, &events_loop);
    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();
    let mut renderer = Renderer::init(&mut imgui, &mut factory, main_color.clone())
        .expect("Failed to initialize renderer");

    configure_keys(&mut imgui);

    // println!("making ...");
    // let (plane_vertices, plane_indices) = make_geometry(90000);
    // println!("done!");

    let (cube_colors_pso, cube_uvs_pso) = {
        // let (/*triangle_pso, */cube_uv_pso, /*generated_pso*/) = {
        let mut pso_factory = gpu::PsoFactory::new(&mut factory);
        // let triangle_pso = pso_factory.triangle_list_uv();
        let cube_colors_pso = pso_factory.triangle_list_colors();
        let cube_uvs_pso = pso_factory.triangle_list_uv();
        // let generated_pso = pso_factory.triangle_list_uv();
        // (triangle_pso, cube_pso, generated_pso)
        (cube_colors_pso, cube_uvs_pso)

    };

    let mut world = World::new();
    world.register::<state::Model>();
    world.register::<State>();

    state.player.camera.move_forward(10.0);
    state.player.camera.look_at(&[0.0, 0.0, -1.0].into(), &[0.0, 1.0, 0.0].into());
    world.add_resource(state);

    #[derive(Debug, Deserialize, Serialize)]
    struct Rectangles {
        values: Vec<(f32, f32, f32)>,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct FooTxtFile {
        rectangles: Rectangles,
    }

    // println!("about to toml file contents");
    // let foo_content: FooTxtFile = toml::from_str(file_contents).unwrap();
    // println!("foo: {:?}", foo_content);

    // for &(x, y, z) in foo_content.rectangles.values.iter() {
    // let mut model = state::Model::new();
    // model.translation = Vector3::new(x, y, z);
    // world.create_entity().with(model).build();
    // }

    println!("pre load textures");
    macro_rules! load {
        ($filename:tt) => {{
            println!("loading file: {}", concat!("../assets/", $filename));
            load_texture(&mut factory, include_bytes!(concat!("../assets/", $filename)))?
        }}
    }
    let front = load!("cube_front.png");
    let back = load!("cube_back.png");
    let top = load!("cube_top.png");
    let bottom = load!("cube_bottom.png");
    let left = load!("cube_left.png");
    let right = load!("cube_right.png");

    eprintln!("pre sampler create");
    let sampler = factory.create_sampler_linear();
    println!("post load");

    let (xr, yr, zr) = (20, 20, 10);
    let num_divisions = 4;
    for x in 0..xr {
        for y in 0..yr {
            for z in 0..zr {
                for i in 0..num_divisions {
                    let (x, y, z, i, num_divisions) =
                        (x as f32, y as f32, z as f32, i as f32, num_divisions as f32);
                    let f = |v| v + (num_divisions / i);
                    let (x, y, z) = (f(x), f(y), f(z));
                    let mut model = state::Model::new();
                    model.translation = Vector3::new(x, y, z);
                    // model.translation += Vector3::new(0.5, 0.5, 0.5);
                    model.color = calculate_color(y / yr as f32);
                    // model.scale = Vector3::new(0.40, 0.40, 0.40);

                    // if i as i32 % 3 == 0 {
                    // model.color = color::YELLOW;
                    // }
                    world.create_entity().with(model).build();
                }
            }
        }
    }

    let mut dispatcher = DispatcherBuilder::new()
        .add(UpdateMouseStateSystem, "UpdateMouseStateSystem", &[])
        .add(TestSystem, "TestSystem", &["UpdateMouseStateSystem"])
        .build();

    let mut last_frame = Instant::now();
    let mut mouse = MouseState::default();

    let mut clock = GameClock::new();
    let mut counter = FrameCounter::new(60.0, RunningAverageSampler::with_max_samples(120));
    let mut sim_time;

    let mut rng = rand::thread_rng();

    loop {
        dispatcher.dispatch(&mut world.res);
        let mut state = &mut *world.write_resource::<State>();
        {
            sim_time = clock.tick(&step::FixedStep::new(&counter));
            counter.tick(&sim_time);
            state.framerate = sim_time.instantaneous_frame_rate();

            events_loop.poll_events(|glutin::Event::WindowEvent { event, .. }| {
                process_event(&event,
                              &mut imgui,
                              &window,
                              &mut renderer,
                              &mut mouse,
                              &mut state,
                              &mut main_color,
                              &mut main_depth);
            });
        }
        let now = Instant::now();
        let delta = now - last_frame;
        let delta_s = delta.as_secs() as f32 + delta.subsec_nanos() as f32 / 1_000_000_000.0;
        last_frame = now;

        update_mouse(&mut imgui, &mut mouse);

        // Draw our scene
        //
        // 1. Clear the background.
        encoder.clear(&mut main_color, clear_color);
        encoder.clear_depth(&mut main_depth, 1.0);

        // 2. Submit geometry to GPU.
        {
            // non-ui 2d stuffz
            let projection = {
                let (width, height) = state.window_dimensions;
                let aspect_ratio = width / height;
                let (near, far) = (0.1, 2000.0);
                let fovy = cgmath::Deg(75.0);
                cgmath::perspective(fovy, aspect_ratio as f32, near, far)
            };

            for model in world.read::<state::Model>().join() {
                let tmatrix = Matrix4::from_translation(model.translation);
                let rmatrix: Matrix4<f32> = model.rotation.into();
                let smatrix = {
                    let (x, y, z) = (model.scale / 4.0).into();
                    Matrix4::from_nonuniform_scale(x, y, z)
                };

                let mmatrix = tmatrix * rmatrix * smatrix;
                let view = state.player.camera.compute_view();
                let uv_matrix = projection * view * mmatrix;

                let c = model.color;
                let colors = [c, c, c, c, c, c];
                let viewpos = model.translation.into();
                let (uv_vertices, uv_indices) = shape::construct_uv_cube();
                let (color_vertices, color_indices) = shape::construct_color_cube(&colors);
                let cube_texture_data = shader::CubeTextureData {
                    front: front.clone(),
                    back: back.clone(),
                    top:top.clone(),
                    bottom: bottom.clone(),
                    left:left.clone(),
                    right:right.clone()
                };
                if rng.gen() {
                    copy_uv_vertices(&mut factory,
                                &mut encoder,
                                state.ambient_color,
                                state.diffuse_color,
                                state.diffuse_color_pos,
                                &main_color,
                                &main_depth,
                                &cube_uvs_pso,
                                &sampler,
                                &cube_texture_data,
                                uv_matrix,
                                viewpos,
                                &uv_vertices,
                                uv_indices);
                } else {
                    copy_color_vertices(&mut factory,
                                  &mut encoder,
                                  state.ambient_color,
                                  state.diffuse_color,
                                  state.diffuse_color_pos,
                                  &main_color,
                                  &main_depth,
                                  &cube_colors_pso,
                                  uv_matrix,
                                  viewpos,
                                  &color_vertices,
                                  color_indices);
                }
            }
            /*
            let tmatrix = Matrix4::from_translation(Vector3::new(0.0, 0.0, 0.0));
            let rmatrix: Matrix4<f32> = Quaternion::from_angle_x(cgmath::Deg(0.0)).into();
            let smatrix = {
                let sf = 1.0;
                Matrix4::from_translation(Vector3::new(sf, sf, sf))
            };
            let mmatrix = tmatrix * rmatrix * smatrix;

            let uv_matrix = projection * view * mmatrix;
            let viewpos = Vector3::new(0.0, 0.0, 0.0);
            copy_vertices(&mut factory,
                          &mut encoder,
                          state.ambient_color,
                          state.diffuse_color,
                          state.diffuse_color_pos,
                          &main_color,
                          &main_depth,
                          &generated_pso,
                          uv_matrix,
                          viewpos.into(),
                          &plane_vertices,
                          plane_indices.as_slice());
            */
        }

        // 3. Construct our UI.
        let size_points = window.get_inner_size_points().unwrap();
        let size_pixels = window.get_inner_size_pixels().unwrap();
        let ui = imgui.frame(size_points, size_pixels, delta_s);
        build_ui(&ui, &mut state);

        // 4. Draw our scene (both UI and geometry submitted via encoder).
        renderer.render(ui, &mut factory, &mut encoder).expect("Rendering failed");

        // 3) Flush our device and swap the buffers.
        encoder.flush(&mut device);
        window.swap_buffers().unwrap();
        device.cleanup();

        if state.quit {
            break;
        }
    }
    Ok(())
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

    if mouse_state.cursor_pos.is_none() {
        return;
    }
    let pos = mouse_state.cursor_pos.unwrap();
    imgui.set_mouse_pos(pos.0 as f32 / scale.0, pos.1 as f32 / scale.1);
    imgui.set_mouse_down(&[mouse_state.pressed.0,
                           mouse_state.pressed.1,
                           mouse_state.pressed.2,
                           false,
                           false]);
    imgui.set_mouse_wheel(mouse_state.wheel / scale.1);
    mouse_state.wheel = 0.0;
}
