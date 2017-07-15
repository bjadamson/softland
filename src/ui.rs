use cgmath::*;
use imgui::*;

use color;
use chat_history::{ChannelId, ChatHistory};
use state::*;

pub fn render_ui<'a>(ui: &Ui<'a>, state: &mut State) {
    {
        let chat_history = &mut state.chat_history;
        let chat_window_state = &mut state.chat_window_state;
        let ui_buffers = &mut state.ui_buffers;
        let edit_field_option = &mut state.editing_field;
        let quit = &mut state.quit;
        let framerate = state.framerate;
        let position = state.player.camera.position();
        show_main_menu(ui,
                       quit,
                       framerate,
                       position,
                       chat_window_state,
                       edit_field_option,
                       chat_history,
                       ui_buffers);
    }
    set_chat_window_pos(state);
    show_chat_window(ui, state);

    let chat_history = &mut state.chat_history;
    let ui_buffers = &mut state.ui_buffers;
    let edit_field_option = &mut state.editing_field;
    match edit_field_option.clone() {
        EditingFieldOption::ChannelName(id, name) => {
            create_rename_chat_channel(&ui, id, &name, edit_field_option, chat_history, ui_buffers);
        }
        EditingFieldOption::ChannelColorText(id) => {
            create_set_channel_text_color(&ui, id, edit_field_option, chat_history, ui_buffers);
        }
        EditingFieldOption::ChatHistoryMaximumLength => {
            create_set_maximum_chat_history(&ui, edit_field_option, chat_history, ui_buffers);
        }
        EditingFieldOption::ChatHistoryViewAll => {
            create_view_all_chat_history(&ui, edit_field_option, chat_history);
        }
        EditingFieldOption::NotEditing => {}
    };
}

fn print_chat_msg<'a>(ui: &Ui<'a>, text_color: [f32; 4], msg_bytes: Vec<u8>) {
    let msg_string: ImString = unsafe { ImString::from_vec_unchecked(msg_bytes) };
    ui.with_color_var(ImGuiCol::Text, ImVec4::from(text_color), || {
        ui.text_wrapped(&msg_string);
    });
}

fn print_chat_messages<'a>(ui: &Ui<'a>, channel_id: ChannelId, history: &ChatHistory) {
    // If looking at channel 0, show all results.
    // Otherwise only yield results for the channel.
    for msg in history.iter_history()
        .filter(|&msg| channel_id == ChannelId::new(0) || msg.channel_id == channel_id) {
        if let Some(channel) = history.lookup_channel(msg.channel_id) {
            print_chat_msg(&ui, channel.text_color, msg.to_owned());
        }
    }
}

fn print_all_chat_message<'a>(ui: &Ui<'a>, history: &ChatHistory) {
    for msg in history.iter_backup() {
        if let Some(channel) = history.lookup_channel(msg.channel_id) {
            print_chat_msg(&ui, channel.text_color, msg.to_owned());
        }
    }
    for msg in history.iter_history() {
        if let Some(channel) = history.lookup_channel(msg.channel_id) {
            print_chat_msg(&ui, channel.text_color, msg.to_owned());
        }
    }
}

