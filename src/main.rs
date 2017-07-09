#[macro_use]
extern crate imgui;
extern crate itertools;

extern crate glium;
extern crate imgui_glium_renderer;

use imgui::*;
use itertools::Itertools;

use chat_history::{ChannelId, ChatHistory};
use chat_window::ChatWindowConfig;
use self::support::Support;

mod chat_history;
mod chat_window;
mod support;

const CLEAR_COLOR: (f32, f32, f32, f32) = (0.2, 0.7, 0.8, 0.89);

struct GameConfig {
    window_dimensions: (u32, u32),
    chat_window_config: ChatWindowConfig
}

enum EditingFieldOption {
    NotEditing,
    EditChatHistoryMaximumLength(i32),
    EditChannelName(String),
    EditChannelColorText((f32, f32, f32, f32)),
}

struct State {
    chat_input_buffer: ImString,
    menu_input_buffer: ImString,
    chat_history: ChatHistory,
    chat_button_pressed: ChannelId,
    editing_field: EditingFieldOption
}

struct Game {
    config: GameConfig,
    state: State
}

fn main() {
    let chat_config = ChatWindowConfig {
        dimensions: (480.0, 200.0),
        offset: (10.0, 6.0),
        button_padding: 20.0,
        window_rounding: 0.0,
        max_length_chat_input_text: 128,
        max_length_menu_input_text: 10,
        pos: (0.0, 0.0),
        };
    let chat_buffer_capacity = chat_config.max_length_chat_input_text;
    let menu_input_buffer_capacity = chat_config.max_length_menu_input_text;
    let config = GameConfig {
        window_dimensions: (1024, 768),
        chat_window_config: chat_config
        };
        let chat_history_text = &[
            ("Wizz: Hey", ChannelId::new(0)),
            ("Thorny: Yo", ChannelId::new(0)),
            ("Mufk: SUp man", ChannelId::new(0)),
            ("Kazaghual: anyone w2b this axe I just found?", ChannelId::new(2)),
            ("PizzaMan: Yo I'm here to deliver this pizza, I'll just leave it over here by the dragon ok?", ChannelId::new(2)),
            ("Moo:grass plz", ChannelId::new(3)),
            ("Aladin: STFU Jafar", ChannelId::new(4)),
            ("Rocky: JKSLFJS", ChannelId::new(5)),

            ("You took 31 damage.", ChannelId::new(1)),
            ("You've given 25 damage.", ChannelId::new(1)),
            ("You took 61 damage.", ChannelId::new(1)),
            ("You've given 20 damage.", ChannelId::new(1)),
        ];
    let init_channels = vec![
        (String::from("General"), (1.0, 1.0, 1.0, 1.0)),
        (String::from("Combat Log"), (1.0, 1.0, 1.0, 1.0)),
        (String::from("Whisper"), (0.8, 0.0, 0.7, 1.0)),
        (String::from("Group"), (0.2, 0.4, 0.9, 1.0)),
        (String::from("Guild"), (0.1, 0.8, 0.3, 1.0)),
    ];
    let state = State {
        chat_input_buffer: ImString::with_capacity(chat_buffer_capacity),
        menu_input_buffer: ImString::with_capacity(menu_input_buffer_capacity),
        chat_history: ChatHistory::from_existing(&init_channels, chat_history_text),
        chat_button_pressed: ChannelId::new(0),
        editing_field: EditingFieldOption::NotEditing
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
    let msg_string: ImString = unsafe { ImString::from_vec_unchecked(msg_bytes) };
    ui.with_color_var(ImGuiCol::Text, ImVec4::from(text_color), || {
        ui.text_wrapped(&msg_string);
    });
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

fn add_chat_button<'a>(text: &ImStr, text_padding: (f32, f32), ui: &Ui<'a>) -> bool {
    let dont_wrap = -1.0;
    let text_size = ui.calc_text_size(text, false, dont_wrap);

    let (padding_x, padding_y) = text_padding;
    let button_size = ImVec2::new(text_size.x + padding_x, text_size.y + padding_y);
    let pressed = ui.button(text, button_size);

    // setting the POS_X to 0.0 tells imgui to place the next item immediately after the last item,
    // allowing for spacing specified by the second parameter.
    const POS_X: f32 = 0.0;
    const SPACING_BETWEEN_BUTTONS: f32 = 15.0;
    ui.same_line_spacing(POS_X, SPACING_BETWEEN_BUTTONS);

    pressed
}

fn create_rename_chat_channel_popup<'a>(ui: &Ui<'a>, state: &mut State) {
    ui.popup(im_str!("Edit Channel Name"), || {
        ui.input_text(im_str!("Enter a new channel name"), &mut state.menu_input_buffer)
                    .flags(ImGuiInputTextFlags_CharsNoBlank)
                    .flags(ImGuiInputTextFlags_CharsUppercase)
                    .auto_select_all(true)
                    .build();
        let button_size = (100.0, 20.0);
        if ui.button(im_str!("Cancel"), button_size) {
            state.editing_field = EditingFieldOption::NotEditing;
            ui.close_current_popup();
        }
        ui.same_line_spacing(0.0, 15.0);
        if ui.button(im_str!("Ok"), button_size) {
            let renamed = state.chat_history.rename_channel(ChannelId::new(0), &state.menu_input_buffer);
            if !renamed {
                panic!("error renaming channel!");
            }
            state.editing_field = EditingFieldOption::NotEditing;
            ui.close_current_popup();
        }
    });
}

