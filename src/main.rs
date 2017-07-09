#[macro_use]
extern crate imgui;
extern crate itertools;

extern crate glium;
extern crate imgui_glium_renderer;

use imgui::*;
use itertools::Itertools;

use chat_history::{ChannelId, ChatHistory};
use self::support::Support;

mod chat_history;
mod support;

const CLEAR_COLOR: (f32, f32, f32, f32) = (0.2, 0.7, 0.8, 0.89);

pub struct ChatWindowState {
    pub dimensions: (f32, f32),
    pub offset: (f32, f32),
    pub button_padding: f32,
    pub window_rounding: f32,
    pub max_length_chat_input_text: usize,
    pub max_length_menu_input_text: usize,
    pub pos: (f32, f32),
    pub movable: bool,
    pub resizable: bool,
    pub save_settings: bool,
}

#[derive(Clone, Debug)]
enum EditingFieldOption {
    NotEditing,
    EditChatHistoryMaximumLength(i32),
    EditChannelName(ChannelId, String),
    EditChannelColorText(ChannelId),
}

struct State {
    chat_input_buffer: ImString,
    menu_input_buffer: ImString,
    chat_window_state: ChatWindowState,
    chat_history: ChatHistory,
    chat_button_pressed: ChannelId,
    editing_field: EditingFieldOption,
    window_dimensions: (u32, u32),
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
        save_settings: false
        };
    let chat_buffer_capacity = chat_config.max_length_chat_input_text;
    let menu_input_buffer_capacity = chat_config.max_length_menu_input_text;
    let chat_history_text = &[
        ("Welcome to the server 'Turnshroom Habitat'", ChannelId::new(0)),
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
        (String::from("Combat Log"), (0.7, 0.2, 0.1, 1.0)),
        (String::from("Whisper"), (0.8, 0.0, 0.7, 1.0)),
        (String::from("Group"), (0.2, 0.4, 0.9, 1.0)),
        (String::from("Guild"), (0.1, 0.8, 0.3, 1.0)),
    ];
    let mut state = State {
        chat_input_buffer: ImString::with_capacity(chat_buffer_capacity),
        menu_input_buffer: ImString::with_capacity(menu_input_buffer_capacity),
        chat_history: ChatHistory::from_existing(&init_channels, chat_history_text),
        chat_button_pressed: ChannelId::new(0),
        chat_window_state: chat_config,
        editing_field: EditingFieldOption::NotEditing,
        window_dimensions: (1024, 768),
        };

    let mut support = Support::init(state.window_dimensions);

    loop {
        support.render(CLEAR_COLOR, &mut state, run_game);
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
        if let Some(channel) = history.lookup_channel(msg.channel_id) {
            print_chat_msg(&ui, channel.text_color, msg.to_owned());
        }
    }
}

fn add_chat_button<'a>(text: &ImStr, button_color: (f32, f32, f32, f32), text_padding: (f32, f32), ui: &Ui<'a>) -> bool {
    let dont_wrap = -1.0;
    let text_size = ui.calc_text_size(text, false, dont_wrap);

    const COLOR_FACTOR: f32 = 4.0;
    let (r, g, b, a) = button_color;
    let button_color = (r, g, b, a / COLOR_FACTOR);

    let (padding_x, padding_y) = text_padding;
    let button_size = ImVec2::new(text_size.x + padding_x, text_size.y + padding_y);

    let mut pressed = false;
    ui.with_color_var(ImGuiCol::Button, button_color, || {
        pressed = ui.button(text, button_size);
    });

    // setting the POS_X to 0.0 tells imgui to place the next item immediately after the last item,
    // allowing for spacing specified by the second parameter.
    const POS_X: f32 = 0.0;
    const SPACING_BETWEEN_BUTTONS: f32 = 15.0;
    ui.same_line_spacing(POS_X, SPACING_BETWEEN_BUTTONS);

    pressed
}

