extern crate glium;
#[macro_use]
extern crate imgui;
extern crate imgui_glium_renderer;

use imgui::*;

use self::support::Support;

mod support;

const CLEAR_COLOR: (f32, f32, f32, f32) = (1.0, 1.0, 1.0, 1.0);

struct Game {
    config: GameConfig,
    state: State
}

#[derive(Clone)]
struct ChatMessage {
    msg: Vec<u8>
}

impl ChatMessage {
    pub fn new(bytes: Vec<u8>) -> ChatMessage {
        ChatMessage { msg: bytes }
    }
}

impl Iterator for ChatMessage {
    type Item = u8;
    fn next(&mut self) -> Option<u8> {
        match self.msg.iter().next() {
            Some(b) => Some(*b),
            None => None
        }
    }
}

#[derive(Clone)]
struct ChatHistory {
    history: Vec<ChatMessage>,
}

impl ChatHistory {
    pub fn new() -> ChatHistory {
        const GENERAL_CHAT_HISTORY: &'static [&'static str] = &["Wizz: Hey\0", "Thorny: Yo\0", "Mufk: SUp man\0",
            "Kazaghual: anyone w2b this axe I just found?\0",
            "PizzaMan: Yo I'm here to deliver this pizza, I'll just leave it over here by the dragon ok? NO FUCK YOU\0",
            "Moo:grass plz\0",
            "Aladin: STFU Jafar\0",
            "Rocky: JKSLFJS\0",
            "Diamond: In the sky...\0"];
        let hst_collection: Vec<ChatMessage> = GENERAL_CHAT_HISTORY.iter().rev().map(|x| { ChatMessage::new((*x).to_string().into_bytes()) }).collect();
        ChatHistory { history: hst_collection }
    }

    pub fn iter<'a>(&'a self) -> ChatHistoryIterator<'a> {
        ChatHistoryIterator::new(&self.history)
    }
}

struct ChatHistoryIterator<'a> {
    data: &'a Vec<ChatMessage>,
    pos: usize
}

impl<'a> ChatHistoryIterator<'a> {
    pub fn new(data: &'a Vec<ChatMessage>) -> ChatHistoryIterator<'a> {
        ChatHistoryIterator { data: data, pos: 0 }
    }
}

impl<'a> Iterator for ChatHistoryIterator<'a> {
    type Item = &'a ChatMessage;
    fn next(&mut self) -> Option<&'a ChatMessage> {
        let pos = self.pos;
        self.pos += 1;
        self.data.iter().nth(pos)
    }
}

struct GameConfig {
    window_dimensions: (u32, u32),
    chat_window_config: ChatWindowConfig
}

struct ChatWindowConfig {
    dimensions: (f32, f32),
    offset: (f32, f32),
    pos: (f32, f32),
    button_heights: f32
}

fn main() {
    let chat_config = ChatWindowConfig {
        dimensions: (480.0, 200.0),
        offset: (10.0, 6.0),
        button_heights: 20.0,
        pos: (0.0, 0.0)
        };
    let config = GameConfig {
        window_dimensions: (1024, 768),
        chat_window_config: chat_config
        };
    let state = State { chat_input_buffer: ImString::with_capacity(10), chat_history: ChatHistory::new()};
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

fn print_chat_msg<'a>(ui: &Ui<'a>, msg: &ChatMessage) {
    let mut msg_string = msg.msg.to_owned();
    msg_string.push(b'\0');
    unsafe {
        let msg_string: ImString = ImString::from_vec_unchecked(msg_string);
        ui.text_wrapped(&msg_string);
    }
}

fn print_chat_messages<'a>(ui: &Ui<'a>, history: &ChatHistory) {
    for msg in history.iter() {
        print_chat_msg(&ui, &msg);
    }
    ui.text_colored((0.0, 0.77, 0.46, 1.0), im_str!("Admin: Let there be color!"));
}

fn add_chat_button<'a>(text: &ImStr, ui: &Ui<'a>) {
    //let text_size = ui.calc_text_size(text, false, -1.0);

    //const BUTTON_PADDING: f32 = 0.0;
    const SPACING_BETWEEN_BUTTONS: f32 = 15.0;

    //let button_width = text_size.x + BUTTON_PADDING;
    ui.small_button(text);

    // setting the pos_x to 0.0 tells imgui to place the next item immediately after the last item,
    // allowing for spacing specified by the second parameter.
    let pos_x = 0.0;
    ui.same_line_spacing(pos_x, SPACING_BETWEEN_BUTTONS);
}

fn show_chat_window<'a>(ui: &Ui<'a>, config: &ChatWindowConfig, state: &mut State) {
    let (chat_w, chat_h) = config.dimensions;
    let (chat_x, chat_y) = config.pos;
    //let button_height = config.button_heights;

    ui.window(im_str!("ChatWindow"))
        .position((chat_x, chat_y), ImGuiSetCond_FirstUseEver)
        .size((chat_w, chat_h), ImGuiSetCond_FirstUseEver)
        .title_bar(false)
        .movable(true)
        .resizable(true)
        .save_settings(false)
        .no_bring_to_front_on_focus(true)
        .show_borders(false)
        .always_use_window_padding(false)
        .scrollable(false)
        .build(|| {
            add_chat_button(im_str!("General"), &ui);
            add_chat_button(im_str!("Combat Log"), &ui);
            add_chat_button(im_str!("Whisper"), &ui);
            add_chat_button(im_str!("Group"), &ui);
            add_chat_button(im_str!("Guild"), &ui);
            ui.new_line();

            print_chat_messages(&ui, &state.chat_history);
            ui.separator();
            ui.input_text(im_str!("enter text..."), &mut state.chat_input_buffer)
                .flags(ImGuiInputTextFlags_CharsHexadecimal)
                .auto_select_all(true)
                .build();
        });
        let mouse_pos = ui.imgui().mouse_pos();
        ui.text(im_str!("Mouse Position: ({:.1},{:.1})", mouse_pos.0, mouse_pos.1));
}

struct State {
    chat_input_buffer: ImString,
    chat_history: ChatHistory
}

fn run_game<'a>(ui: &Ui<'a>, game: &mut Game) {
    let config = &game.config;

    let (_, window_h) = config.window_dimensions;
    let window_h = window_h as f32;
    let (chat_w, chat_h) = config.chat_window_config.dimensions;

    let (offset_x, offset_y) = config.chat_window_config.offset;
    let (chat_x, chat_y) = (0.0 + offset_x, window_h - chat_h - offset_y);

    let chat_config = ChatWindowConfig { dimensions: (chat_w, chat_h), offset: (offset_x, offset_y),
        pos: (chat_x, chat_y), button_heights: config.chat_window_config.button_heights };

    show_chat_window(ui, &chat_config, &mut game.state)
}