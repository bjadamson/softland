use chat_history::*;
use imgui::*;

#[derive(Copy, Clone, Debug)]
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
    pub view_all: bool,
}

#[derive(Debug)]
pub struct State {
    pub ui_buffers: UiBuffers,
    pub chat_window_state: ChatWindowState,
    pub chat_history: ChatHistory,
    pub chat_button_pressed: ChannelId,
    pub editing_field: EditingFieldOption,
    pub window_dimensions: (u32, u32),
    pub quit: bool,
}

#[derive(Clone, Debug)]
pub struct UiBuffers {
    pub chat_input_buffer: ImString,
    pub menu_input_buffer: ImString,
    pub menu_int_buffer: i32,
    pub menu_int_buffer_backup: i32,
    pub menu_bool_buffer: bool,
    pub menu_bool_buffer_backup: bool,
    pub menu_color_buffer: [f32; 4],
    pub menu_color_buffer_backup: [f32; 4],
}

#[derive(Clone, Debug)]
pub enum EditingFieldOption {
    NotEditing,
    ChatHistoryMaximumLength,
    ChannelName(ChannelId, String),
    ChannelColorText(ChannelId),
    ChatHistoryViewAll,
}
