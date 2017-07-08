use ImStr;

pub struct ChatWindowConfig {
    pub dimensions: (f32, f32),
    pub offset: (f32, f32),
    pub button_padding: f32,
    pub window_rounding: f32,
    pub max_length_input_text: usize,
    pub pos: (f32, f32),
    pub channels: [(&'static ImStr, (f32, f32, f32, f32)); 5]
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ChannelId(usize);

impl ChannelId {
    pub fn new(id: usize) -> ChannelId {
        ChannelId(id)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Channel {
    pub id: ChannelId,
    pub name: String,
    pub text_color: (f32, f32, f32, f32)
}

impl Channel {
    pub fn new(id: ChannelId, name: &str, text_color: (f32, f32, f32, f32)) -> Channel {
        Channel { id: id, name: name.to_owned(), text_color: text_color }
    }
}

#[derive(Clone)]
pub struct ChatMessage {
    pub msg: Vec<u8>,
    pub channel_id: ChannelId
}

impl ChatMessage {
    pub fn new(bytes: Vec<u8>, channel_id: ChannelId) -> ChatMessage {
        ChatMessage { msg: bytes, channel_id: channel_id }
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
    channels: Vec<Channel>
}

impl ChatHistory {
    pub fn new() -> ChatHistory {
        const CHAT_HISTORY_TEXT: &'static [(&'static str, ChannelId)] = &[
            ("Wizz: Hey\0", ChannelId(0)),
            ("Thorny: Yo\0", ChannelId(0)),
            ("Mufk: SUp man\0", ChannelId(0)),
            ("Kazaghual: anyone w2b this axe I just found?\0", ChannelId(2)),
            ("PizzaMan: Yo I'm here to deliver this pizza, I'll just leave it over here by the dragon ok? NO FUCK YOU\0", ChannelId(2)),
            ("Moo:grass plz\0", ChannelId(3)),
            ("Aladin: STFU Jafar\0", ChannelId(4)),
            ("Rocky: JKSLFJS\0", ChannelId(5)),

            ("You took 31 damage.\0", ChannelId(1)),
            ("You've given 25 damage.\0", ChannelId(1)),
            ("You took 61 damage.\0", ChannelId(1)),
            ("You've given 20 damage.\0", ChannelId(1)),
            ];

        let hst_collection: Vec<ChatMessage> = CHAT_HISTORY_TEXT.iter().rev().map(|&(msg, chan_id)| { ChatMessage::new((*msg).to_string().into_bytes(), chan_id) }).collect();
        ChatHistory { history: hst_collection, channels: vec![] }
    }

    pub fn iter<'a>(&'a self) -> ChatHistoryIterator<'a> {
        ChatHistoryIterator::new(&self.history)
    }

    pub fn lookup_channel(&self, id: ChannelId) -> Option<&Channel> {
        self.channels.iter().filter(|&x| {x.id == id}).next()
    }

    pub fn add_channel(&mut self, id: ChannelId, name: &str, text_color: (f32, f32, f32, f32)) -> bool {
        let channel_already_present = self.channels.iter().any(|ref x| {x.id == id});
        if !channel_already_present {
            // We don't add the channel if it's already present.
            self.channels.push(Channel::new(id, name, text_color));
        }
        channel_already_present
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
