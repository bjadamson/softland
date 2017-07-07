pub struct ChatWindowConfig {
    pub dimensions: (f32, f32),
    pub offset: (f32, f32),
    pub button_padding: f32,
    pub window_rounding: f32,
    pub max_length_input_text: usize,
    pub pos: (f32, f32),
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Channel(usize);

impl Channel {
    pub fn new(value: usize) -> Channel {
        Channel(value)
    }
}

#[derive(Clone)]
pub struct ChatMessage {
    msg: Vec<u8>,
    pub channel: Channel
}

impl ChatMessage {
    pub fn new(bytes: Vec<u8>, channel: Channel) -> ChatMessage {
        ChatMessage { msg: bytes, channel: channel }
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
        const CHAT_HISTORY_TEXT: &'static [(&'static str, Channel)] = &[
            ("Wizz: Hey\0", Channel(0)),
            ("Thorny: Yo\0", Channel(0)),
            ("Mufk: SUp man\0", Channel(0)),
            ("Kazaghual: anyone w2b this axe I just found?\0", Channel(2)),
            ("PizzaMan: Yo I'm here to deliver this pizza, I'll just leave it over here by the dragon ok? NO FUCK YOU\0", Channel(2)),
            ("Moo:grass plz\0", Channel(3)),
            ("Aladin: STFU Jafar\0", Channel(4)),
            ("Rocky: JKSLFJS\0", Channel(5)),

            ("You took 31 damage.\0", Channel(1)),
            ("You've given 25 damage.\0", Channel(1)),
            ("You took 61 damage.\0", Channel(1)),
            ("You've given 20 damage.\0", Channel(1)),
            ];

        let hst_collection: Vec<ChatMessage> = CHAT_HISTORY_TEXT.iter().rev().map(|&(msg, chan)| { ChatMessage::new((*msg).to_string().into_bytes(), chan) }).collect();
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