fn add_chat_button<'a>(text: &ImStr,
                       button_color: [f32; 4],
                       text_padding: (f32, f32),
                       ui: &Ui<'a>)
                       -> bool {
    let dont_wrap = -1.0;
    let text_size = ui.calc_text_size(text, false, dont_wrap);

    const COLOR_FACTOR: f32 = 4.0;
    let (r, g, b, a) = (button_color[0], button_color[1], button_color[2], button_color[3]);
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

fn create_rename_chat_channel<'a>(ui: &Ui<'a>,
                                  id: ChannelId,
                                  channel_name: &str,
                                  edit_field_option: &mut EditingFieldOption,
                                  chat_history: &mut ChatHistory,
                                  ui_buffers: &mut UiBuffers) {
    ui.window(im_str!("Rename Channel"))
        .position((100.0, 100.0), ImGuiSetCond_FirstUseEver)
        .title_bar(true)
        .movable(true)
        .resizable(false)
        .save_settings(false)
        .inputs(true)  // interacting with buttons.
        .collapsible(false)
        .scroll_bar(false)
        .always_auto_resize(true)
        .build(|| {
            if ui_buffers.menu_input_buffer.is_empty() {
                ui_buffers.menu_input_buffer.push_str(channel_name);
            }
            let text = "Rename channel: ".to_owned();
            let mut text = unsafe { ImString::from_string_unchecked(text) };
            ui.text(&text);

            ui.same_line(0.0);
            if let Some(channel) = chat_history.lookup_channel(id) {
                let text = channel_name.to_owned();
                let text = unsafe { ImString::from_string_unchecked(text) };
                ui.text_colored(channel.text_color, &text);
            };
            text.clear();
            ui.input_text(&text, &mut ui_buffers.menu_input_buffer)
                .auto_select_all(true)
                .chars_noblank(true)
                .chars_uppercase(true)
                .build();
            ui.new_line();

            let button_size = (100.0, 20.0);
            let mut button_was_pressed = ui.button(im_str!("Cancel"), button_size);
            ui.same_line(0.0);
            if ui.button(im_str!("Ok"), button_size) {
                button_was_pressed = true;
                let renamed = chat_history.rename_channel(id, &ui_buffers.menu_input_buffer);
                if !renamed {
                    panic!("error renaming channel!");
                }
            }

            if button_was_pressed {
                *edit_field_option = EditingFieldOption::NotEditing;
                ui_buffers.menu_input_buffer.clear();
            }
        });
}

fn create_set_channel_text_color<'a>(ui: &Ui<'a>,
                                     id: ChannelId,
                                     edit_field_option: &mut EditingFieldOption,
                                     chat_history: &mut ChatHistory,
                                     ui_buffers: &mut UiBuffers) {
    let (mut ok_pressed, mut cancel_pressed) = Default::default();
    ui.window(im_str!("Edit Channel Text Color"))
        .position((100.0, 100.0), ImGuiSetCond_FirstUseEver)
        .title_bar(true)
        .movable(true)
        .resizable(false)
        .save_settings(false)
        .inputs(true)  // interacting with buttons.
        .collapsible(false)
        .scroll_bar(false)
        .always_auto_resize(true)
        .build(|| {
            if let Some(channel) = chat_history.lookup_channel_mut(id) {
                let color = (0.4, 0.4, 0.4, 1.0);
                ui.text_colored(color, im_str!("Edit text color channel "));
                let channel_name = unsafe { ImString::from_string_unchecked(channel.name.clone()) };
                ui.text_colored(channel.text_color.clone(), &channel_name);
                ui.new_line();

                ui.color_edit4(im_str!(""), &mut ui_buffers.menu_color_buffer).build();

                let button_size = (100.0, 20.0);
                cancel_pressed = ui.button(im_str!("Cancel"), button_size);
                ui.same_line_spacing(0.0, 15.0);
                ok_pressed = ui.button(im_str!("Ok"), button_size);
            };
        });
    if let Some(mut channel) = chat_history.lookup_channel_mut(id) {
        if ok_pressed {
            channel.text_color = ui_buffers.menu_color_buffer;
            ui_buffers.menu_color_buffer_backup = ui_buffers.menu_color_buffer;
        } else if cancel_pressed {
            ui_buffers.menu_color_buffer = ui_buffers.menu_color_buffer_backup;
        }
        if cancel_pressed || ok_pressed {
            *edit_field_option = EditingFieldOption::NotEditing;
        }
    };
}

