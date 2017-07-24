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
extern crate imgui_sys;
extern crate imgui_gfx_renderer;

#[macro_use]
extern crate min_max_macros;

use imgui::*;
use itertools::Itertools;

#[macro_use]
extern crate serde_derive;

use camera::Camera;
use chat_history::{ChannelId, ChatHistory, ChatPrune};
use state::{ChatWindowState, EditingFieldOption, Player, State, UiBuffers};

use std::fs::File;
use std::io::prelude::*;

extern crate genmesh;
extern crate noise;
extern crate rand;

extern crate specs;
extern crate toml;

mod camera;
mod chat_history;
mod color;
mod gpu;
mod shader;
mod shape;
mod state;
mod support;
mod ui;

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
          ("Moo: grass plz", ChannelId::new(3)),
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
    let state = {
        let s = 0.22;
        let c = color::WHITE;
        let ambient_color = [c[0] * s, c[1] * s, c[2] * s, c[3]];

        let diffuse_color = [1.0, 0.0, 1.0, 1.0];
        let diffuse_color_pos = [0.0, 0.0, 0.0];

        State {
            ui_buffers: ui_buffers,
            chat_history: ChatHistory::from_existing(&init_channels, chat_history_text, prune),
            chat_button_pressed: ChannelId::new(0),
            chat_window_state: chat_config,
            edit_chat_field: EditingFieldOption::NotEditing,
            framerate: 0.0,
            window_dimensions: (1920, 1080),
            fullscreen: true,
            quit: false,

            player: Player {
                camera: Camera::from_rot([0.0, 0.0, 0.0]),
                move_speed: 0.2,
            },

            // level data
            ambient_color: ambient_color,

            diffuse_color: diffuse_color,
            diffuse_color_pos: diffuse_color_pos,
        }
    };

    let contents = {
        let mut file = File::open("data/foo.txt").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        contents
    };

    let clear_color: [f32; 4] = color::BLACK;
    support::run_game("Softland", clear_color, state, &contents, ui::render_ui);
}
