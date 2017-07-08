#[macro_use]
extern crate imgui;
extern crate itertools;

extern crate glium;
extern crate imgui_glium_renderer;

use imgui::*;
use itertools::Itertools;

use chat_history::{ChannelId, ChatWindowConfig, ChatHistory};
use self::support::Support;

mod chat_history;
mod support;

const CLEAR_COLOR: (f32, f32, f32, f32) = (1.0, 1.0, 1.0, 1.0);

struct Game {
    config: GameConfig,
    state: State
}

struct GameConfig {
    window_dimensions: (u32, u32),
    chat_window_config: ChatWindowConfig
}

struct State {
    chat_input_buffer: ImString,
    chat_history: ChatHistory,
    chat_button_pressed: ChannelId
}

fn main() {
    let chat_config = ChatWindowConfig {
        dimensions: (480.0, 200.0),
        offset: (10.0, 6.0),
        button_padding: 20.0,
        window_rounding: 0.0,
        max_length_input_text: 128,
        pos: (0.0, 0.0),
        channels: ([
            ("General", (1.0, 1.0, 1.0, 1.0)),
            ("Combat Log", (1.0, 1.0, 1.0, 1.0)),
            ("Whisper", (0.8, 0.0, 0.7, 1.0)),
            ("Group", (0.2, 0.4, 0.9, 1.0)),
            ("Guild", (0.1, 0.8, 0.3, 1.0)),
            ])
        };
    let capacity = chat_config.max_length_input_text;
    let config = GameConfig {
        window_dimensions: (1024, 768),
        chat_window_config: chat_config
        };
    let state = State { chat_input_buffer: ImString::with_capacity(capacity), chat_history: ChatHistory::new(),
        chat_button_pressed: ChannelId::new(0)
        };
    let mut game = Game { config: config, state: state };
    let mut support = Support::init(game.config.window_dimensions);

    loop {
        support.render(CLEAR_COLOR, &mut game, run_game);
        let active = support.update_events();
        if !active {
            break;
        }
    }
}

fn print_chat_msg<'a>(ui: &Ui<'a>, text_color: (f32, f32, f32, f32), msg_bytes: Vec<u8>) {
    unsafe {
        let msg_string: ImString = ImString::from_vec_unchecked(msg_bytes);
        //ui.text_wrapped(&msg_string);
        ui.text_colored(text_color, &msg_string);
    }
}

fn print_chat_messages<'a>(channel_id: ChannelId, ui: &Ui<'a>, history: &ChatHistory) {
    // If looking at channel 0, show all results.
    // Otherwise only yield results for the channel.
    for msg in history.iter().filter(|&msg| { channel_id == ChannelId::new(0) || msg.channel_id == channel_id }) {
        let mut msg_string = msg.to_owned();
        msg_string.push(b'\0');

        if let Some(channel) = history.lookup_channel(msg.channel_id) {
            print_chat_msg(&ui, channel.text_color, msg_string);
        }
    }
    ui.text_colored((0.0, 0.77, 0.46, 1.0), im_str!("Admin: Let there be color!"));
}

fn add_chat_button<'a>(text: &ImStr, ui: &Ui<'a>) -> bool {
    let dont_wrap = -1.0;
    let text_size = ui.calc_text_size(text, false, dont_wrap);

    let button_padding = ImVec2::new(10.0, 7.0);
    let pressed = ui.button(text, text_size + button_padding);

    // setting the POS_X to 0.0 tells imgui to place the next item immediately after the last item,
    // allowing for spacing specified by the second parameter.
    const POS_X: f32 = 0.0;
    const SPACING_BETWEEN_BUTTONS: f32 = 15.0;
    ui.same_line_spacing(POS_X, SPACING_BETWEEN_BUTTONS);

    pressed
}

fn show_chat_window<'a>(ui: &Ui<'a>, config: &ChatWindowConfig, state: &mut State) {
    let (chat_w, chat_h) = config.dimensions;
    let (chat_x, chat_y) = config.pos;
    //let button_height = config.button_padding;

    ui.with_style_var(StyleVar::WindowRounding(config.window_rounding), || {
        ui.window(im_str!("ChatWindow"))
                .position((chat_x, chat_y), ImGuiSetCond_FirstUseEver)
                .size((chat_w, chat_h), ImGuiSetCond_FirstUseEver)
                .title_bar(false)
                .movable(true)
                .resizable(false)
                .save_settings(false)
                .inputs(true)  // interacting with buttons.
                .no_bring_to_front_on_focus(true)
                .show_borders(false)
                .always_use_window_padding(false)
                .scroll_bar(false)
                .scrollable(false)
                .build(|| {

                    for (count, &channels) in config.channels.iter().enumerate() {
                        let (name, (r, g, b, a)) = channels;
                        let id = ChannelId::new(count);

                        // 1) Add the channel to the chat_history
                        state.chat_history.add_channel(id, name, (r, g, b, a));

                        // 2) Convert the &'static str to a ImString
                        let s = ImString::new(String::from_utf8(name.as_bytes().to_owned()).unwrap()).unwrap();

                        // 3) Draw the button for the chat channel.
                        let pressed = add_chat_button(&s, &ui);
                        if pressed {
                            state.chat_button_pressed = id;
                        }
                    }

                    ui.new_line();
                    ui.child_frame(im_str!(""), ImVec2::new(chat_w - 10.0, chat_h - 58.0))
                        .always_resizable(false)
                        .input_allow(true) // interacting with internal scrollbar.
                        .scrollbar_horizontal(false)
                        .always_show_horizontal_scroll_bar(false)
                        .show_scrollbar(true)
                        .build(|| {
                            print_chat_messages(state.chat_button_pressed, &ui, &state.chat_history);
                        });

                    ui.input_text(im_str!("enter text..."), &mut state.chat_input_buffer)
                        .flags(ImGuiInputTextFlags_CharsHexadecimal)
                        .auto_select_all(true)
                        .build();
                    //let mouse_pos = ui.imgui().mouse_pos();
                    //ui.text(im_str!("Mouse Position: ({:.1},{:.1})", mouse_pos.0, mouse_pos.1));
                });
    });
}

fn run_game<'a>(ui: &Ui<'a>, game: &mut Game) {
    let config = &game.config.chat_window_config;

    let (_, window_h) = game.config.window_dimensions;
    let window_h = window_h as f32;
    let (chat_w, chat_h) = config.dimensions;

    let (offset_x, offset_y) = config.offset;
    let (chat_x, chat_y) = (0.0 + offset_x, window_h - chat_h - offset_y);
    let max_input_length = config.max_length_input_text;

    let chat_config = ChatWindowConfig { dimensions: (chat_w, chat_h), offset: (offset_x, offset_y),
        button_padding: config.button_padding, window_rounding: config.window_rounding,
        max_length_input_text: max_input_length, pos: (chat_x, chat_y),
        channels: config.channels
        };

    show_chat_window(ui, &chat_config, &mut game.state)
}
