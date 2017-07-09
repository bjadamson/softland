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
    fn channel_present(&self, id: ChannelId) -> bool {
        self.channels.iter().any(|ref x| {x.id == id})
    }

    pub fn new<'a>(history: &'a [(&'a str, ChannelId)]) -> ChatHistory {
        let hst_collection: Vec<ChatMessage> = history.iter().rev().map(|&(msg, chan_id)| { ChatMessage::new((*msg).to_string().into_bytes(), chan_id) }).collect();
        ChatHistory { history: hst_collection, channels: vec![] }
    }

    pub fn channel_names(&self) -> Vec<(String, (f32, f32, f32, f32))> {
        let copy_channel_name = |c: &Channel| {
            (c.name.clone(), c.text_color)
        };
        self.channels.iter().map(copy_channel_name).collect()
    }

    pub fn iter<'a>(&'a self) -> ChatHistoryIterator<'a> {
        ChatHistoryIterator::new(&self.history)
    }

    fn lookup_channel_mut(&mut self, id: ChannelId) -> Option<&mut Channel> {
        self.channels.iter_mut().filter(|x| {x.id == id}).next()
    }

    pub fn lookup_channel(&self, id: ChannelId) -> Option<&Channel> {
        self.channels.iter().filter(|x| {x.id == id}).next()
    }
    pub fn add_channel(&mut self, id: ChannelId, name: &str, text_color: (f32, f32, f32, f32)) -> bool {
        let channel_already_present = self.channel_present(id);
        if !channel_already_present {
            // We don't add the channel if it's already present.
            self.channels.push(Channel::new(id, name, text_color));
        }
        channel_already_present
    }

    pub fn rename_channel(&mut self, id: ChannelId, name: &str) -> bool {
        if !self.channel_present(id) {
            // can't rename a channel that is not present.
            return false;
        }
        let x = {
            let x = self.lookup_channel_mut(id).and_then(|f| {
                println!("renaming {} to {}", f.name, name);
                f.name = String::from(name);
                Some(f)
            });

            match x {
                Some(_) => true,
                None => false
            }
        };
        x
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