fn create_rename_chat_channel_popup<'a>(ui: &Ui<'a>, id: ChannelId, channel_name: &str, state: &mut State) {    
    ui.popup(im_str!("Edit Channel Name"), || {
        if state.menu_input_buffer.is_empty() {
            state.menu_input_buffer.push_str(channel_name);
        }
        ui.input_text(im_str!("Enter a new channel name"), &mut state.menu_input_buffer)
            .auto_select_all(true)
            .chars_noblank(true)
            .chars_uppercase(true)
            .build();
        let button_size = (100.0, 20.0);
        let mut button_was_pressed = ui.button(im_str!("Cancel"), button_size);
        ui.same_line_spacing(0.0, 15.0);
        if ui.button(im_str!("Ok"), button_size) {
            button_was_pressed = true;
            let renamed = state.chat_history.rename_channel(id, &state.menu_input_buffer);
            if !renamed {
                panic!("error renaming channel!");
            }
        }

        if button_was_pressed {
            state.editing_field = EditingFieldOption::NotEditing;
            state.menu_input_buffer.clear();
            ui.close_current_popup();
        }
    });
}

fn create_set_channel_text_color_popup<'a>(ui: &Ui<'a>, id: ChannelId, state: &mut State) {
    ui.popup(im_str!("Edit Text Color"), || {
        state.chat_history.lookup_channel_mut(id).and_then(|mut channel| {
            let &mut (mut r, mut g, mut b, mut a) = &mut channel.text_color;
            ui.text_colored((0.4, 0.4, 0.4, 1.0), im_str!("Edit text color for channel: "));
            ui.same_line(0.0);

            let channel_name = unsafe { ImString::from_string_unchecked(channel.name.clone()) };

            let color = (r, g, b, a);
            ui.text_colored(color, &channel_name);
            ui.new_line();
            ui.new_line();

            const ALPHA: f32 = 0.7;

            ui.with_color_var(ImGuiCol::Text, (1.0, 0.0, 0.0, ALPHA), || {
                ui.input_float(im_str!("R"), &mut r)
                    .chars_decimal(true)
                    .enter_returns_true(true)
                    .auto_select_all(true)
                    .build();
            });
            ui.with_color_var(ImGuiCol::Text, (0.0, 1.0, 0.0, ALPHA), || {
                ui.input_float(im_str!("G"), &mut g)
                    .chars_decimal(true)
                    .enter_returns_true(true)
                    .auto_select_all(true)
                    .build();
            });
            ui.with_color_var(ImGuiCol::Text, (0.0, 0.0, 1.0, ALPHA), || {
                ui.input_float(im_str!("B"), &mut b)
                    .chars_decimal(true)
                    .enter_returns_true(true)
                    .auto_select_all(true)
                    .build();
            });
            ui.with_color_var(ImGuiCol::Text, (1.0, 1.0, 1.0, ALPHA), || {
                ui.input_float(im_str!("A"), &mut a)
                    .chars_decimal(true)
                    .enter_returns_true(true)
                    .auto_select_all(true)
                    .build();
            });

            Some(channel)
        });
        let button_size = (100.0, 20.0);
        let mut button_was_pressed = ui.button(im_str!("Cancel"), button_size);
        ui.same_line_spacing(0.0, 15.0);
        button_was_pressed |= ui.button(im_str!("Ok"), button_size);

        if button_was_pressed {
            state.editing_field = EditingFieldOption::NotEditing;
            ui.close_current_popup();
        }
    });
}

