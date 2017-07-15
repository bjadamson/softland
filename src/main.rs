extern crate cgmath;
use cgmath::*;

#[macro_use]
extern crate imgui;
extern crate itertools;

extern crate game_time;

#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate imgui_gfx_renderer;

#[macro_use]
extern crate min_max_macros;

use imgui::*;
use itertools::Itertools;

use camera::Camera;
use chat_history::{ChannelId, ChatHistory, ChatPrune};
use state::{ChatWindowState, EditingFieldOption, Player, State, UiBuffers};

extern crate genmesh;
extern crate noise;
extern crate rand;

extern crate specs;
use specs::*;

mod camera;
mod chat_history;
mod color;
mod gpu;
mod shader;
mod shape;
mod state;
mod support;
mod ui;

const CLEAR_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

struct MainSystem;

impl<'a> System<'a> for MainSystem {
    type SystemData = (FetchMut<'a, State>, ReadStorage<'a, state::Model>);

    fn run(&mut self, (mut state, model): Self::SystemData) {
        use specs::Join;

        // (state, model).join();
        for model in (&model).join() {
        }

        // for (&mut state, &model) in (&mut state, &model).join() {
        // }
        support::run_game("Softland", CLEAR_COLOR, &mut *state, ui::render_ui);
    }
}

struct TestSystem;

impl<'a> System<'a> for TestSystem {
    type SystemData = WriteStorage<'a, state::Model>;

    fn run(&mut self, mut model: Self::SystemData) {
        println!("TestSystem::run() fn here");
        for model in (&mut model).join() {
            model.rotation = Quaternion::from_angle_x(cgmath::Deg(10.0));
        }
    }
}

fn main() {
    let chat_config = ChatWindowState {
        dimensions: (480.0, 200.0),
        offset: (10.0, 6.0),
        button_padding: 20.0,
        window_rounding: 0.0,
        max_length_chat_input_text: 128,
        max_length_menu_input_text: 10,
        pos: (0.0, 0.0),
        movable: false,
        resizable: false,
        save_settings: false,
        view_all: false,
        user_editing: false,
    };
    let chat_buffer_capacity = chat_config.max_length_chat_input_text;
    let menu_input_buffer_capacity = chat_config.max_length_menu_input_text;
    let chat_history_text =
        &[("Welcome to the server 'Turnshroom Habitat'", ChannelId::new(0)),
          ("Wizz: Hey", ChannelId::new(0)),
          ("Thorny: Yo", ChannelId::new(0)),
          ("Mufk: SUp man", ChannelId::new(0)),
          ("Kazaghual: anyone w2b this axe I just found?", ChannelId::new(2)),
          ("PizzaMan: Yo I'm here to deliver this pizza, I'll just leave it over here by the \
            dragon ok?",
           ChannelId::new(2)),
          ("Moo:grass plz", ChannelId::new(3)),
          ("Aladin: STFU Jafar", ChannelId::new(4)),
          ("Rocky: JKSLFJS", ChannelId::new(5)),
          ("You took 31 damage.", ChannelId::new(1)),
          ("You've given 25 damage.", ChannelId::new(1)),
          ("You took 61 damage.", ChannelId::new(1)),
          ("You've given 20 damage.", ChannelId::new(1)),
          ("A gender chalks in the vintage coke. When will the murder pocket a wanted symptom? My \
            attitude observes any nuisance into the laughing constant.
        Every candidate \
            offers the railway under the beforehand molecule. The rescue buys his wrath \
            underneath the above garble.",
           ChannelId::new(4)),
          ("The truth collars the bass into a lower heel. A squashed machinery kisses the \
            abandon. Across its horse swims a sheep. Any umbrella damage rants over a sniff.
        \
            How can a theorem chalk the frustrating fraud? Should the world wash an \
            incomprehensible curriculum?",
           ChannelId::new(3)),
          ("The cap ducks inside the freedom. The mum hammers the apathy above our preserved \
            ozone. Will the peanut nose a review species? His vocabulary beams near the virgin.
        \
            The short supporter blames the hack fudge. The waffle exacts the bankrupt within an \
            infantile attitude.",
           ChannelId::new(1)),
          ("A flesh hazards the sneaking tooth. An analyst steams before an instinct! The muscle \
            expands within each brother! Why can't the indefinite garbage harden? The feasible \
            cider
        moans in the forest.",
           ChannelId::new(2)),
          ("Opposite the initiative scratches an inane plant. Why won't the late school \
            experiment with a crown? The sneak papers a go dinner without a straw. How can an \
            eating guy camp?
        Around the convinced verdict waffles a scratching shed. The \
            inhabitant escapes before whatever outcry.",
           ChannelId::new(1))];
    let init_channels = vec![
        (String::from("General"), [1.0, 1.0, 1.0, 1.0]),
        (String::from("Combat Log"), [0.7, 0.2, 0.1, 1.0]),
        (String::from("Whisper"), [0.8, 0.0, 0.7, 1.0]),
        (String::from("Group"), [0.2, 0.4, 0.9, 1.0]),
        (String::from("Guild"), [0.1, 0.8, 0.3, 1.0]),
    ];
    let prune = ChatPrune {
        length: 10,
        enabled: false,
    };
    let ui_buffers = UiBuffers {
        chat_input_buffer: ImString::with_capacity(chat_buffer_capacity),
        menu_input_buffer: ImString::with_capacity(menu_input_buffer_capacity),
        menu_int_buffer: Default::default(),
        menu_int_buffer_backup: Default::default(),
        menu_bool_buffer: Default::default(),
        menu_bool_buffer_backup: Default::default(),
        menu_color_buffer: Default::default(),
        menu_color_buffer_backup: Default::default(),
    };
    let mut state = State {
        ui_buffers: ui_buffers,
        chat_history: ChatHistory::from_existing(&init_channels, chat_history_text, prune),
        chat_button_pressed: ChannelId::new(0),
        chat_window_state: chat_config,
        editing_field: EditingFieldOption::NotEditing,
        framerate: 0.0,
        window_dimensions: (1024, 768),
        quit: false,

        player: Player {
            camera: Camera::new(),
            move_speed: 0.2,
        },
    };

    state.player.camera.move_backward(5.0);

    let mut world = World::new();
    world.register::<state::Model>();
    world.register::<State>();

    world.add_resource(state);
    world.create_entity().with(state::Model::new()).build();

    let mut dispatcher = DispatcherBuilder::new()
        .add(TestSystem, "TestSystem", &[])
        .add_thread_local(MainSystem)
        .build();
    dispatcher.dispatch(&mut world.res);
}
