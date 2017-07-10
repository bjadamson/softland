#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ChannelId(usize);

impl ChannelId {
    pub fn new(id: usize) -> ChannelId {
        ChannelId(id)
    }
}

#[derive(Debug, PartialEq)]
pub struct Channel {
    pub id: ChannelId,
    pub name: String,
    pub text_color: [f32; 4]
}

impl Channel {
    pub fn new(id: ChannelId, name: &str, text_color: [f32; 4]) -> Channel {
        Channel { id: id, name: name.to_owned(), text_color: text_color }
    }
}

pub struct ChatMessage {
    pub msg: Vec<u8>,
    pub channel_id: ChannelId
}

impl ChatMessage {
    pub fn new<B: Into<Vec<u8>>>(bytes: B, channel_id: ChannelId) -> ChatMessage {
        ChatMessage { msg: bytes.into(), channel_id: channel_id }
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

pub struct ChatPrune {
    pub length: i32,
    pub enabled: bool
}

pub struct ChatHistory {
    history: Vec<ChatMessage>,
    history_backup: Vec<ChatMessage>,
    channels: Vec<Channel>,
    prune: ChatPrune
}

impl ChatHistory {
    pub fn new<'a>() -> ChatHistory {
        ChatHistory { history: vec![], history_backup: vec![], channels: vec![], prune: ChatPrune { length: 0, enabled: false } }
    }

    pub fn from_existing<'a>(channels: &[((String), [f32; 4])], history: &'a [(&'a str, ChannelId)], prune: ChatPrune) -> ChatHistory {
        let mut chat_history = ChatHistory::new();
        chat_history.prune = prune;
        chat_history.history = history.iter().map(|&(msg, chan_id)| {ChatMessage::new((*msg).to_string().into_bytes(), chan_id) }).collect();

        for (idx, channels) in channels.iter().enumerate() {
            let &(ref name, color) = channels;
            let id = ChannelId::new(idx);
            chat_history.add_channel(id, &name, color);
        }
        chat_history.send_message_str(ChannelId::new(2), "dev: hi");
        chat_history
    }

    pub fn channel_names(&self) -> Vec<(String, [f32; 4])> {
        let copy_channel_name = |c: &Channel| {
            (c.name.clone(), c.text_color)
        };
        self.channels.iter().map(copy_channel_name).collect()
    }

    pub fn lookup_channel_mut(&mut self, id: ChannelId) -> Option<&mut Channel> {
        self.channels.iter_mut().filter(|x| {x.id == id}).next()
    }

    pub fn lookup_channel(&self, id: ChannelId) -> Option<&Channel> {
        self.channels.iter().filter(|x| {x.id == id}).next()
    }
    pub fn add_channel(&mut self, id: ChannelId, name: &str, text_color: [f32; 4]) -> bool {
        let channel_already_present = self.channel_present(id);
        if !channel_already_present {
            // We don't add the channel if it's already present.
            self.channels.push(Channel::new(id, name, text_color));
        }
        channel_already_present
    }

    pub fn clear(&mut self) {
        // Move everything from history into history_backup
        self.history_backup.append(&mut self.history);
    }

    pub fn restore(&mut self) {
        // 1) flush everything currently in recent history to the end of the history backup
        self.clear();

        // 2) Move everything from backup to recent history
        self.history.append(&mut self.history_backup);
    }

    fn channel_present(&self, id: ChannelId) -> bool {
        self.channels.iter().any(|ref x| {x.id == id})
    }

    pub fn rename_channel(&mut self, id: ChannelId, name: &str) -> bool {
        self.lookup_channel_mut(id).and_then(|f| {
            f.name = String::from(name);
            Some(f)
        }).is_some()
    }

    pub fn prune(&mut self) {
        let length = self.prune.length as usize;
        let history_length = self.history.len();
        if length < history_length {
            let extend_length = history_length - length;
            self.history_backup.extend(self.history.drain(..extend_length));
        }
    }

    pub fn get_prune(&self) -> &ChatPrune {
        &self.prune
    }

    pub fn set_prune(&mut self, enabled: bool, length: i32) {
        self.prune.enabled = enabled;
        self.prune.length = length;
    }

    pub fn send_message_u8(&mut self, id: ChannelId, msg: &[u8]) {
        let msg = ChatMessage::new(msg.to_owned(), id);
        self.history.push(msg);
        if self.prune.enabled {
            self.prune();
        }
    }

    pub fn send_message_str(&mut self, id: ChannelId, msg: &str) {
        self.send_message_u8(id, msg.as_bytes())
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
