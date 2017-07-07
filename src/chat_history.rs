pub struct ChatWindowConfig {
    pub dimensions: (f32, f32),
    pub offset: (f32, f32),
    pub button_padding: f32,
    pub window_rounding: f32,
    pub max_length_input_text: usize,
    pub pos: (f32, f32),
}

#[derive(Clone)]
pub struct ChatMessage {
    msg: Vec<u8>
}

impl ChatMessage {
    pub fn new(bytes: Vec<u8>) -> ChatMessage {
        ChatMessage { msg: bytes }
    }

    pub fn to_owned(&self) -> Vec<u8> {
        self.msg.to_owned()
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
pub struct ChatHistory {
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

pub struct ChatHistoryIterator<'a> {
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