fn create_set_maximum_chat_history<'a>(ui: &Ui<'a>,
                                       edit_field_option: &mut EditingFieldOption,
                                       chat_history: &mut ChatHistory,
                                       ui_buffers: &mut UiBuffers) {
    ui.window(im_str!("History Length"))
        .position((100.0, 100.0), ImGuiSetCond_FirstUseEver)
        .title_bar(true)
        .movable(false)
        .resizable(false)
        .save_settings(false)
        .inputs(true)  // interacting with buttons.
        .collapsible(false)
        .scroll_bar(false)
        .build(|| {
            let color = (0.4, 0.4, 0.4, 1.0);
            ui.text_colored(color, im_str!("Enter maximum number of lines to display in your chat window."));
            ui.text_colored(color, im_str!("All further messages will be stored in memory (until you run out of physical memory)."));

            ui.checkbox(im_str!("Limit Chat History Length"), &mut ui_buffers.menu_bool_buffer);
            ui.with_color_var(ImGuiCol::Text, (0.0, 0.0, 1.0, 0.7), || {
                ui.input_int(im_str!(""), &mut ui_buffers.menu_int_buffer)
                    .chars_decimal(true)
                    .enter_returns_true(false)
                    .auto_select_all(true)
                    .build();
            });

            let button_size = (100.0, 20.0);
            let cancel_pressed = ui.button(im_str!("Cancel"), button_size);
            ui.same_line_spacing(0.0, 15.0);
            let ok_pressed = ui.button(im_str!("Ok"), button_size);
            if ok_pressed {
                // Copy value from our buffer.
                let prune_length = ui_buffers.menu_int_buffer;
                ui_buffers.menu_int_buffer_backup = prune_length;

                let prune_enabled = ui_buffers.menu_bool_buffer;
                ui_buffers.menu_bool_buffer_backup = prune_enabled;

                chat_history.set_prune(prune_enabled, prune_length);
                chat_history.restore();
                if prune_enabled {
                    chat_history.prune();
                }
            } else if cancel_pressed {
                // If they pressed cancel, undo our changes.
                ui_buffers.menu_int_buffer = ui_buffers.menu_int_buffer_backup;
                ui_buffers.menu_bool_buffer = ui_buffers.menu_bool_buffer_backup;
            }

            if cancel_pressed || ok_pressed {
                *edit_field_option = EditingFieldOption::NotEditing;
            }

            // restrict values to the positive domain of i32
            ui_buffers.menu_int_buffer = max!(0, ui_buffers.menu_int_buffer);
        });
}

fn create_view_all_chat_history<'a>(ui: &Ui<'a>,
                                    edit_field_option: &mut EditingFieldOption,
                                    chat_history: &mut ChatHistory) {
    ui.window(im_str!("Examine Chat"))
            .position((100.0, 100.0), ImGuiSetCond_FirstUseEver)
            .size((600.0, 400.0), ImGuiSetCond_FirstUseEver)
            .title_bar(true)
            .movable(true)
            .resizable(true)
            .save_settings(false)
            .inputs(true)  // interacting with buttons.
            .collapsible(false)
            .scroll_bar(false)
            .build(|| {
                ui.child_frame(im_str!(""), (0.0, -25.0))
                    .build(|| {
                        print_all_chat_message(&ui, chat_history);
                    });
                let button_size = (100.0, 20.0);
                if ui.button(im_str!("Done"), button_size) {
                    *edit_field_option = EditingFieldOption::NotEditing;
                }
            });
}