fn show_main_menu<'a>(ui: &Ui<'a>, state: &mut State) {
    // 1) Create the popups within imgui.
    create_rename_chat_channel_popup(&ui, state);

    // 2) Show the menu, and maybe a popup
    ui.main_menu_bar(|| {
        ui.menu(im_str!("Menu")).build(|| {
            ui.menu_item(im_str!("Exit")).build();
        });
        ui.menu(im_str!("Options")).build(|| {
            ui.menu_item(im_str!("...")).build();
        });
        ui.menu(im_str!("Chat")).build(|| {
            ui.menu_item(im_str!("Maximum History")).build();
            for &(ref channel_name, text_color) in state.chat_history.channel_names().iter() {
                let cn = unsafe { ImString::from_string_unchecked(channel_name.clone()) };
                ui.menu(&cn).build(|| {
                    if ui.menu_item(im_str!("Name")).build() {
                        state.editing_field = EditingFieldOption::EditChannelName(channel_name.clone());
                    }
                    if ui.menu_item(im_str!("Color")).build() {
                        state.editing_field = EditingFieldOption::EditChannelColorText(text_color);
                    }
                    ui.menu_item(im_str!("Font")).build();
                });
            }
        });
    });

    match state.editing_field {
        EditingFieldOption::EditChannelName(ref name) => {
            ui.open_popup(im_str!("Edit Channel Name"));
        },
        EditingFieldOption::EditChannelColorText(color) => {
            ui.open_popup(im_str!("Edit Text Color"));
        },
        EditingFieldOption::EditChatHistoryMaximumLength(length) => {},
        EditingFieldOption::NotEditing => {
            //ui.close_current_popup();
        }
    };
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
                for (count, channels) in state.chat_history.channel_names().iter().enumerate() {
                    let &(ref name, (r, g, b, a)) = channels;
                    let id = ChannelId::new(count);

                    // 1) Add the channel to the chat_history
                    state.chat_history.add_channel(id, &name, (r, g, b, a));

                    // 2) Draw the button for the chat channel.
                    let name = unsafe { ImString::from_string_unchecked(name.clone()) };
                    let pressed = add_chat_button(&name, (10.0, 7.0), &ui);
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

                let chat_entered_by_user = ui.input_text(im_str!(""), &mut state.chat_input_buffer)
                    .auto_select_all(true)
                    .always_insert_mode(true)
                    .chars_noblank(true)
                    .enter_returns_true(true)
                    .build();
                if chat_entered_by_user {
                    let prefix = b"You: ";
                    let mut msg = state.chat_input_buffer.as_bytes().to_owned();
                    for (pos, byte) in prefix.iter().enumerate() {
                        msg.insert(pos, *byte);
                    }
                    state.chat_history.send_message_u8(state.chat_button_pressed, &msg);
                    state.chat_input_buffer.clear();
                }
                //let mouse_pos = ui.imgui().mouse_pos();
                //ui.text(im_str!("Mouse Position: ({:.1},{:.1})", mouse_pos.0, mouse_pos.1));
            });
    });
}

fn set_chat_window_pos<'a>(game: &mut Game) {
    fn calculate_chat_window_position(window_dimensions: (u32, u32), config: &ChatWindowConfig) -> (f32, f32) {
        let (_, window_h) = window_dimensions;
        let window_h = window_h as f32;
        let (_, chat_h) = config.dimensions;

        let (offset_x, offset_y) = config.offset;
        let (chat_x, chat_y) = (0.0 + offset_x, window_h - chat_h - offset_y);
        (chat_x, chat_y)
    }
    let window_dimensions = game.config.window_dimensions;
    let chat_pos = {
        let chat_config = &game.config.chat_window_config;
        calculate_chat_window_position(window_dimensions, chat_config)
    };

    let chat_config = &mut game.config.chat_window_config;
    chat_config.pos = chat_pos;
}

fn run_game<'a>(ui: &Ui<'a>, game: &mut Game) {
    show_main_menu(ui, &mut game.state);

    set_chat_window_pos(game);
    let chat_config = &game.config.chat_window_config;
    show_chat_window(ui, &chat_config, &mut game.state)
}