fn show_main_menu<'a>(ui: &Ui<'a>, state: &mut State) {
    ui.main_menu_bar(|| {
        ui.menu(im_str!("Menu")).build(|| {
            ui.menu_item(im_str!("Exit")).build();
        });
        ui.menu(im_str!("Options")).build(|| {
            ui.menu_item(im_str!("...")).build();
        });
        ui.menu(im_str!("Chat")).build(|| {
            ui.menu_item(im_str!("Maximum History")).build();
            if ui.menu_item(im_str!("Clear")).build() {
                state.chat_history.clear();
            }
            if ui.menu_item(im_str!("Restore")).build() {
                state.chat_history.restore();
            }
            ui.menu_item(im_str!("Movable")).selected(&mut state.chat_window_state.movable).build();
            ui.menu_item(im_str!("Resizable")).selected(&mut state.chat_window_state.resizable).build();
            ui.menu_item(im_str!("Save Settings")).selected(&mut state.chat_window_state.save_settings).build();
            for (idx, &(ref channel_name, _)) in state.chat_history.channel_names().iter().enumerate() {
                let cn = unsafe { ImString::from_string_unchecked(channel_name.clone()) };
                ui.menu(&cn).build(|| {
                    let channel_id = ChannelId::new(idx);
                    if ui.menu_item(im_str!("Name")).build() {
                        state.editing_field = EditingFieldOption::EditChannelName(channel_id, channel_name.to_owned());
                    }
                    if ui.menu_item(im_str!("Color")).build() {
                        state.editing_field = EditingFieldOption::EditChannelColorText(channel_id);
                    }
                    ui.menu_item(im_str!("Font")).build();
                });
            }
        });
    });

    match state.editing_field.clone() {
        EditingFieldOption::EditChannelName(id, name) => {
            create_rename_chat_channel_popup(&ui, id, &name, state);
            ui.open_popup(im_str!("Edit Channel Name"));
        },
        EditingFieldOption::EditChannelColorText(id) => {
            create_set_channel_text_color_popup(&ui, id, state);
            ui.open_popup(im_str!("Edit Text Color"));
        },
        EditingFieldOption::EditChatHistoryMaximumLength(length) => {},
        EditingFieldOption::NotEditing => {
            //ui.close_current_popup();
        }
    };
}

fn show_chat_window<'a>(ui: &Ui<'a>, state: &mut State) {
    let window_rounding = StyleVar::WindowRounding(state.chat_window_state.window_rounding);
    let (chat_w, chat_h) = state.chat_window_state.dimensions;
    let (chat_w, chat_h) = (chat_w as f32, chat_h as f32);
    let window_pos = state.chat_window_state.pos;
    //let button_height = state.button_padding;

    ui.with_style_var(window_rounding, || {
        ui.window(im_str!("ChatWindow"))
            .position(window_pos, ImGuiSetCond_FirstUseEver)
            .size((chat_w, chat_h), ImGuiSetCond_FirstUseEver)
            .title_bar(false)
            .movable(state.chat_window_state.movable)
            .resizable(state.chat_window_state.resizable)
            .save_settings(state.chat_window_state.save_settings)
            .inputs(true)  // interacting with buttons.
            .no_bring_to_front_on_focus(true)
            .show_borders(false)
            .always_use_window_padding(false)
            .scroll_bar(false)
            .scrollable(false)
            .build(|| {
                for (count, channels) in state.chat_history.channel_names().iter().enumerate() {
                    let &(ref name, color) = channels;
                    let id = ChannelId::new(count);

                    // 1) Add the channel to the chat_history
                    state.chat_history.add_channel(id, &name, color);

                    // 2) Draw the button for the chat channel.
                    let name = unsafe { ImString::from_string_unchecked(name.clone()) };
                    let pressed = add_chat_button(&name, color, (10.0, 7.0), &ui);
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

fn set_chat_window_pos<'a>(state: &mut State) {
    fn calculate_chat_window_position(window_dimensions: (u32, u32), config: &ChatWindowState) -> (f32, f32) {
        let (_, window_h) = window_dimensions;
        let window_h = window_h as f32;
        let (_, chat_h) = config.dimensions;

        let (offset_x, offset_y) = config.offset;
        let (chat_x, chat_y) = (0.0 + offset_x, window_h - chat_h - offset_y);
        (chat_x, chat_y)
    }
    let window_dimensions = state.window_dimensions;
    let chat_pos = {
        let chat_config = &state.chat_window_state;
        calculate_chat_window_position(window_dimensions, chat_config)
    };

    let chat_config = &mut state.chat_window_state;
    chat_config.pos = chat_pos;
}

fn run_game<'a>(ui: &Ui<'a>, state: &mut State) {
    show_main_menu(ui, state);

    set_chat_window_pos(state);
    show_chat_window(ui, state)
}