fn show_main_menu<'a>(ui: &Ui<'a>,
                      quit: &mut bool,
                      framerate: f64,
                      position: Vector3<f32>,
                      chat_window_state: &mut ChatWindowState,
                      edit_field_option: &mut EditingFieldOption,
                      chat_history: &mut ChatHistory,
                      ui_buffers: &mut UiBuffers) {
    ui.main_menu_bar(|| {
        ui.menu(im_str!("Menu")).build(|| {
            ui.menu_item(im_str!("Exit")).selected(quit).build();
        });
        ui.menu(im_str!("Options")).build(|| {
            ui.menu_item(im_str!("...")).build();
        });
        ui.menu(im_str!("Chat")).build(|| {
            if ui.menu_item(im_str!("View All")).build() {
                *edit_field_option = EditingFieldOption::ChatHistoryViewAll;
            }
            for (idx, &(ref channel_name, _)) in chat_history.channel_names().iter().enumerate() {
                let cn = unsafe { ImString::from_string_unchecked(channel_name.clone()) };
                ui.menu(&cn).build(|| {
                    let channel_id = ChannelId::new(idx);
                    if ui.menu_item(im_str!("Name")).build() {
                        *edit_field_option =
                            EditingFieldOption::ChannelName(channel_id, channel_name.to_owned());
                    }
                    if ui.menu_item(im_str!("Color")).build() {
                        if let Some(channel) =
                               chat_history.lookup_channel_mut(ChannelId::new(idx)) {
                            ui_buffers.menu_color_buffer = channel.text_color;
                            // Store the color currently in the buffer for later.
                            ui_buffers.menu_color_buffer_backup = ui_buffers.menu_color_buffer;
                            *edit_field_option = EditingFieldOption::ChannelColorText(channel_id);
                        };
                    }
                    ui.menu_item(im_str!("Font")).build();
                });
            }
            if ui.menu_item(im_str!("Max Length")).build() {
                let prune = chat_history.get_prune();
                ui_buffers.menu_int_buffer_backup = prune.length;
                ui_buffers.menu_bool_buffer_backup = prune.enabled;
                *edit_field_option = EditingFieldOption::ChatHistoryMaximumLength;
            }
            ui.menu_item(im_str!("Movable"))
                .selected(&mut chat_window_state.movable)
                .build();
            ui.menu_item(im_str!("Resizable"))
                .selected(&mut chat_window_state.resizable)
                .build();
            ui.menu_item(im_str!("Save Settings"))
                .selected(&mut chat_window_state.save_settings)
                .build();
        });
        let pos = format!("Position: [{}, {}, {}]", position.x, position.y, position.z);
        let pos = unsafe { ImString::from_string_unchecked(pos) };
        ui.with_color_var(ImGuiCol::TextDisabled, color::BLACK, || {
            ui.menu(&pos).enabled(false).build(|| {});
        });

        let fps = "Framerate: ".to_string() + &framerate.to_string();
        let fps = unsafe { ImString::from_string_unchecked(fps) };
        ui.with_color_var(ImGuiCol::TextDisabled, color::GREEN_YELLOW, || {
            ui.menu(&fps).enabled(false).build(|| {});
        });
    });
}

fn show_chat_window<'a>(ui: &Ui<'a>, state: &mut State) {
    let window_rounding = StyleVar::WindowRounding(state.chat_window_state.window_rounding);
    let (chat_w, chat_h) = state.chat_window_state.dimensions;
    let (chat_w, chat_h) = (chat_w as f32, chat_h as f32);
    let window_pos = state.chat_window_state.pos;
    // let button_height = state.button_padding;

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
                ui.child_frame(im_str!(""), ImVec2::new(-5.0, -20.0))
                    .always_resizable(false)
                    .input_allow(true) // interacting with internal scrollbar.
                    .scrollbar_horizontal(false)
                    .always_show_horizontal_scroll_bar(false)
                    .show_scrollbar(true)
                    .build(|| {
                        print_chat_messages(&ui, state.chat_button_pressed, &state.chat_history);
                    });
                if state.chat_window_state.user_editing {
                    let chat_entered_by_user = ui.input_text(im_str!(""), &mut state.ui_buffers.chat_input_buffer)
                        .auto_select_all(true)
                        .always_insert_mode(true)
                        .chars_noblank(true)
                        .enter_returns_true(true)
                        .build();
                    if chat_entered_by_user {
                        let prefix = b"You: ";
                        let mut msg = state.ui_buffers.chat_input_buffer.as_bytes().to_owned();
                        for (pos, byte) in prefix.iter().enumerate() {
                            msg.insert(pos, *byte);
                        }
                        state.chat_history.send_message_u8(state.chat_button_pressed, &msg);
                        state.ui_buffers.chat_input_buffer.clear();
                    }
                }
                //let mouse_pos = ui.imgui().mouse_pos();
                //ui.text(im_str!("Mouse Position: ({:.1},{:.1})", mouse_pos.0, mouse_pos.1));
            });
    });
}

fn set_chat_window_pos<'a>(state: &mut State) {
    fn calculate_chat_window_position(window_dimensions: (u32, u32),
                                      config: &ChatWindowState)
                                      -> (f32, f32) {
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
